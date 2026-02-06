//! Paragraph converter - handles paragraph elements and their structure.

use super::{ConversionContext, RunConverter};
use crate::Result;
use rs_docx::document::{Hyperlink, Paragraph, ParagraphContent};

/// Converter for Paragraph elements.
pub struct ParagraphConverter;

/// Segment of formatted text with consistent styling.
#[derive(Debug, Clone, PartialEq, Default)]
struct FormattedSegment {
    text: String,
    is_bold: bool,
    is_italic: bool,
    has_underline: bool,
    has_strike: bool,
    is_insertion: bool,
    is_deletion: bool,
    anchor: Option<String>,
}

impl ParagraphConverter {
    /// Converts a Paragraph to Markdown.
    pub fn convert(para: &Paragraph, context: &mut ConversionContext) -> Result<String> {
        // Collect all formatted segments from runs
        let segments = Self::collect_segments(para, context)?;

        // Merge adjacent segments with same formatting
        let merged = Self::merge_segments(segments);

        // Separate leading anchors (anchors at the start with empty text) from the rest
        let mut leading_anchors = Vec::new();
        let mut content_segments = Vec::new();
        let mut looking_for_anchors = true;

        for seg in merged {
            if looking_for_anchors && seg.text.is_empty() && seg.anchor.is_some() {
                if let Some(anchor) = &seg.anchor {
                    // Use id attribute instead of name for better compatibility (VS Code etc.)
                    leading_anchors.push(format!("<a id=\"{}\"></a>", anchor));
                }
            } else {
                looking_for_anchors = false;
                content_segments.push(seg);
            }
        }

        // Convert merged segments to markdown
        let text = Self::segments_to_markdown(&content_segments, context);

        let anchor_tags = leading_anchors.join("");

        if text.trim().is_empty() {
            // If there is no content but there are anchors, return just the anchors
            return Ok(anchor_tags);
        }

        // Apply paragraph-level formatting
        let formatted_text = Self::apply_paragraph_formatting(para, text, context)?;

        if !anchor_tags.is_empty() {
            // Place anchors on the line BEFORE the paragraph
            // This ensures scrolling lands above the header/list item
            // and maintains valid Markdown syntax for headers (e.g. ### Title)
            Ok(format!("{}\n{}", anchor_tags, formatted_text))
        } else {
            Ok(formatted_text)
        }
    }

