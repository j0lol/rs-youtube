use std::io::Read;
use std::process::{Command, Output};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let mut res = client.post("https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")
        .body(r#"{
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
  "browseId": "UCRC6cNamj9tYAO6h_RXd5xA",
  "params": "EgZ2aWRlb3M%3D"
 }"#)
        .send()?;


    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    //println!("Body:\n{}", body);
    // let mut res = reqwest::get("https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")?;
    // let mut body = String::new();
    // res.read_to_string(&mut body)?;
    //
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    let res: serde_json::Value = serde_json::from_str(&body).expect("Unable to parse");
    let channel_name: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["title"];
    let channel_name = channel_name.as_str().unwrap();

    // Get channel name
    let channel_subs: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["subscriberCountText"]["simpleText"];
    let channel_subs = channel_subs.as_str().unwrap();

    let mut arr:[&str;30] = ["null";30];

    for i in 0..30 {
    // Get ID
        let video_id: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["videoId"];
        let video_id = video_id.as_str().unwrap();

        // Get name
        let video_name: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][i]["gridVideoRenderer"]["title"]["runs"][0]["text"];
        let video_name = video_name.as_str().unwrap();

        arr[i] = video_id;
        println!("{}) {} by {} — https://youtube.com/watch?v={}, {}", i, video_name, channel_name, video_id, channel_subs);

    }
    // Print

    println!("Type in a video number to open it in MPV");


    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input.trim().parse().expect("Please type a number!");

    // Make command
    let command = make_url("mpv https://youtube.com/watch?v=", arr[input]);

    // Run MPV command
    run_command(&command);
    Ok(())
}

fn make_url(prefix: &str, suffix: &str) -> String {
    [prefix, suffix].join("")
}
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