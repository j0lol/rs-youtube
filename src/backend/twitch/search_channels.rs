use console::style;
use serde_json::Value;

use crate::backend::requests::request;
use std::time::Duration;
use ureq::Agent;

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub display_name: String,
}

// Perform a twitch channel search
pub fn perform_search(search_term: String) -> Option<Vec<Channel>> {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let res = request(
        crate::backend::requests::Request {
            url: String::from("https://gql.twitch.tv/gql"),
            body: ureq::json!([
	{
		"operationName": "SearchResultsPage_SearchResults",
		"variables": {
			"query": search_term,
			"options": null,
			"requestID": "d4c41496-f4cb-4c19-8fbb-ef241d7ceeea"
		},
		"extensions": {
			"persistedQuery": {
				"version": 1,
				"sha256Hash": "ee977ac21b324669b4c109be49ed3032227e8850bea18503d0ced68e8156c2a5"
			}
		}
	}
]),
            header: Some((
                "Client-Id".to_string(),
                "kimne78kx3ncx6brgo4mv6wki5h1ko".to_string(),
            )),
        },
        Some(&agent),
    )
    .unwrap();

    let len = res[0]["data"]["searchFor"]["channels"]["edges"]
        .as_array()
        .unwrap()
        .len();
    let mut vec = Vec::with_capacity(len);

    for i in 0..len {
        // Get current items

        let channel_name = &res[0]["data"]["searchFor"]["channels"]["edges"][i]["item"]["login"];
        let channel_display_name =
            &res[0]["data"]["searchFor"]["channels"]["edges"][i]["item"]["displayName"];
        vec.push(Channel {
            name: channel_name.as_str().unwrap().to_string(),
            display_name: channel_display_name.as_str().unwrap().to_string(),
        })
    }
    Some(vec)
}
