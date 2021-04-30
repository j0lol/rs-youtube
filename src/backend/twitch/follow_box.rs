use serde_json::Value;

use crate::backend::config::load_config;
use crate::backend::requests::request;
use std::thread;
use std::sync::mpsc;

// TODO: use serde_query for querying res

#[derive(Debug, Clone)]
pub enum ChannelStatus {
    Live,
    Hosting,
    Offline,
}
//
// #[derive(Debug, Clone)]
// pub enum FeedStatus {
//     Live(LiveFeedItem),
//     Hosting(HostingFeedItem),
//     Offline(OfflineFeedItem),
// }
//
// pub enum LiveMetadata {
//     DisplayName(String),
//     Title(String),
//     Game(String),
// }

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
// #[derive(Debug, Clone, DeserializeQuery)]
// pub struct LiveFeedItem {
//     #[query(".data.channel.displayName")]
//     pub channel_display_name: String,
//     #[query(".data.channel.stream.broadcaster.broadcastSettings.title")]
//     title: String,
//     #[query(".data.channel.stream.game.name")]
//     game: String,
// }
//
// #[derive(Debug, Clone, DeserializeQuery)]
// struct HostingFeedItem {
//     #[query(".data.channel.displayName")]
//     channel_display_name: String,
//     #[query(".data.channel.hosting.login")]
//     hosting_channel_name: String,
//     #[query(".data.channel.hosting.displayName")]
//     hosting_channel_display_name: String,
//     #[query(".data.channel.hosting.stream.broadcaster.broadcastSettings.title")]
//     hosting_livestream_title: String,
// }
//
// #[derive(Debug, Clone, DeserializeQuery)]
// struct OfflineFeedItem {
//     #[query(".data.channel.displayName")]
//     channel_display_name: String,
// }

pub fn follow_box() -> Option<Vec<FeedItem>> {
    let config = load_config()?;

    let mut vec: Vec<FeedItem> = Vec::new();

    // Multithreading: Open new channel
    // tx: send, rx: receive
    let (tx, rx) = mpsc::channel();

    // Loop through config
    for i in config.twitch.follows {

        // Clone tx so each thread can use their own sender
        let tx_1 = tx.clone();

        // Spawn a new thread
        thread::spawn(move || {
            // Run get_feed_item()
            let item = get_feed_item(i);

            // Send output to sender
            tx_1.send(item).unwrap();
        });
    }

    // Drop tx, as it is unused and does not go out of scope.
    // Removing this will make the program hang.
    drop(tx);

    // Loop through inputs as they are received and add them to vec
    for received in &rx {
        vec.push(received);
    }

    Some(vec)
}

fn get_feed_item(i: toml::value::Value) -> FeedItem {

    //let res = request_browse(&client, config.subscriptions[i].as_str().unwrap()).unwrap();

    // Make GraphQl request
    let res = request(crate::backend::requests::Request {
        url: String::from("https://gql.twitch.tv/gql"),
        body: ureq::json!({"query":"query ChannelProfile_Query($login: String!) {\n  channel: user(login: $login) {\n    ...ChannelInfoCard_user\n    ...ChannelProfileVideos_user\n    id\n    login\n    displayName\n    stream {\n      id\n      broadcaster {\n        broadcastSettings {\n          title\n        }\n      }\n    }\n    hosting {\n      id\n      __typename\n      login\n      stream {\n        id\n        __typename\n      }\n    }\n  }\n}\n\nfragment ChannelInfoCard_user on User {\n  displayName\n  hosting {\n    id\n  }\n  stream {\n    type\n  }\n}\n\nfragment ChannelProfileVideos_user on User {\n  ...FeaturedContentCard_user\n  login\n  displayName\n  stream {\n    game {\n      name\n    }\n  }\n\n  hosting {\n    id\n  }\n}\n\nfragment FeaturedContentCard_user on User {\n  displayName\n  hosting {\n    id\n    login\n    displayName\n    stream {\n      broadcaster {\n        broadcastSettings {\n          title\n        }\n      }\n      type\n      game {\n        name\n        id\n      }\n      id\n    }\n  }\n}\n","variables":{"login":i},"operationName":"ChannelProfile_Query"}),
        header: Some(("Client-Id".to_string(), "kimne78kx3ncx6brgo4mv6wki5h1ko".to_string())),
    }, None).unwrap();

    // If channel is Live
    let item =
        if res["data"]["channel"]["stream"]["type"] != Value::Null {
            FeedItem {
                channel_status: ChannelStatus::Live,
                channel_name: i.as_str().unwrap().to_string(),
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
            }
            // If channel is hosting
        } else if res["data"]["channel"]["hosting"] != Value::Null {
            FeedItem {
                channel_status: ChannelStatus::Hosting,
                channel_name: i.as_str().unwrap().to_string(),
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
            }
        } else {
            FeedItem {
                channel_status: ChannelStatus::Offline,
                channel_name: i.as_str().unwrap().to_string(),
                channel_display_name: res["data"]["channel"]["displayName"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                livestream_title: None,
                livestream_game: None,
                hosting_channel_name: None,
                hosting_channel_display_name: None,
                hosting_livestream_title: None,
            }
        };
    item
}