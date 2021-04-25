use std::time::Duration;

use serde_json::Value;
use ureq::Agent;

use crate::backend::config::load_config;
use crate::backend::requests::request;

#[derive(Debug, Clone)]
pub enum ChannelStatus {
    Live,
    Hosting,
    Offline,
}

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub(crate) channel_status: ChannelStatus,
    pub(crate) channel_name: String,
    pub(crate) channel_display_name: String,
    pub(crate) livestream_title: Option<String>,
    pub(crate) livestream_game: Option<String>,
    pub(crate) hosting_channel_name: Option<String>,
    pub(crate) hosting_channel_display_name: Option<String>,
    pub(crate) hosting_livestream_title: Option<String>,
}

pub fn follow_box() -> Option<Vec<FeedItem>> {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let config = load_config()?;

    let mut vec: Vec<FeedItem> = Vec::new();

    for i in 0..config.follows.len() {
        //let res = request_browse(&client, config.subscriptions[i].as_str().unwrap()).unwrap();

        let res = request(crate::backend::requests::Request {
            url: String::from("https://gql.twitch.tv/gql"),
            body: ureq::json!({"query":"query ChannelProfile_Query($login: String!) {\n  channel: user(login: $login) {\n    ...ChannelInfoCard_user\n    ...ChannelProfileVideos_user\n    id\n    login\n    displayName\n    stream {\n      id\n      broadcaster {\n        broadcastSettings {\n          title\n        }\n      }\n    }\n    hosting {\n      id\n      __typename\n      login\n      stream {\n        id\n        __typename\n      }\n    }\n  }\n}\n\nfragment ChannelInfoCard_user on User {\n  displayName\n  hosting {\n    id\n  }\n  stream {\n    type\n  }\n}\n\nfragment ChannelProfileVideos_user on User {\n  ...FeaturedContentCard_user\n  login\n  displayName\n  stream {\n    game {\n      name\n    }\n  }\n\n  hosting {\n    id\n  }\n}\n\nfragment FeaturedContentCard_user on User {\n  displayName\n  hosting {\n    id\n    login\n    displayName\n    stream {\n      broadcaster {\n        broadcastSettings {\n          title\n        }\n      }\n      type\n      game {\n        name\n        id\n      }\n      id\n    }\n  }\n}\n","variables":{"login":config.follows[i]},"operationName":"ChannelProfile_Query"}),
            header: Some(("Client-Id".to_string(), "kimne78kx3ncx6brgo4mv6wki5h1ko".to_string()))
        }, Some(&agent)).unwrap();

        // If channel is Live
        if res["data"]["channel"]["stream"]["type"] != Value::Null {
            vec.push(FeedItem {
                channel_status: ChannelStatus::Live,
                channel_name: config.follows[i].as_str().unwrap().to_string(),
                channel_display_name: res["data"]["channel"]["displayName"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                livestream_title: Some(
                    res["data"]["channel"]["stream"]["broadcaster"]["broadcastSettings"]["title"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
                livestream_game: Some(
                    res["data"]["channel"]["stream"]["game"]["name"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
                hosting_channel_name: None,
                hosting_channel_display_name: None,
                hosting_livestream_title: None,
            });
            // If channel is hosting
        } else if res["data"]["channel"]["hosting"] != Value::Null {
            vec.push(FeedItem {
                channel_status: ChannelStatus::Hosting,
                channel_name: config.follows[i].as_str().unwrap().to_string(),
                channel_display_name: res["data"]["channel"]["displayName"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                livestream_title: None,
                livestream_game: None,
                hosting_channel_name: Some(
                    res["data"]["channel"]["hosting"]["login"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
                hosting_channel_display_name: Some(
                    res["data"]["channel"]["hosting"]["displayName"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
                hosting_livestream_title: Some(
                    res["data"]["channel"]["hosting"]["stream"]["broadcaster"]["broadcastSettings"]
                        ["title"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
            });
        } else {
            vec.push(FeedItem {
                channel_status: ChannelStatus::Offline,
                channel_name: config.follows[i].as_str().unwrap().to_string(),
                channel_display_name: res["data"]["channel"]["displayName"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                livestream_title: None,
                livestream_game: None,
                hosting_channel_name: None,
                hosting_channel_display_name: None,
                hosting_livestream_title: None,
            });
        }
    }

    return Some(vec);
}
