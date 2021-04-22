use std::io::{Read};
use std::process::{Command, Output};
use std::io;
use directories::{ProjectDirs};
use std::fs;
use toml::value::Array;
use serde_derive::Deserialize;
use serde_json::Value;


#[derive(Debug, Clone)]
enum Results {
    Video(YoutubeVideo),
    Channel(YoutubeChannel),
    Playlist(YoutubePlaylist),
    Shelf(YoutubeShelf),
    None,
}

#[derive(Debug, Clone)]
enum ChannelResults {
    Video(YoutubeChannelVideo),
    Playlist(YoutubeChannelPlaylist),
    None,
}

#[derive(Deserialize)]
struct Config {
    subscriptions: Array<>,
}

/// Summarize content of youtube search items so they can be rendered in text.
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Results {
    fn summarize(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct YoutubeVideo {
    id: String,
    title: String,
    owner: String,
}

impl Summary for YoutubeVideo {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.owner)
    }
}

#[derive(Debug, Clone)]
struct YoutubeChannelVideo {
    id: String,
    title: String,
    timestamp: String,
}

impl Summary for YoutubeChannelVideo {
    fn summarize(&self) -> String {
        format!("{}  {}", self.title, self.timestamp)
    }
}

#[derive(Debug, Clone)]
struct YoutubeChannel {
    id: String,
    name: String,
    subs: String,
    description: String,
    video_count: String,
}

impl Summary for YoutubeChannel {
    fn summarize(&self) -> String {
        format!("{} ({})", self.name, self.subs)
    }
}

#[derive(Debug, Clone)]
struct YoutubePlaylist {
    id: String,
    title: String,
}

impl Summary for YoutubePlaylist {
    fn summarize(&self) -> String {
        format!("PLAYLIST: {}", self.title)
    }
}

#[derive(Debug, Clone)]
struct YoutubeChannelPlaylist {
    url: String,
    title: String,
}

impl Summary for YoutubeChannelPlaylist {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

// A shelf is a group of youtube videos
#[derive(Debug, Clone)]
struct YoutubeShelf {
    title: String,
    content: Vec<Results>,
}

impl Summary for YoutubeShelf {
    fn summarize(&self) -> String {
        let mut output = String::new();

        for i in 0..self.content.len() {
            let text = match &self.content[i] {
                Results::Video(video) => { video.summarize() }
                Results::None => { String::from("null") }
                _ => { panic!("Shelf contains something other than videos! Please report this.") }
            };
            output = [output, format!("\t{}) {}\n", i, text)].join("");
        }

        format!("{}\n{}", self.title, output)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        clear_screen();

        println!("1) Load Subscriptions");
        println!("2) Search");
        println!("exit)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        let _result = match input {
            "1" => { sub_box() }
            "2" => { perform_search() }
            "exit" | "quit" => break,
            string => {
                println!("Please input a valid option. {}", string);
                Ok(())
            }
        };
    }