    /// Collects formatted segments from paragraph content.
    fn collect_segments(
        para: &Paragraph,
        context: &mut ConversionContext,
    ) -> Result<Vec<FormattedSegment>> {
        let mut segments = Vec::new();
        // Field state: 0 = normal, 1 = after begin (skip instrText), 2 = after separate (visible TOC text)
        let mut field_state = 0;

        // Get paragraph style ID for inheritance
        let para_style_id = para
            .property
            .as_ref()
            .and_then(|p| p.style_id.as_ref())
            .map(|s| s.value.as_ref());

        for content in &para.content {
            match content {
                ParagraphContent::Run(run) => {
                    // Check for field characters (TOC, page refs, etc.)
                    for rc in &run.content {
                        if let rs_docx::document::RunContent::FieldChar(fc) = rc {
                            if let Some(char_type) = &fc.ty {
                                match char_type {
                                    rs_docx::document::CharType::Begin => field_state = 1,
                                    rs_docx::document::CharType::Separate => field_state = 2,
                                    rs_docx::document::CharType::End => field_state = 0,
                                }
                            }
                        }
                    }

                    // Skip field codes (instrText) in state 1
                    if field_state == 1 {
                        continue;
                    }

                    // In state 2 (after separate) or 0 (normal), extract visible text

                    let text = Self::extract_text(run, context);
                    if !text.is_empty() {
                        let segs = Self::run_to_segment(run, &text, context, para_style_id);
                        segments.extend(segs);
                    }
                }
                ParagraphContent::Link(hyperlink) => {
                    let link_md = Self::convert_hyperlink(hyperlink, context, para_style_id)?;
                    if !link_md.is_empty() {
                        // Hyperlinks are treated as plain text segments
                        segments.push(FormattedSegment {
                            text: link_md,
                            ..Default::default()
                        });
                    }
                }
                ParagraphContent::BookmarkStart(bookmark) => {
                    if let Some(name) = &bookmark.name {
                        segments.push(FormattedSegment {
                            anchor: Some(name.to_string()),
                            ..Default::default()
                        });
                    }
                }
                ParagraphContent::SDT(sdt) => {
                    // Structured document tags (TOC, etc.) - extract inner content
                    if let Some(sdt_content) = &sdt.content {
                        for bc in &sdt_content.content {
                            if let rs_docx::document::BodyContent::Paragraph(inner_para) = bc {
                                let inner_segs = Self::collect_segments(inner_para, context)?;
                                segments.extend(inner_segs);
                            }
                        }
                    }
                }
                ParagraphContent::Insertion(ins) => {
                    // Handle inserted content (track changes)
                    for run in &ins.runs {
                        let text = Self::extract_text(run, context);
                        if !text.is_empty() {
                            let mut segs = Self::run_to_segment(run, &text, context, para_style_id);
                            for seg in &mut segs {
                                seg.is_insertion = true;
                            }
                            segments.extend(segs);
                        }
                    }
                }
                ParagraphContent::Deletion(del) => {
                    // Handle deleted content (track changes)
                    let text = Self::extract_deleted_text(del);
                    if !text.is_empty() {
                        segments.push(FormattedSegment {
                            text,
                            is_deletion: true,
                            ..Default::default()
                        });
                    }
                }
                _ => {}
            }
        }

        Ok(segments)
    }

    /// Extracts deleted text from a Deletion element.
    fn extract_deleted_text(del: &rs_docx::document::Deletion) -> String {
        let mut text = String::new();
        for run in &del.runs {
            for content in &run.content {
                if let rs_docx::document::RunContent::DelText(del_text) = content {
                    text.push_str(&del_text.text);
                }
            }
        }
        text
    }

