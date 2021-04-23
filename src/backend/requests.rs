use std::io::Read;

use ureq::Agent;

fn make_request() -> &'static str {
    "test"
}

pub struct Request {
    pub(crate) url: String,
    pub(crate) body: serde_json::Value,
}
/*
#[allow(unused)]
pub fn testing() {
    let new_request = Request {
        url: String::from("UUxuzbhuzcfghuidef"),
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
    };
    request(new_request);
    let search_body = ureq::json!({
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": "2.20201211.09.00"
            }
        },
        "query": "string"
    });
}
*/
pub fn request(request: Request, agent: Option<&Agent>) -> Option<serde_json::Value> {
    match agent {
        None => {
            let resp = ureq::post(&request.url).send_json(request.body).ok()?;
            let body: serde_json::Value = resp.into_json().ok()?;
            Some(body)
        }
        Some(agent) => {
            let resp = agent.post(&request.url).send_json(request.body).ok()?;
            let body: serde_json::Value = resp.into_json().ok()?;
            Some(body)
        }
    }
}

// Make a /youtubei/v1/search Request
fn request_search(client: &reqwest::Client, search_term: &str) -> Option<serde_json::Value> {
    let mut res = client
        .post("https://www.youtube.com/youtubei/v1/search?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")
        .body(
            format!("{{\r\n\t\"context\": {{\r\n\t\t\"client\": {{\r\n\t\t\t\"clientName\": \"WEB\",\r\n\t\t\t\"clientVersion\": \"2.20201211.09.00\"\r\n\t\t}}\r\n\t}},\r\n\t\"query\": \"{}\"\r\n}}", search_term)
        )
        .send().ok()?;

    let mut body = String::new();
    res.read_to_string(&mut body).ok()?;

    Some(serde_json::from_str(&body).expect("Unable to parse"))
}
