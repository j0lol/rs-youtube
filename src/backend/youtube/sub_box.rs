use crate::backend::config::load_config;
use crate::backend::requests::request;
use std::sync::mpsc;
use std::thread;
use toml::Value;

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub channel_id: String,
    pub channel_name: String,
    pub video_name: String,
    pub video_timestamp: String,
}

pub fn sub_box() -> Option<Vec<FeedItem>> {

    let config = load_config().unwrap();

    let mut vec: Vec<FeedItem> = Vec::new();

    let (tx, rx) = mpsc::channel();

    for i in config.youtube.subscriptions {
        let tx_1 = tx.clone();
        thread::spawn(move || {
            let item = get_feed_item(i);
            tx_1.send(item).unwrap();
        });
    }
    drop(tx);
    for received in &rx {
        //println!("Got: {:?}", received);
        vec.push(received);
    }
    //println!("{:?}", vec);

    Some(vec)

}


fn get_feed_item(i: Value) -> FeedItem {
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
         "browseId": i,
         "params": "EgZ2aWRlb3M%3D"
        }),
        header: None,
    }, None).unwrap();
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

    FeedItem {
        channel_id: i.as_str().unwrap().to_string(),
        channel_name: channel_name.as_str().unwrap().to_string(),
        video_name: video_name.as_str().unwrap().to_string(),
        video_timestamp: video_timestamp
            .as_str()
            .unwrap_or("(No Timestamp, probably streaming...)")
            .to_string(),
    }
}