    Ok(())
}

// Make a /youtubei/v1/browse request
fn request_browse(client: &reqwest::Client, channel_id: &str) -> Option<serde_json::Value> {
    let mut res = client.post("https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")
        .body(format!("{{\r\n  \"context\": {{\r\n    \"client\": {{\r\n      \"clientName\": \"WEB\",\r\n      \"clientVersion\": \"2.20201210.01.00\",\r\n      \"originalUrl\": \"https://www.youtube.com/\",\r\n      \"platform\": \"DESKTOP\",\r\n      \"clientFormFactor\": \"UNKNOWN_FORM_FACTOR\",\r\n      \"newVisitorCookie\": true\r\n    }}\r\n  }},\r\n  \"browseId\": \"{}\",\r\n  \"params\": \"EgZ2aWRlb3M%3D\"\r\n }}", channel_id)
        )
        .send().ok()?;


    let mut body = String::new();
    res.read_to_string(&mut body).ok()?;

    Some(serde_json::from_str(&body).expect("Unable to parse"))
}

// Make a /youtubei/v1/search request
fn request_search(client: &reqwest::Client, search_term: &str) -> Option<serde_json::Value> {
    let mut res = client
        .post("https://www.youtube.com/youtubei/v1/search?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")
        .body(
            format!("{{\r\n\t\"context\": {{\r\n\t\t\"client\": {{\r\n\t\t\t\"clientName\": \"WEB\",\r\n\t\t\t\"clientVersion\": \"2.20201211.09.00\"\r\n\t\t}}\r\n\t}},\r\n\t\"query\": \"{}\"\r\n}}", search_term)
        )
        .send().ok()?;

    let mut body = String::new();
    res.read_to_string(&mut body).ok()?;

    Some(serde_json::from_str(&body).expect("Unable to parse"))
}

// Make a link made of two parts
fn make_url(prefix: &str, suffix: &str) -> String {
    [prefix, suffix].join("")
}

// Run a command on windows, linux or mac
fn run_command(command: &str) -> Output {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
    output
}

fn show_channel(channel_id: &str) {
    let client = reqwest::Client::new();

    let mut output: String = String::from("");

    println!("Loading...");
    // let args: Vec<String> = env::args().collect();
    //
    // let channel_id = &args[1];

    let res = request_browse(&client, channel_id).unwrap();
    let channel_name: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["title"];
    let channel_name = channel_name.as_str().unwrap();

    // Get channel name
    let channel_subs: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["subscriberCountText"]["simpleText"];
    let channel_subs = channel_subs.as_str().unwrap();

    let channel_playlist = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["subMenu"]["channelSubMenuRenderer"]["playAllButton"]["buttonRenderer"]["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"];

    let mut vec = Vec::with_capacity(30);

    output = [output, format!("{}\n{}\n\n", channel_name, channel_subs)].join("");
    //let mut arr: [&str; 30] = ["null"; 30];

    let parse_json = |json: &serde_json::Value| {
        json.as_str()
            .unwrap_or("null")
            .parse::<String>()
            .unwrap()
    };

    for i in 0..30 {
        // Get ID
        let video_id: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["videoId"];

        // Get name
        let video_name: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["title"]["runs"][0]["text"];

        // Get timestamp
        let video_timestamp: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["publishedTimeText"]["simpleText"];


        let video = YoutubeChannelVideo {
            id: parse_json(video_id),
            title: parse_json(video_name),
            timestamp: parse_json(video_timestamp),
        };
        //arr[i] = video_id;
        vec.push(ChannelResults::Video(video));
        // Render video list

    }
    // Print

    vec.push(ChannelResults::Playlist(YoutubeChannelPlaylist { url: parse_json(channel_playlist), title: format!("Uploads from {}", channel_name) }));



    clear_screen();

    println!("{}", output);

    for i in 0..vec.len() {
        let message = match &vec[i] {
            ChannelResults::Video(video) => { video.summarize() }
            ChannelResults::Playlist(playlist) => { playlist.summarize() }
            ChannelResults::None => { String::from("None") }
        };
        println!("{}) {}", i, message);
    }
    println!("Select an item to open it in MPV.");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();


    match input {
        "exit" => {}
        input => {
            let input: usize = input.parse().unwrap();
            if input < vec.len() {
                clear_screen();

                match &vec[input] {
                    ChannelResults::Video(video) => {
                        println!("▶ Now playing: {}\nhttps://youtube.com/watch?v={}", video.title, video.id);
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }
                    ChannelResults::Playlist(playlist) => {
                        println!("▶ Now playing playlist: {}\nhttps://youtube.com{}", playlist.title, playlist.url);
                        let command = make_url("mpv https://youtube.com", &*playlist.url);
                        run_command(&command);
                    }
                    ChannelResults::None => { println!("Option picked is blank.") }
                }
            } else {
                println!("Not a valid number!")
            }
        }
    }
}

fn sub_box() -> Result<(), Box<dyn std::error::Error>> {
    // SUB BOX
    let project = ProjectDirs::from
        ("com", "j0lol", "rs-youtube").unwrap();
    let mut config_dir = project.config_dir();

    fs::create_dir_all(config_dir)?;

    let config_path = format!("{}/config.toml", config_dir.to_str().unwrap());
    config_dir = std::path::Path::new(&config_path);

    let config: Config = toml::from_str(&fs::read_to_string(config_dir)?)
        .unwrap_or(Config { subscriptions: Array::new() });

    let client = reqwest::Client::new();

    let mut output: String = String::from("");

    println!("Loading...");

    output = [output, String::from("Subscription Box: \n")].join("");

    for i in 0..config.subscriptions.len() {
        let res = request_browse(&client, config.subscriptions[i].as_str().unwrap()).unwrap();
        let channel_name: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["title"];
        let channel_name = channel_name.as_str().unwrap();

        // Get channel name
        //let channel_subs: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["subscriberCountText"]["simpleText"];
        //let channel_subs = channel_subs.as_str().unwrap();
        let video_name: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][0]["gridVideoRenderer"]["title"]["runs"][0]["text"];
        let video_name = video_name.as_str().unwrap();
        // Get timestamp
        let video_timestamp: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][0]["gridVideoRenderer"]["publishedTimeText"]["simpleText"];
        let video_timestamp = video_timestamp.as_str().unwrap();

