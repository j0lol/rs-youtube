use crate::backend::requests::request;
#[derive(Debug, Clone)]
pub enum ChannelResults {
    Video(YoutubeVideo),
    Playlist(YoutubePlaylist),
    None,
}
pub trait Summary {
    fn summarize(&self) -> String;
}
#[derive(Debug, Clone)]
pub struct YoutubePlaylist {
    url: String,
    title: String,
}

impl Summary for YoutubePlaylist {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

#[derive(Debug, Clone)]
pub struct YoutubeVideo {
    id: String,
    title: String,
    timestamp: String,
}

impl Summary for YoutubeVideo {
    fn summarize(&self) -> String {
        format!("{}  {}", self.title, self.timestamp)
    }
}

pub fn show_channel(channel_id: &str) -> Vec<ChannelResults> {
    let res = request(crate::backend::requests::Request {
        url: String::from("https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8"),
        body: ureq::json!({
         "context": {
           "client": {
             "clientName": "WEB",
             "clientVersion": "2.20201210.01.00",
             "originalUrl": "https://www.youtube.com/",
             "platform": "DESKTOP",
             "clientFormFactor": "UNKNOWN_FORM_FACTOR",
             "newVisitorCookie": true
           }
         },
         "browseId": channel_id,
         "params": "EgZ2aWRlb3M%3D"
        }),
    }).unwrap();

    let channel_name: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["title"];
    let channel_name = channel_name.as_str().unwrap();

    // Get channel name
    let channel_subs: &serde_json::Value =
        &res["header"]["c4TabbedHeaderRenderer"]["subscriberCountText"]["simpleText"];
    let channel_subs = channel_subs.as_str().unwrap();

    let channel_playlist = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]
        ["tabRenderer"]["content"]["sectionListRenderer"]["subMenu"]["channelSubMenuRenderer"]
        ["playAllButton"]["buttonRenderer"]["navigationEndpoint"]["commandMetadata"]
        ["webCommandMetadata"]["url"];

    let mut vec = Vec::with_capacity(30);

    //output = [output, format!("{}\n{}\n\n", channel_name, channel_subs)].join("");
    //let mut arr: [&str; 30] = ["null"; 30];

    let parse_json =
        |json: &serde_json::Value| json.as_str().unwrap_or("null").parse::<String>().unwrap();

    for i in 0..30 {
        // Get ID
        let video_id: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]
            ["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]
            ["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]
            ["videoId"];

        // Get name
        let video_name: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]
            ["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]
            ["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]
            ["title"]["runs"][0]["text"];

        // Get timestamp
        let video_timestamp: &serde_json::Value = &res["contents"]
            ["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]
            ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]
            ["gridRenderer"]["items"][i]["gridVideoRenderer"]["publishedTimeText"]["simpleText"];

        let video = YoutubeVideo {
            id: parse_json(video_id),
            title: parse_json(video_name),
            timestamp: parse_json(video_timestamp),
        };
        //arr[i] = video_id;
        vec.push(ChannelResults::Video(video));
        // Render video list
    }
    // Print

    vec.push(ChannelResults::Playlist(YoutubePlaylist {
        url: parse_json(channel_playlist),
        title: format!("Uploads from {}", channel_name),
    }));

    vec
}

/*
#[derive(Debug, Clone)]
struct YoutubePlaylist {
    url: String,
    title: String,
}

impl Summary for YoutubePlaylist {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

#[derive(Debug, Clone)]
struct YoutubeVideo {
    id: String,
    title: String,
    timestamp: String,
}

impl Summary for YoutubeVideo {
    fn summarize(&self) -> String {
        format!("{}  {}", self.title, self.timestamp)
    }
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
*/
