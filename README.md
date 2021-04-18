# rs-youtube

Newpipe clone for CLI.

- Requires `mpv` and `youtube-dl`

To compile on windows or linux:

1. [Install Rust (using rustup)](https://www.rust-lang.org/tools/install)

2. Clone and run the project using `cargo`
```
git clone https://github.com/j0lol/rs-youtube
cd rs-youtube
cargo run
```

Currently, the program will make a directory for configs not run without a config.toml file premade. (this will be fixed in the future)

Put a cargo.toml in one of these locations:
- Linux:   `/home/alice/.config/rs-youtube/cargo.toml`
- Windows: `C:\Users\Alice\AppData\Roaming\j0lol\rs-youtube\cargo.toml`
- macOS:   `/Users/Alice/Library/Application Support/com.j0lol.rs-youtube/cargo.toml`

Write a config similar to this:

`cargo.toml`
```toml
subscriptions = ["UCX6OQ3DkcsbYNE6H8uQQuVA","UCiYpKsB66LZsk7s4yhxJqlQ"]
```