    /// Extracts text from a run, excluding field codes.
    fn extract_text(run: &rs_docx::document::Run, context: &mut ConversionContext) -> String {
        let mut text = String::new();
        for content in &run.content {
            match content {
                rs_docx::document::RunContent::Text(t) => {
                    text.push_str(&t.text);
                }
                rs_docx::document::RunContent::Tab(_) => {
                    text.push('\t');
                }
                rs_docx::document::RunContent::Break(br) => match br.ty {
                    Some(rs_docx::document::BreakType::Page) => text.push_str("\n\n---\n\n"),
                    _ => text.push('\n'),
                },
                rs_docx::document::RunContent::CarriageReturn(_) => {
                    text.push('\n');
                }
                rs_docx::document::RunContent::Drawing(drawing) => {
                    if let Ok(Some(img_md)) = context
                        .image_extractor
                        .extract_from_drawing(drawing, context.rels)
                    {
                        text.push_str(&img_md);
                    }
                }
                rs_docx::document::RunContent::Pict(pict) => {
                    if let Ok(Some(img_md)) = context
                        .image_extractor
                        .extract_from_pict(pict, context.rels)
                    {
                        text.push_str(&img_md);
                    }
                }
                // Skip InstrText (field codes like TOC, PAGEREF)
                rs_docx::document::RunContent::InstrText(_) => {}
                rs_docx::document::RunContent::DelInstrText(_) => {}
                rs_docx::document::RunContent::CommentReference(cref) => {
                    // Extract comment ID and look up comment text
                    if let Some(id) = &cref.id {
                        let id_str = id.to_string();
                        // Look up comment content
                        if let Some(comments) = context.docx_comments {
                            if let Some(comment) = comments
                                .comments
                                .iter()
                                .find(|c| c.id.map(|i| i.to_string()) == Some(id_str.clone()))
                            {
                                // Extract text from comment paragraph
                                let comment_text = comment.content.text();
                                context.comments.push((id_str.clone(), comment_text));
                            }
                        }
                        text.push_str(&format!("[^c{}]", id_str));
                    }
                }
                rs_docx::document::RunContent::FootnoteReference(fnref) => {
                    // Extract footnote ID and look up footnote text
                    if let Some(ref id_str) = fnref.id {
                        // Parse id string to isize for comparison
                        if let Ok(id_num) = id_str.parse::<isize>() {
                            // Look up footnote content
                            if let Some(footnotes) = context.docx_footnotes {
                                if let Some(footnote) =
                                    footnotes.content.iter().find(|f| f.id == Some(id_num))
                                {
                                    // Extract text from footnote body content
                                    let footnote_text: String = footnote
                                        .content
                                        .iter()
                                        .filter_map(|bc| match bc {
                                            rs_docx::document::BodyContent::Paragraph(p) => {
                                                Some(p.text())
                                            }
                                            _ => None,
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" ");
                                    context.footnotes.push(footnote_text);
                                }
                            }
                            let idx = context.footnotes.len();
                            text.push_str(&format!("[^{}]", idx));
                        }
                    }
                }
                rs_docx::document::RunContent::EndnoteReference(enref) => {
                    // Extract endnote ID and look up endnote text
                    if let Some(ref id_str) = enref.id {
                        // Parse id string to isize for comparison
                        if let Ok(id_num) = id_str.parse::<isize>() {
                            // Look up endnote content
                            if let Some(endnotes) = context.docx_endnotes {
                                if let Some(endnote) =
                                    endnotes.content.iter().find(|e| e.id == Some(id_num))
                                {
                                    // Extract text from endnote body content
                                    let endnote_text: String = endnote
                                        .content
                                        .iter()
                                        .filter_map(|bc| match bc {
                                            rs_docx::document::BodyContent::Paragraph(p) => {
                                                Some(p.text())
                                            }
                                            _ => None,
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" ");
                                    context.endnotes.push(endnote_text);
                                }
                            }
                            let idx = context.endnotes.len();
                            text.push_str(&format!("[^en{}]", idx));
                        }
                    }
                }
                _ => {}
            }
        }
        text
    }

    /// Creates formatted segments from a run, splitting on page breaks.
    fn run_to_segment(
        run: &rs_docx::document::Run,
        text: &str,
        context: &mut ConversionContext,
        para_style_id: Option<&str>,
    ) -> Vec<FormattedSegment> {
        // Resolve run style ID
        let mut run_style_id = None;
        if let Some(props) = &run.property {
            if let Some(style) = &props.style_id {
                run_style_id = Some(style.value.as_ref());
            }
        }

        // Resolve effective properties
        let props = context.style_resolver.resolve_run_property(
            run.property.as_ref(),
            run_style_id,
            para_style_id,
        );

        let is_bold = props
            .bold
            .as_ref()
            .map(|b| b.value.unwrap_or(true))
            .unwrap_or(false);
        let is_italic = props
            .italics
            .as_ref()
            .map(|i| i.value.unwrap_or(true))
            .unwrap_or(false);
        let has_underline = props.underline.is_some();
        let has_strike = props
            .strike
            .as_ref()
            .map(|s| s.value.unwrap_or(true))
            .unwrap_or(false);

        let delimiter = "\n\n---\n\n";
        let parts: Vec<&str> = text.split(delimiter).collect();
        let mut segments = Vec::new();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                // Add the break segment with no formatting
                segments.push(FormattedSegment {
                    text: delimiter.to_string(),
                    is_bold: false,
                    is_italic: false,
                    has_underline: false,
                    has_strike: false,
                    is_insertion: false,
                    is_deletion: false,
                    anchor: None,
                });
            }
            if !part.is_empty() {
                segments.push(FormattedSegment {
                    text: part.to_string(),
                    is_bold,
                    is_italic,
                    has_underline,
                    has_strike,
                    is_insertion: false,
                    is_deletion: false,
                    anchor: None,
                });
            }
        }

        segments
    }

