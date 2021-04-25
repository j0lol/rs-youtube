use crate::backend::config::load_config;
use crate::backend::requests::request;
use std::time::Duration;
use ureq::Agent;

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub channel_id: String,
    pub channel_name: String,
    pub video_name: String,
    pub video_timestamp: String,
}

pub fn sub_box() -> Option<Vec<FeedItem>> {
    // SUB BOX

    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let config = load_config()?;

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
            header: None,
        }, Some(&agent))?;
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
}
