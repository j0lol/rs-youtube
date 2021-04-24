// Stuff that im not using bc im using better stuff now

// Make a /youtubei/v1/search Request
#[allow(unused)]
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
#[allow(unused)]
fn testing() {
    let new_request = Request {
        url: String::from("UUxuzbhuzcfghuidef"),
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
    };
    request(new_request);
    let search_body = ureq::json!({
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": "2.20201211.09.00"
            }
        },
        "query": "string"
    });
}

fn make_request() -> &'static str {
    "test"
}

// Select results
fn results_selector(vec: Vec<Results>) {
    clear_screen();

    for i in 0..vec.len() {
        let message = match &vec[i] {
            Results::Video(video) => video.summarize(),
            Results::Channel(channel) => channel.summarize(),
            Results::Playlist(playlist) => playlist.summarize(),
            Results::Shelf(shelf) => shelf.summarize(),
            Results::None => String::from(""),
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
                        println!(
                            "▶ Now playing: {} by {}\nhttps://youtube.com/watch?v={}",
                            video.title, video.owner, video.id
                        );
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }

                    Results::Channel(channel) => {
                        show_channel(&*channel.id);
                    }

                    Results::Playlist(playlist) => {
                        println!(
                            "▶ Now playing playlist: {}\nhttps://youtube.com/playlist?list={}",
                            playlist.title, playlist.id
                        );
                        let command =
                            make_url("mpv https://youtube.com/playlist?list=", &*playlist.id);
                        run_command(&command);
                    }
                    Results::Shelf(shelf) => {
                        let vec = shelf.content.to_vec();
                        results_selector(vec);
                    }
                    Results::None => {
                        println!("Option picked is blank.")
                    }
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
            Results::Video(video) => video.summarize(),
            Results::Channel(channel) => channel.summarize(),
            Results::Playlist(playlist) => playlist.summarize(),
            Results::Shelf(shelf) => shelf.summarize(),
            Results::None => String::from(""),
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
                        println!(
                            "▶ Now playing: {} by {}\nhttps://youtube.com/watch?v={}",
                            video.title, video.owner, video.id
                        );
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }

                    Results::Channel(channel) => {
                        //println!("{} {}", channel.id, channel.name);
                        show_channel(&*channel.id);
                    }

                    Results::Playlist(playlist) => {
                        println!(
                            "▶ Now playing: {}\nhttps://youtube.com/playlist?list={}",
                            playlist.title, playlist.id
                        );
                        let command =
                            make_url("mpv https://youtube.com/playlist?list=", &*playlist.id);
                        run_command(&command);
                    }
                    Results::Shelf(shelf) => {
                        let vec = shelf.content.to_vec();
                        results_selector(vec);
                    }
                    Results::None => {
                        println!("Option picked is blank.")
                    }
                }
            } else {
                println!("Not a valid number!")
            }
        }
    }
}

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

    let res = request_browse(&client, channel_id).unwrap();
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

    output = [output, format!("{}\n{}\n\n", channel_name, channel_subs)].join("");
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

    vec.push(ChannelResults::Playlist(YoutubeChannelPlaylist {
        url: parse_json(channel_playlist),
        title: format!("Uploads from {}", channel_name),
    }));

    clear_screen();

    println!("{}", output);

    for i in 0..vec.len() {
        let message = match &vec[i] {
            ChannelResults::Video(video) => video.summarize(),
            ChannelResults::Playlist(playlist) => playlist.summarize(),
            ChannelResults::None => String::from("None"),
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
                        println!(
                            "▶ Now playing: {}\nhttps://youtube.com/watch?v={}",
                            video.title, video.id
                        );
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }
                    ChannelResults::Playlist(playlist) => {
                        println!(
                            "▶ Now playing playlist: {}\nhttps://youtube.com{}",
                            playlist.title, playlist.url
                        );
                        let command = make_url("mpv https://youtube.com", &*playlist.url);
                        run_command(&command);
                    }
                    ChannelResults::None => {
                        println!("Option picked is blank.")
                    }
                }
            } else {
                println!("Not a valid number!")
            }
        }
    }
}
// Make a link made of two parts
fn make_url(prefix: &str, suffix: &str) -> String {
    [prefix, suffix].join("")
}