    /// Merges adjacent segments with identical formatting.
    fn merge_segments(segments: Vec<FormattedSegment>) -> Vec<FormattedSegment> {
        let mut merged: Vec<FormattedSegment> = Vec::new();

        for seg in segments {
            if let Some(last) = merged.last_mut() {
                // Check if formatting matches (including track changes flags)
                if last.is_bold == seg.is_bold
                    && last.is_italic == seg.is_italic
                    && last.has_underline == seg.has_underline
                    && last.has_strike == seg.has_strike
                    && last.is_insertion == seg.is_insertion
                    && last.is_deletion == seg.is_deletion
                    && last.anchor == seg.anchor
                {
                    // Merge text
                    last.text.push_str(&seg.text);
                    continue;
                }
            }
            merged.push(seg);
        }

        merged
    }

    /// Applies markdown formatting markers safely, handling edge cases.
    ///
    /// Handles:
    /// - Empty or whitespace-only text (skips formatting)
    /// - Text with newlines (applies formatting per line)
    /// - Leading/trailing whitespace (preserves outside markers)
    fn apply_format_safely(text: &str, open: &str, close: &str) -> String {
        // Skip if text is empty or whitespace-only
        if text.trim().is_empty() {
            return text.to_string();
        }

        // Handle leading/trailing whitespace - preserve it outside the markers
        let leading_ws: String = text
            .chars()
            .take_while(|c| c.is_whitespace() && *c != '\n')
            .collect();
        let trailing_ws: String = text
            .chars()
            .rev()
            .take_while(|c| c.is_whitespace() && *c != '\n')
            .collect::<String>()
            .chars()
            .rev()
            .collect();

        let content_start = leading_ws.len();
        let content_end = text.len() - trailing_ws.len();
        let content = &text[content_start..content_end];

        // If content contains newlines, apply formatting to each non-empty line
        if content.contains('\n') {
            let formatted: Vec<String> = content
                .split('\n')
                .map(|line| {
                    let line_trimmed = line.trim();
                    if line_trimmed.is_empty() {
                        line.to_string()
                    } else {
                        // Preserve line's own leading/trailing whitespace
                        let line_leading: String =
                            line.chars().take_while(|c| c.is_whitespace()).collect();
                        let line_trailing: String = line
                            .chars()
                            .rev()
                            .take_while(|c| c.is_whitespace())
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect();
                        format!(
                            "{}{}{}{}{}",
                            line_leading, open, line_trimmed, close, line_trailing
                        )
                    }
                })
                .collect();
            return format!("{}{}{}", leading_ws, formatted.join("\n"), trailing_ws);
        }

        // Normal case: wrap content with markers, preserve outer whitespace
        format!(
            "{}{}{}{}{}",
            leading_ws,
            open,
            content.trim(),
            close,
            trailing_ws
        )
    }

    /// Converts segments to markdown text.
    fn segments_to_markdown(segments: &[FormattedSegment], context: &ConversionContext) -> String {
        let mut result = String::new();

        for seg in segments {
            // Render anchor if present
            if let Some(anchor) = &seg.anchor {
                result.push_str(&format!("<a id=\"{}\"></a>", anchor));
            }

            let mut text = seg.text.clone();

            // Apply track changes formatting first
            if seg.is_deletion {
                // Deleted text: strikethrough
                text = Self::apply_format_safely(&text, "~~", "~~");
            }
            if seg.is_insertion {
                // Inserted text: HTML ins tag or underline
                text = format!("<ins>{}</ins>", text);
            }

            // Apply regular formatting
            if seg.has_underline && context.options.html_underline && !seg.is_insertion {
                text = format!("<u>{}</u>", text);
            }

            if seg.has_strike && !seg.is_deletion {
                if context.options.html_strikethrough {
                    text = format!("<s>{}</s>", text);
                } else {
                    text = Self::apply_format_safely(&text, "~~", "~~");
                }
            }

            if seg.is_bold && seg.is_italic {
                text = format!("<strong><em>{}</em></strong>", text);
            } else if seg.is_bold {
                text = format!("<strong>{}</strong>", text);
            } else if seg.is_italic {
                text = format!("<em>{}</em>", text);
            }

            result.push_str(&text);
        }

        result
    }

