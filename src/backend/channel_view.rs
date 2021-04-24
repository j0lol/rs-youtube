use crate::backend::requests::request;
use std::time::Duration;
use ureq::Agent;

#[derive(Debug, Clone)]
pub enum ChannelResults {
    Video(YoutubeVideo),
    Playlist(YoutubePlaylist),
    None(String),
}
pub trait Summary {
    fn summarize(&self) -> String;
}
#[derive(Debug, Clone)]
pub struct YoutubePlaylist {
    pub url: String,
    pub title: String,
}

impl Summary for YoutubePlaylist {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

#[derive(Debug, Clone)]
pub struct YoutubeVideo {
    pub id: String,
    pub title: String,
    pub timestamp: String,
}

impl Summary for YoutubeVideo {
    fn summarize(&self) -> String {
        format!("{}  {}", self.title, self.timestamp)
    }
}

pub fn show_channel(channel_id: &str) -> Vec<ChannelResults> {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

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
    }, Some(&agent)).unwrap();

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
