use std::io::Read;
use std::process::{Command, Output, exit};
use std::io;
use std::env;
use directories::{BaseDirs, UserDirs, ProjectDirs};
use std::fs;
use std::path::Path;
use toml::value::Array;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    subscriptions: Array<>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    loop {

    // SUB BOX
    let project = ProjectDirs::from("com", "j0lol", "rs-youtube").unwrap();
    let mut config_dir = project.config_dir();

    fs::create_dir_all(config_dir)?;

    let config_path = format!("{}/config.toml",config_dir.to_str().unwrap());
    config_dir = std::path::Path::new(&config_path);

    let config: Config = toml::from_str(&fs::read_to_string(config_dir)?).unwrap();

    let client = reqwest::Client::new();

    let mut output: String = String::from("");

    println!("Loading...");
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
        //println!("{}) {} {}\n{}\n",i, channel_name, video_timestamp, video_name);
        output = [output, format!("{}) {} {}\n{}\n\n",i, channel_name, video_timestamp, video_name)].join("");


    }
    output = [output, String::from("Type in a channel's number to open it")].join("");

    print!("{esc}c", esc = 27 as char);
    println!("{}", output);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input.trim().parse().expect("Please type a number!");
    print!("{esc}c", esc = 27 as char);
    show_channel(config.subscriptions[input].as_str().unwrap());
    }

    Ok(())
}

// Make a /browse request
fn request_browse(client: &reqwest::Client, channel_id: &str) -> Option<serde_json::Value> {
    let mut res = client.post("https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")
        .body(format!("{{\r\n  \"context\": {{\r\n    \"client\": {{\r\n      \"clientName\": \"WEB\",\r\n      \"clientVersion\": \"2.20201210.01.00\",\r\n      \"originalUrl\": \"https://www.youtube.com/\",\r\n      \"platform\": \"DESKTOP\",\r\n      \"clientFormFactor\": \"UNKNOWN_FORM_FACTOR\",\r\n      \"newVisitorCookie\": true\r\n    }}\r\n  }},\r\n  \"browseId\": \"{}\",\r\n  \"params\": \"EgZ2aWRlb3M%3D\"\r\n }}", channel_id)
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

    // Render channel banner

    output = [output, format!("{}\n{}\n\n",channel_name,channel_subs)].join("");
    let mut arr:[&str;30] = ["null";30];

    for i in 0..30 {
        // Get ID
        let video_id: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["videoId"];
        let video_id = video_id.as_str().unwrap();

        // Get name
        let video_name: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["title"]["runs"][0]["text"];
        let video_name = video_name.as_str().unwrap();

        // Get timestamp
        let video_timestamp: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["publishedTimeText"]["simpleText"];
        let video_timestamp = video_timestamp.as_str().unwrap();


        arr[i] = video_id;
        // Render video list

        output = [output, format!("{}) {}  {}\n", i, video_name, video_timestamp)].join("");


    }
    // Print

    output = [output, String::from("\nType in a video number to open it in MPV")].join("");

    print!("{esc}c", esc = 27 as char);
    println!("{}", output);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input.trim().parse().expect("Please type a number!");

    // Make command
    let command = make_url("mpv https://youtube.com/watch?v=", arr[input]);

    // Run MPV command
    run_command(&command);
}