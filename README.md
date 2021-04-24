# rs-youtube

Command line youtube client that uses the web api so no login or API key is needed.

- Requires `mpv` and `youtube-dl`, or another video player capable of loading videos from youtube links (eg. `celluloid`, even a web browser would work) 

To compile on windows or linux:

1. Make sure your video player is installed and is accessible by typing its name via command line (Typing `mpv` should print a help screen, for example)

2. [Install Rust (using rustup)](https://www.rust-lang.org/tools/install)

3. Clone and run the project using `cargo`
```
git clone https://github.com/j0lol/rs-youtube
cd rs-youtube
cargo install --path .
rs-youtube(.exe)
```

This program will create a config file the first time it reads config (checking subscriptions or checking video player) in one of these directories:
- Linux:   `/home/USER/.config/rs-youtube/config.toml`
- Windows: `C:\Users\USER\AppData\Roaming\j0lol\rs-youtube\config\config.toml`
- macOS:   `/Users/USER/Library/Application Support/com.j0lol.rs-youtube/config.toml`

If you would like to edit the config, the format looks like this:

`config.toml`
```toml
subscriptions = ["UCX6OQ3DkcsbYNE6H8uQQuVA","UCiYpKsB66LZsk7s4yhxJqlQ"]
video_player = "mpv"
```
The config file defaults to loading mpv, and will have 0 subscriptions. You can subscribe to a channel by searching for it's channel page, then picking the "sub" option.

# Screenshots
## "Sub box"
![image](https://user-images.githubusercontent.com/24716467/115159857-2403e500-a08d-11eb-8393-97d16ff7c31f.png)

## "Channel view"
![image](https://user-images.githubusercontent.com/24716467/115159867-37af4b80-a08d-11eb-9c70-7fd22609a26b.png)

Selecting a video will open it in MPV
