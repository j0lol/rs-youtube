use std::io;

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Results {
    Video(YoutubeVideo),
    Channel(YoutubeChannel),
    Playlist(YoutubePlaylist),
    Shelf(YoutubeShelf),
    None,
}

/// Summarize content of youtube search items so they can be rendered in text.
pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct YoutubeVideo {
    pub id: String,
    title: String,
    owner: String,
}

impl Summary for YoutubeVideo {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.owner)
    }
}

#[derive(Debug, Clone)]
pub struct YoutubeChannel {
    pub id: String,
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
pub struct YoutubePlaylist {
    pub id: String,
    title: String,
}

impl Summary for YoutubePlaylist {
    fn summarize(&self) -> String {
        format!("PLAYLIST: {}", self.title)
    }
}

// A shelf is a group of youtube videos
#[derive(Debug, Clone)]
pub struct YoutubeShelf {
    pub title: String,
    pub content: Vec<Results>,
}

impl Summary for YoutubeShelf {
    fn summarize(&self) -> String {
        let mut output = String::new();

        for i in 0..self.content.len() {
            let text = match &self.content[i] {
                Results::Video(video) => video.summarize(),
                Results::None => String::from("null"),
                _ => {
                    panic!("Shelf contains something other than videos! Please report this.")
                }
            };
            output = [output, format!("\t{}) {}\n", i, text)].join("");
        }

        format!("{}\n{}", self.title, output)
    }
}
use crate::backend::requests::request;

// Perform a youtube search
pub fn perform_search(search_term: String) -> Option<Vec<Results>> {
    //

    let res = request(crate::backend::requests::Request {
        url: String::from("https://www.youtube.com/youtubei/v1/search?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8"),
        body: ureq::json!({
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": "2.20201211.09.00"
            }
        },
        "query": search_term
    }),
    }, None).unwrap();

    let mut vec = Vec::with_capacity(30);

    let mut j = 0;
    let i = 0;

    // Loops through section list renderers until the section is NOT an adwords section
    // (signified through .promotedSparklesTextSearchRenderer)
    // This means this section contains actual videos instead of adwords.
    // Example: searching "youtube"
    loop {
        match res["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
            ["sectionListRenderer"]["contents"][j]["itemSectionRenderer"]["contents"][i]
            ["promotedSparklesTextSearchRenderer"]
        {
            Value::Null => {
                break;
            }
            _ => {
                j = j + 1;
            }
        }
        if j > 10 {
            panic!("Whoops! infinite loop!")
        }
    }

    for i in 0..21 {
        // Get current items
        let current_search_item = res["contents"]["twoColumnSearchResultsRenderer"]
            ["primaryContents"]["sectionListRenderer"]["contents"][j]["itemSectionRenderer"]
            ["contents"][i]
            .clone();

        // TODO: Fix adwords stuff (go to [2] instead of [0]
        let channel_renderer = current_search_item["channelRenderer"].clone();
        let video_renderer = current_search_item["videoRenderer"].clone();
        let playlist_renderer = current_search_item["playlistRenderer"].clone();
        let shelf_renderer = current_search_item["shelfRenderer"].clone();

        let parse_json =
            |json: &serde_json::Value| json.as_str().unwrap_or("null").parse::<String>().unwrap();
        let parse_video = || {
            let video_title = &video_renderer["title"]["runs"][0]["text"];
            let video_id = &video_renderer["videoId"];
            let video_owner = &video_renderer["ownerText"]["runs"][0]["text"];

            let video = YoutubeVideo {
                id: parse_json(video_id),
                title: parse_json(video_title),
                owner: parse_json(video_owner),
            };

            Results::Video(video)
        };
        let parse_channel = || {
            let channel_id = &channel_renderer["channelId"];
            let channel_name = &channel_renderer["title"]["simpleText"];
            let channel_subs = &channel_renderer["subscriberCountText"]["simpleText"];
            let channel_description = &channel_renderer["descriptionSnippet"]["runs"][0]["text"];
            let channel_video_count = &channel_renderer["videoCountText"]["runs"][0]["text"];

            let channel = YoutubeChannel {
                id: parse_json(channel_id),
                name: parse_json(channel_name),
                subs: parse_json(channel_subs),
                description: parse_json(channel_description),
                video_count: parse_json(channel_video_count),
            };

            Results::Channel(channel)
        };
        let parse_playlist = || {
            let playlist_title = &playlist_renderer["title"]["simpleText"];
            let playlist_id = &res["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
                ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][i]
                ["playlistRenderer"]["playlistId"];

            let playlist = YoutubePlaylist {
                id: parse_json(playlist_id),
                title: parse_json(playlist_title),
            };

            Results::Playlist(playlist)
        };
        let parse_shelf = || {
            let shelf_title = &shelf_renderer["title"]["simpleText"];

            let mut i = 0;
            let mut vec = Vec::new();
            loop {
                if shelf_renderer["content"]["verticalListRenderer"]["items"][i].is_null() {
                    break;
                } else {
                    let video_title = &shelf_renderer["content"]["verticalListRenderer"]["items"]
                        [i]["videoRenderer"]["title"]["runs"][0]["text"];
                    let video_id = &shelf_renderer["content"]["verticalListRenderer"]["items"][i]
                        ["videoRenderer"]["videoId"];
                    let video_owner = &shelf_renderer["content"]["verticalListRenderer"]["items"]
                        [i]["videoRenderer"]["ownerText"]["runs"][0]["text"];

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
        };
        let parse_none = || Results::None;
        let parse_misread = || {
            panic!("Something went wrong in parsing.");
        };

        match channel_renderer {
            Value::Object(_) => {
                vec.push(parse_channel());
            }
            Value::Null => match video_renderer {
                Value::Object(_) => vec.push(parse_video()),
                Value::Null => match playlist_renderer {
                    Value::Object(_) => vec.push(parse_playlist()),
                    Value::Null => match shelf_renderer {
                        Value::Null => {
                            parse_none();
                        }
                        Value::Object(_) => {
                            vec.push(parse_shelf());
                        }
                        _ => {
                            parse_misread();
                        }
                    },
                    _ => {
                        parse_misread();
                    }
                },
                _ => {
                    parse_misread();
                }
            },
            _ => {
                parse_misread();
            }
        }
    }
    //results_selector(vec);
    Some(vec)
}
