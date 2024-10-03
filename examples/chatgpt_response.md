To ensure that `style.css` is always loaded in a CLI application, regardless of the current working directory, you need to consider how to package and reference this file properly. When you run the CLI from different directories, relative paths like `./src/style.css` might not work because they depend on the current working directory at runtime.

Here are a few approaches you can take:

### 1. Use an Absolute Path or the Executable's Directory
You can use the directory of the executable as a base for finding your `style.css`. This way, no matter where the CLI is executed from, it can find the CSS file relative to the executable itself. You can get the directory of the executable with `std::env::current_exe()`.

Here’s how you can modify your function:

```rust
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn read_style_css() -> String {
    let default_css = r#"
        body { font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; max-width: 800px; margin: 0 auto; }
        pre { background-color: #f4f4f4; padding: 10px; border-radius: 5px; }
        code { background-color: #f4f4f4; padding: 2px 4px; border-radius: 3px; }
    "#.to_string();

    let mut contents = String::new();

    // Get the path of the executable
    if let Ok(exe_path) = env::current_exe() {
        let mut css_path = exe_path;
        css_path.pop();  // Remove the executable file name
        css_path.push("src/style.css");  // Add the relative path to style.css

        // Try to open and read the file
        if let Ok(mut file) = File::open(&css_path) {
            if let Err(_) = file.read_to_string(&mut contents) {
                contents = default_css;
            }
        } else {
            contents = default_css;
        }
    } else {
        contents = default_css;
    }

    contents
}
```

This approach ensures that your program always looks for `style.css` relative to the executable’s location, regardless of where the user runs the CLI.

### 2. Embed the CSS File into the Binary Using `include_str!`
If the CSS file is static and doesn’t change often, you can include it directly into your Rust binary using the `include_str!` macro. This method will compile the CSS file into the binary, making it portable and removing the need to load it from a file at runtime.

Here’s how to use `include_str!`:

```rust
fn read_style_css() -> String {
    let default_css = include_str!("../src/style.css").to_string();  // Compile-time inclusion

    default_css
}
```

With this approach, `style.css` is included during the build process, so you no longer need to worry about the file system or runtime loading issues.

### 3. Use the `dirs` crate to Find a Standard Configuration Directory
If you want to provide the flexibility of having a user-editable `style.css` but want to ensure it is always found, you can use the `dirs` crate to find standard directories for configuration files (e.g., `$HOME/.config/your_app` on Linux).

You can install the `dirs` crate by adding it to your `Cargo.toml`:

```toml
[dependencies]
dirs = "4.0"
```

And then modify your function:

```rust
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use dirs::config_dir;

fn read_style_css() -> String {
    let default_css = r#"
        body { font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; max-width: 800px; margin: 0 auto; }
        pre { background-color: #f4f4f4; padding: 10px; border-radius: 5px; }
        code { background-color: #f4f4f4; padding: 2px 4px; border-radius: 3px; }
    "#.to_string();

    let mut contents = String::new();

    // Use the config directory (e.g., ~/.config/your_app/style.css)
    if let Some(mut config_path) = config_dir() {
        config_path.push("your_app/style.css");  // Adjust to your app name

        // Try to open and read the file
        if let Ok(mut file) = File::open(&config_path) {
            if let Err(_) = file.read_to_string(&mut contents) {
                contents = default_css;
            }
        } else {
            contents = default_css;
        }
    } else {
        contents = default_css;
    }

    contents
}
```

This method looks for the `style.css` in a configuration directory that follows platform conventions (e.g., `~/.config` on Linux, `%APPDATA%` on Windows). It gives users the ability to override the default CSS.

---

Which one of these approaches fits your CLI the best? (๑>ᴗ<๑)
