# README - markdown_minder

`markdown_minder` is a simple command-line application written in Rust that processes Markdown files with optional YAML frontmatter.

## Features

- Save content from `stdin` to Markdown files
- Support for YAML frontmatter in Markdown files
- Ability to specify custom frontmatter fields via command-line arguments
- Use of templates for predefined frontmatter structures
- Option to automatically generate a unique ID for frontmatter
- Command-line interface for seamless integration with shell scripts and workflows

## Installation

Before you can use `markdown_minder`, make sure you have Rust and Cargo installed on your machine. If you don't have them installed, you can download and install them from the official Rust website: <https://www.rust-lang.org/tools/install>.

Once Rust and Cargo are installed, you can install `markdown_minder` by running the following command in your terminal:

```bash
cargo install markdown_minder
```

## Usage

After installing `markdown_minder`, you can start using it to save `stdin` and other content your Markdown files with YAML frontmatter templates. Here's a simple example of how to use it:

```bash
echo "Pipe in content" | markdown_minder --output process your_file.md --frontmatter my_prop="my value" --title "Default Title" --template default_template.yml
```

Replace `your_file.md` with the path to your Markdown file.

## License

`markdown_minder` is released under the MIT License. See the LICENSE file for more details.

## Project status

`markdown_minder` is actively maintained. For the latest updates, check the repository's commit history.
