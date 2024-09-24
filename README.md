# omd

**omd** is a simple, fast, and lightweight Markdown renderer and previewer written in Rust. It allows you to convert Markdown files to HTML and preview them in your browser, either statically or with live-reload support.

## Features

- **Static Mode**: Convert Markdown files to HTML and open them directly in your default browser without running a server.
- **Server Mode**: Run a local server to preview your Markdown files with live-reload functionality as you edit them.
- **CommonMark Extensions**: Supports strikethrough, tables, footnotes, task lists, and smart punctuation.
- **Customizable Styling**: Includes default CSS styling, which can be customized by editing `style.css`.
- **Embedded Fonts and Favicon**: Uses embedded fonts and favicon for a consistent look and self-contained HTML output.

## Installation

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (for building from source)

### Build from Source

1. **Clone the Repository**

   git clone https://github.com/yourusername/omd.git
   cd omd


2. **Build the Project**

   cargo build --release


3. **Install**

   Optionally, you can install `omd` to your local Cargo bin directory:

   cargo install --path .


   This allows you to run `omd` from anywhere on your system.

## Usage

omd [OPTIONS] [FILE]


### Options

- `-s`, `--static-mode`: Run in static mode. Converts the Markdown file to HTML and opens it in your default browser without starting a server.

### Examples

#### Static Mode

Convert a Markdown file to HTML and open it in your browser:

omd --static-mode README.md


If no file is specified, `omd` will read from `stdin`:

cat README.md | omd --static-mode


#### Server Mode (Live Preview)

Start a local server to preview your Markdown file with live-reload functionality:

omd README.md


Open [http://localhost:3030](http://localhost:3030) in your browser. Whenever you save changes to `README.md`, the browser will automatically reload to reflect the updates.

## Configuration

### Custom CSS

You can customize the CSS styling by editing the `style.css` file located in the `src` directory or by replacing it with your own CSS file.

### Fonts and Favicon

The application uses embedded fonts and a favicon located in the `fonts` directory and `favicon.ico` respectively. To use your own fonts or icon, replace these files and update the code if necessary.

## Supported Markdown Extensions

`omd` supports several CommonMark extensions:

- **Strikethrough**
- **Tables**
- **Footnotes**
- **Task Lists**
- **Smart Punctuation**

These extensions are enabled by default to enhance the formatting capabilities of your Markdown files.

## How It Works

- **Static Mode**: Renders the Markdown to HTML, writes it to a temporary file, and opens it in your default browser.
- **Server Mode**: Starts a local web server using [Warp](https://github.com/seanmonstar/warp) and watches the Markdown file for changes using [Notify](https://github.com/notify-rs/notify). The browser automatically reloads when changes are detected.

## Dependencies

- [Pulldown-Cmark](https://github.com/raphlinus/pulldown-cmark) for parsing and rendering Markdown.
- [Warp](https://github.com/seanmonstar/warp) for running the web server in server mode.
- [Notify](https://github.com/notify-rs/notify) for watching file changes.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please follow these steps:

1. **Fork the repository**.
2. **Create a new branch** for your feature or bugfix.
3. **Commit your changes** with clear messages.
4. **Push to your fork** and submit a **Pull Request**.

Please make sure to update tests as appropriate.

## Acknowledgments

- Thanks to the Rust community for their amazing crates that make projects like this possible.
- Inspired by the need for a simple Markdown previewer without unnecessary overhead.

## Contact

For questions or suggestions, feel free to open an issue or reach out via email at [petar0golubovic@gmail.com](mailto:petar0golubovic@gmail.com).

