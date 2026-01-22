import dm2xcod
import os


def main():
    # Path to a sample DOCX file (referencing one from the root samples)
    # Adjust this path if you want to test a different file
    script_dir = os.path.dirname(os.path.abspath(__file__))
    sample_docx = os.path.join(script_dir, "../../samples/포항장성동_영검보.docx")

    if not os.path.exists(sample_docx):
        print(f"Error: Sample file not found at {sample_docx}")
        return

    print(f"Converting {sample_docx}...")

    try:
        # Convert DOCX to Markdown
        markdown = dm2xcod.convert_docx(sample_docx)

        # Print first 500 characters as preview
        print("\n--- Conversion Success! Preview (first 500 chars) ---")
        print(markdown[:500])
        print("...\n---------------------------------------------------")

        # Save to file
        output_file = "output.md"
        with open(output_file, "w", encoding="utf-8") as f:
            f.write(markdown)
        print(f"Full output saved to {output_file}")

    except Exception as e:
        print(f"Conversion failed: {e}")


if __name__ == "__main__":
    main()
