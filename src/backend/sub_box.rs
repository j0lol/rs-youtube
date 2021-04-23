use std::io;

use crate::backend::config::load_config;
use crate::backend::requests::request;

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub(crate) channel_id: String,
    pub(crate) channel_name: String,
    pub(crate) video_name: String,
    pub(crate) video_timestamp: String,
}

pub fn sub_box() -> Option<Vec<FeedItem>> {
    // SUB BOX

    let config = load_config()?;

    let mut output: String = String::from("");

    output = [output, String::from("Subscription Box: \n")].join("");

    let mut vec: Vec<FeedItem> = Vec::new();

    for i in 0..config.subscriptions.len() {
        //let res = request_browse(&client, config.subscriptions[i].as_str().unwrap()).unwrap();
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
         "browseId": config.subscriptions[i],
         "params": "EgZ2aWRlb3M%3D"
        }),
        })?;
        let channel_name: &serde_json::Value = &res["header"]["c4TabbedHeaderRenderer"]["title"];

        let video_name: &serde_json::Value = &res["contents"]["twoColumnBrowseResultsRenderer"]
            ["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]
            ["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][0]["gridVideoRenderer"]
            ["title"]["runs"][0]["text"];

        // Get timestamp
        let video_timestamp: &serde_json::Value = &res["contents"]
            ["twoColumnBrowseResultsRenderer"]["tabs"][1]["tabRenderer"]["content"]
            ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]
            ["gridRenderer"]["items"][0]["gridVideoRenderer"]["publishedTimeText"]["simpleText"];

        vec.push(FeedItem {
            channel_id: config.subscriptions[i].as_str().unwrap().to_string(),
            channel_name: channel_name.as_str().unwrap().to_string(),
            video_name: video_name.as_str().unwrap().to_string(),
            video_timestamp: video_timestamp.as_str().unwrap().to_string(),
        })
    }

    return Some(vec);
    // clear_screen();
    //     println!("{}", output);
    //
    //     let mut input = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");
    //
    //     let input = input.trim();
    //
    //     match input {
    //         "exit" => return Ok(()),
    //
    //         input => {
    //             let input: usize = input.parse()?;
    //             if input < config.subscriptions.len() {
    //                 clear_screen();
    //                 show_channel(config.subscriptions[input].as_str().unwrap());
    //             } else {
    //                 return Err("Not a valid number")?;
    //             }
    //         }
    //     }
    //
    //     Ok(())
}
