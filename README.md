# rs-youtube

Command line YouTube/Twitch client that uses the web api so no login or API key is needed.

- Requires `mpv` and `youtube-dl`, or another video player capable of loading videos from YouTube/Twitch links (e.g. `celluloid`, even a web browser would work) 

## Installation

1. Make sure your video player is installed and is accessible by typing its name via command line (Typing `mpv` should print a help screen, for example)

2. [Install Rust (using `rustup`)](https://www.rust-lang.org/tools/install)

3. Install using `cargo`
```
cargo install rs-youtube
```

## Compilation
1. Make sure your video player is installed and is accessible by typing its name via command line (Typing `mpv` should print a help screen, for example)

2. [Install Rust (using `rustup`)](https://www.rust-lang.org/tools/install)

3. Clone and run the project using `cargo`
```
git clone https://github.com/j0lol/rs-youtube
cd rs-youtube
cargo install --path .
rs-youtube(.exe)
```
## Configuration 
This program will create a config file the first time it reads config (checking subscriptions or checking video player) in one of these directories:
- Linux:   `/home/USER/.config/rs-youtube/config.toml`
- Windows: `C:\Users\USER\AppData\Roaming\j0lol\rs-youtube\config\config.toml`
- macOS:   `/Users/USER/Library/Application Support/com.j0lol.rs-youtube/config.toml`

If you would like to edit the config, refer to the `config.toml.default` file.

The config file defaults to loading mpv, and will have 0 subscriptions. You can subscribe to a channel by searching for its channel page, then picking the "sub" option.

# Screenshots
## Main menu
![image](https://user-images.githubusercontent.com/24716467/116722112-ec266700-a9d5-11eb-8b62-bd7c62d5b447.png)

## Youtube search
![image](https://user-images.githubusercontent.com/24716467/116722194-052f1800-a9d6-11eb-8ae0-01d791c18dbf.png)

## Youtube channel
![image](https://user-images.githubusercontent.com/24716467/116722652-943c3000-a9d6-11eb-8ca3-d232511105f5.png)

## Youtube subscription feed
![image](https://user-images.githubusercontent.com/24716467/116722794-c2217480-a9d6-11eb-834e-920fc40ac60d.png)

## Twitch channel search
![image](https://user-images.githubusercontent.com/24716467/116722894-e4b38d80-a9d6-11eb-876b-c2a05b8bdde4.png)

## Twitch channel view
![image](https://user-images.githubusercontent.com/24716467/116722935-f006b900-a9d6-11eb-8487-91cc34b1f296.png)

## Twitch follow feed
![image](https://user-images.githubusercontent.com/24716467/116723327-4b38ab80-a9d7-11eb-8580-d7c88d323337.png)

## Testing
- `rs-youtube` has been thoroughly tested on linux, and somewhat tested on Windows and macOS