    /// Applies paragraph-level formatting (heading, list, alignment).
    fn apply_paragraph_formatting(
        para: &Paragraph,
        text: String,
        context: &mut ConversionContext,
    ) -> Result<String> {
        let para_style_id = para
            .property
            .as_ref()
            .and_then(|p| p.style_id.as_ref())
            .map(|s| s.value.as_ref());

        // Resolve effective paragraph properties
        let effective_props = context
            .style_resolver
            .resolve_paragraph_property(para.property.as_ref(), para_style_id);

        let mut prefix = String::new();
        let mut is_heading = false;

        // Check for heading via pStyle
        if let Some(style) = &effective_props.style_id {
            if let Some(heading_level) = context.localization.parse_heading_style(&style.value) {
                // Don't generate heading for empty text
                if text.trim().is_empty() {
                    return Ok(String::new());
                }
                prefix.push_str(&"#".repeat(heading_level));
                prefix.push(' ');
                is_heading = true;
            }
        }

        // Check for numbering (list items)
        if let Some(num_pr) = &effective_props.numbering {
            if let (Some(num_id), Some(ilvl)) = (&num_pr.id, &num_pr.level) {
                let num_id_val = num_id.value as i32;
                let ilvl_val = ilvl.value as i32;
                let mut marker = context.numbering.next_marker(num_id_val, ilvl_val);

                // Handle localization-specific numbering (e.g., Korean "제N조" -> Heading)
                if let Some(formatted_prefix) = context.localization.handle_numbering(&marker) {
                    if !is_heading {
                        prefix = formatted_prefix;
                        prefix.push(' ');
                        is_heading = true;
                        marker = String::new(); // Clear marker if it was consumed/replaced by prefix
                    }
                }

                if is_heading {
                    prefix.push_str(&marker);
                    if !marker.is_empty() {
                        prefix.push(' ');
                    }
                } else {
                    let indent = context.numbering.get_indent(num_id_val, ilvl_val);
                    let effective_indent = indent.min(1);
                    let indent_str = "  ".repeat(effective_indent);
                    prefix.push_str(&indent_str);
                    prefix.push_str(&marker);
                    prefix.push(' ');
                }
            }
        }

        let final_text = format!("{}{}", prefix, text.trim());

        // Check for text alignment (only if not heading)
        if !is_heading {
            if let Some(jc) = &effective_props.justification {
                match &jc.value {
                    rs_docx::formatting::JustificationVal::Center => {
                        return Ok(format!(
                            "<div style=\"text-align: center;\">{}</div>",
                            final_text
                        ));
                    }
                    rs_docx::formatting::JustificationVal::Right => {
                        return Ok(format!(
                            "<div style=\"text-align: right;\">{}</div>",
                            final_text
                        ));
                    }
                    _ => {}
                }
            }
        }

        Ok(final_text)
    }