        // Print channel name
        output = [output, format!("{}) {} {}\n{}\n\n", i, channel_name, video_timestamp, video_name)].join("");
    }
    output = [output, String::from("Type in a channel's number to open it")].join("");


    clear_screen();
    println!("{}", output);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input {
        "exit" => return Ok(()),

        input => {
            let input: usize = input.parse()?;
            if input < config.subscriptions.len() {
                clear_screen();
                show_channel(config.subscriptions[input].as_str().unwrap());
            } else {
                return Err("Not a valid number")?
            }
        }
    }

    Ok(())
}

// Perform a youtube search
fn perform_search() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    println!("Search youtube:");
    let client = reqwest::Client::new();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let res = request_search(&client, input).unwrap();

    let mut vec = Vec::with_capacity(30);

    let mut j = 0;
    let i = 0;

    // Loops through section list renderers until the section is NOT an adwords section
    // (signified through .promotedSparklesTextSearchRenderer)
    // This means this section contains actual videos instead of adwords.
    // Example: searching "youtube"
    loop {
        match res["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]["sectionListRenderer"]["contents"][j]["itemSectionRenderer"]["contents"][i]["promotedSparklesTextSearchRenderer"] {
            Value::Null => { break; }
            _ => { j = j + 1; }
        }
        if j > 10 { panic!("Whoops! infinite loop!") }
    }

    for i in 0..21 {

        // Get current items
        let current_search_item = res["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]["sectionListRenderer"]["contents"][j]["itemSectionRenderer"]["contents"][i].clone();

        println!("[{}] {:?}", i, current_search_item);
        let parse_json = |json: &serde_json::Value| {
            json.as_str()
                .unwrap_or("null")
                .parse::<String>()
                .unwrap()
        };

        //TODO implement radioRenderer (MIX) and horizontalCardListRenderer ("Searches related to x", "People also searched for")
        // Discovered by searching "videogamedunkey" and "caddicarus"

        enum RendererType {
            Channel(serde_json::Value),
            Video(serde_json::Value),
            Playlist(serde_json::Value),
            Shelf(serde_json::Value),
            None,
        }
        impl RendererType {
            fn parse(&self) -> Results {
                let parse_json = |json: &serde_json::Value| {
                    json.as_str()
                        .unwrap_or("null")
                        .parse::<String>()
                        .unwrap()
                };
                match self {
                    RendererType::Channel(channel) => {
                        let channel_id = &channel["channelId"];
                        let channel_name = &channel["title"]["simpleText"];
                        let channel_subs = &channel["subscriberCountText"]["simpleText"];
                        let channel_description = &channel["descriptionSnippet"]["runs"][0]["text"];
                        let channel_video_count = &channel["videoCountText"]["runs"][0]["text"];

                        let channel = YoutubeChannel {
                            id: parse_json(channel_id),
                            name: parse_json(channel_name),
                            subs: parse_json(channel_subs),
                            description: parse_json(channel_description),
                            video_count: parse_json(channel_video_count),
                        };

                        Results::Channel(channel)
                    }
                    RendererType::Video(video) => {
                        let video_title = &video["title"]["runs"][0]["text"];
                        let video_id = &video["videoId"];
                        let video_owner = &video["ownerText"]["runs"][0]["text"];

                        let video = YoutubeVideo {
                            id: parse_json(video_id),
                            title: parse_json(video_title),
                            owner: parse_json(video_owner),
                        };

                        Results::Video(video)
                    }
                    RendererType::Playlist(playlist) => {
                        let playlist_title = &playlist["title"]["simpleText"];
                        let playlist_id = &playlist["playlistId"];

                        let playlist = YoutubePlaylist {
                            id: parse_json(playlist_id),
                            title: parse_json(playlist_title),
                        };

                        Results::Playlist(playlist)
                    }
                    RendererType::Shelf(shelf) => {
                        let shelf_title = &shelf["title"]["simpleText"];

                        let mut i = 0;
                        let mut vec = Vec::new();
                        loop {
                            if shelf["content"]["verticalListRenderer"]["items"][i].is_null() {
                                break;
                            } else {
                                let video_title = &shelf["content"]["verticalListRenderer"]["items"][i]["videoRenderer"]["title"]["runs"][0]["text"];
                                let video_id = &shelf["content"]["verticalListRenderer"]["items"][i]["videoRenderer"]["videoId"];
                                let video_owner = &shelf["content"]["verticalListRenderer"]["items"][i]["videoRenderer"]["ownerText"]["runs"][0]["text"];

                                let video = YoutubeVideo {
                                    id: parse_json(video_id),
                                    title: parse_json(video_title),
                                    owner: parse_json(video_owner),
                                };


                                vec.push(Results::Video(video));
                            }
                            i = i + 1;
                        }

                        let shelf = YoutubeShelf {
                            title: parse_json(shelf_title),
                            content: vec,
                        };

                        Results::Shelf(shelf)
                    }
                    RendererType::None => Results::None
                }
            }
        }
        let renderer = if !current_search_item["channelRenderer"].is_null() {
            RendererType::Channel(current_search_item["channelRenderer"].clone())
        } else if !current_search_item["videoRenderer"].is_null() {
            RendererType::Video(current_search_item["videoRenderer"].clone())
        } else if !current_search_item["playlistRenderer"].is_null() {
            RendererType::Playlist(current_search_item["playlistRenderer"].clone())
        } else if !current_search_item["shelfRenderer"].is_null() {
            RendererType::Shelf(current_search_item["shelfRenderer"].clone())
        } else { RendererType::None } ;



        let parse_none = || {
            Results::None
        };
        let parse_misread = || {
            panic!("Something went wrong in parsing.");
        };

        vec.push(renderer.parse());
    }
    results_selector(vec);
    Ok(())
}

