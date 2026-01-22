#!/usr/bin/env python3
"""
Example script demonstrating dm2xcod Python bindings.
Install: pip install dm2xcod
"""

import sys


def main():
    try:
        import dm2xcod
    except ImportError:
        print("❌ dm2xcod not installed. Install with: pip install dm2xcod")
        sys.exit(1)

    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <input.docx> [output.md]")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else None

    try:
        markdown = dm2xcod.convert_docx(input_file)

        if output_file:
            with open(output_file, "w", encoding="utf-8") as f:
                f.write(markdown)
            print(f"✅ Converted '{input_file}' to '{output_file}'")
        else:
            print(markdown)

    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