    /// Converts a hyperlink to Markdown format.
    fn convert_hyperlink(
        hyperlink: &Hyperlink,
        context: &mut ConversionContext,
        para_style_id: Option<&str>,
    ) -> Result<String> {
        let mut link_text = String::new();
        let mut field_state = 0; // 0=normal, 1=instrText, 2=visible

        for run in &hyperlink.content {
            // Check field char
            for rc in &run.content {
                if let rs_docx::document::RunContent::FieldChar(fc) = rc {
                    if let Some(char_type) = &fc.ty {
                        match char_type {
                            rs_docx::document::CharType::Begin => field_state = 1,
                            rs_docx::document::CharType::Separate => field_state = 2,
                            rs_docx::document::CharType::End => field_state = 0,
                        }
                    }
                }
            }

            if field_state == 1 {
                continue;
            }

            let text = RunConverter::convert(run, context, para_style_id)?;
            link_text.push_str(&text);
        }

        // Get target URL from relationship or anchor
        let url = if let Some(anchor) = &hyperlink.anchor {
            // Internal bookmark link (used in TOC entries)
            format!("#{}", anchor)
        } else if let Some(id) = &hyperlink.id {
            // External link via relationship
            context
                .rels
                .get(&id.to_string())
                .cloned()
                .unwrap_or_else(|| "#".to_string())
        } else {
            "#".to_string()
        };

        if link_text.is_empty() {
            Ok(url)
        } else {
            Ok(format!("[{}]({})", link_text, url))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rs_docx::document::{Hyperlink, ParagraphContent, Run, RunContent, Text};
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn test_toc_anchor_link() {
        // Create a paragraph with a hyperlink having an anchor
        let mut para = Paragraph::default();

        let mut hyperlink = Hyperlink::default();
        hyperlink.anchor = Some(Cow::Borrowed("_Toc123456789"));

        let mut run = Run::default();
        run.content.push(RunContent::Text(Text {
            text: "Introduction".into(),
            ..Default::default()
        }));

        hyperlink.content.push(run);

        para.content.push(ParagraphContent::Link(hyperlink));

        // Setup minimal context
        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut numbering_resolver = super::super::NumberingResolver::new(&docx);
        let mut image_extractor = super::super::ImageExtractor::new_skip();
        let options = crate::ConvertOptions::default();
        let style_resolver = super::super::StyleResolver::new(&docx.styles);
        let localization = crate::localization::KoreanLocalization;

        let mut context = super::ConversionContext {
            rels: &rels,
            numbering: &mut numbering_resolver,
            image_extractor: &mut image_extractor,
            options: &options,
            footnotes: Vec::new(),
            endnotes: Vec::new(),
            comments: Vec::new(),
            docx_comments: None,
            docx_footnotes: None,
            docx_endnotes: None,
            styles: &docx.styles,
            style_resolver: &style_resolver,
            localization: &localization,
        };

        // Convert
        let md = ParagraphConverter::convert(&para, &mut context).expect("Conversion failed");

        // Verify
        assert_eq!(md, "[Introduction](#_Toc123456789)");
    }

    #[test]
    fn test_toc_anchor_target() {
        use rs_docx::document::BookmarkStart;

        // Create a paragraph with a bookmark start (anchor target)
        let mut para = Paragraph::default();

        let mut bookmark = BookmarkStart::default();
        bookmark.name = Some(Cow::Borrowed("_Toc123456789"));

        let mut run = Run::default();
        run.content.push(RunContent::Text(Text {
            text: "Chapter 1".into(),
            ..Default::default()
        }));

        para.content.push(ParagraphContent::BookmarkStart(bookmark));
        para.content.push(ParagraphContent::Run(run));

        // Setup minimal context
        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut numbering_resolver = super::super::NumberingResolver::new(&docx);
        let mut image_extractor = super::super::ImageExtractor::new_skip();
        let options = crate::ConvertOptions::default();
        let style_resolver = super::super::StyleResolver::new(&docx.styles);
        let localization = crate::localization::KoreanLocalization;

        let mut context = super::ConversionContext {
            rels: &rels,
            numbering: &mut numbering_resolver,
            image_extractor: &mut image_extractor,
            options: &options,
            footnotes: Vec::new(),
            endnotes: Vec::new(),
            comments: Vec::new(),
            docx_comments: None,
            docx_footnotes: None,
            docx_endnotes: None,
            styles: &docx.styles,
            style_resolver: &style_resolver,
            localization: &localization,
        };

        // Convert
        let md = ParagraphConverter::convert(&para, &mut context).expect("Conversion failed");

        // Verify that the anchor tag is generated BEFORE the text (on new line)
        assert_eq!(md, "<a id=\"_Toc123456789\"></a>\nChapter 1");
    }

    #[test]
    fn test_anchor_placement_header() {
        use rs_docx::document::BookmarkStart;

        // Create a paragraph with Heading 1 style and a bookmark
        let mut para = Paragraph::default();
        let mut props = rs_docx::formatting::ParagraphProperty::default();
        props.style_id = Some(rs_docx::formatting::ParagraphStyleId {
            value: "Heading1".into(),
        });
        para.property = Some(props);

        let mut bookmark = BookmarkStart::default();
        bookmark.name = Some(Cow::Borrowed("header_anchor"));

        let mut run = Run::default();
        run.content.push(RunContent::Text(Text {
            text: "Header Title".into(),
            ..Default::default()
        }));

        para.content.push(ParagraphContent::BookmarkStart(bookmark));
        para.content.push(ParagraphContent::Run(run));

        // Setup mock context
        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut numbering_resolver = super::super::NumberingResolver::new(&docx);
        let mut image_extractor = super::super::ImageExtractor::new_skip();
        let options = crate::ConvertOptions::default();
        let style_resolver = super::super::StyleResolver::new(&docx.styles);
        let localization = crate::localization::KoreanLocalization;

        let mut context = super::ConversionContext {
            rels: &rels,
            numbering: &mut numbering_resolver,
            image_extractor: &mut image_extractor,
            options: &options,
            footnotes: Vec::new(),
            endnotes: Vec::new(),
            comments: Vec::new(),
            docx_comments: None,
            docx_footnotes: None,
            docx_endnotes: None,
            styles: &docx.styles,
            style_resolver: &style_resolver,
            localization: &localization,
        };

        // Convert
        let md = ParagraphConverter::convert(&para, &mut context).expect("Conversion failed");

        // Verify: Anchor should be on the line BEFORE the header
        // Expected: "<a id=\"header_anchor\"></a>\n# Header Title"
        assert_eq!(md, "<a id=\"header_anchor\"></a>\n# Header Title");
    }

    #[test]
    fn test_adjacent_anchors() {
        use rs_docx::document::BookmarkStart;

        // Create a paragraph with multiple adjacent bookmarks
        let mut para = Paragraph::default();

        let mut b1 = BookmarkStart::default();
        b1.name = Some(Cow::Borrowed("anchor1"));
        let mut b2 = BookmarkStart::default();
        b2.name = Some(Cow::Borrowed("anchor2"));

        let mut run = Run::default();
        run.content.push(RunContent::Text(Text {
            text: "Content".into(),
            ..Default::default()
        }));

        para.content.push(ParagraphContent::BookmarkStart(b1));
        para.content.push(ParagraphContent::BookmarkStart(b2));
        para.content.push(ParagraphContent::Run(run));

        // Setup minimal context
        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut numbering_resolver = super::super::NumberingResolver::new(&docx);
        let mut image_extractor = super::super::ImageExtractor::new_skip();
        let options = crate::ConvertOptions::default();
        let style_resolver = super::super::StyleResolver::new(&docx.styles);
        let localization = crate::localization::KoreanLocalization;

        let mut context = super::ConversionContext {
            rels: &rels,
            numbering: &mut numbering_resolver,
            image_extractor: &mut image_extractor,
            options: &options,
            footnotes: Vec::new(),
            endnotes: Vec::new(),
            comments: Vec::new(),
            docx_comments: None,
            docx_footnotes: None,
            docx_endnotes: None,
            styles: &docx.styles,
            style_resolver: &style_resolver,
            localization: &localization,
        };

        // Convert
        let md = ParagraphConverter::convert(&para, &mut context).expect("Conversion failed");

        // Verify both anchors are present
        assert_eq!(md, "<a id=\"anchor1\"></a><a id=\"anchor2\"></a>\nContent");
    }
}