// Select results
fn results_selector(vec: Vec<Results>) {
    clear_screen();

    for i in 0..vec.len() {
        let message = match &vec[i] {
            Results::Video(video) => { video.summarize() }
            Results::Channel(channel) => { channel.summarize() }
            Results::Playlist(playlist) => { playlist.summarize() }
            Results::Shelf(shelf) => { shelf.summarize() }
            Results::None => { String::from("") }
        };
        println!("{}) {}", i, message);
    }
    println!("Select an item.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input {
        "exit" => {}
        input => {
            let input: usize = input.parse().unwrap();
            if input < vec.len() {
                clear_screen();

                match &vec[input] {
                    Results::Video(video) => {
                        println!("▶ Now playing: {} by {}\nhttps://youtube.com/watch?v={}", video.title, video.owner, video.id);
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }

                    Results::Channel(channel) => {
                        //println!("{} {}", channel.id, channel.name);
                        show_channel(&*channel.id);
                    }

                    Results::Playlist(playlist) => {
                        println!("▶ Now playing playlist: {}\nhttps://youtube.com/playlist?list={}", playlist.title, playlist.id);
                        let command = make_url("mpv https://youtube.com/playlist?list=", &*playlist.id);
                        run_command(&command);
                    }
                    Results::Shelf(shelf) => {
                        let vec = shelf.content.to_vec();
                        results_selector(vec);
                    }
                    Results::None => { println!("Option picked is blank.") }
                }
            } else {
                println!("Not a valid number!")
            }
        }
    }
}


// Select results
fn channel_video_selector(vec: Vec<Results>) {
    clear_screen();

    for i in 0..vec.len() {
        let message = match &vec[i] {
            Results::Video(video) => { video.summarize() }
            Results::Channel(channel) => { channel.summarize() }
            Results::Playlist(playlist) => { playlist.summarize() }
            Results::Shelf(shelf) => { shelf.summarize() }
            Results::None => { String::from("") }
        };
        println!("{}) {}", i, message);
    }
    println!("Select an item.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input {
        "exit" => {}
        input => {
            let input: usize = input.parse().unwrap();
            if input < vec.len() {
                clear_screen();

                match &vec[input] {
                    Results::Video(video) => {
                        println!("▶ Now playing: {} by {}\nhttps://youtube.com/watch?v={}", video.title, video.owner, video.id);
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }

                    Results::Channel(channel) => {
                        //println!("{} {}", channel.id, channel.name);
                        show_channel(&*channel.id);
                    }

                    Results::Playlist(playlist) => {
                        println!("▶ Now playing: {}\nhttps://youtube.com/playlist?list={}", playlist.title, playlist.id);
                        let command = make_url("mpv https://youtube.com/playlist?list=", &*playlist.id);
                        run_command(&command);
                    }
                    Results::Shelf(shelf) => {
                        let vec = shelf.content.to_vec();
                        results_selector(vec);
                    }
                    Results::None => { println!("Option picked is blank.") }
                }
            } else {
                println!("Not a valid number!")
            }
        }
    }
}


fn clear_screen() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cls").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        })
    } else {
        Command::new("clear").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        })
    };

    print!("{}", String::from_utf8_lossy(&output.stdout));
}
