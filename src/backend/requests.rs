use std::io::Read;

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
pub fn request(request: Request) -> Option<serde_json::Value> {
    let resp = ureq::post(&request.url).send_json(request.body).ok()?;
    let body: serde_json::Value = resp.into_json().ok()?;
    Some(body)
}

// Make a /youtubei/v1/browse Request
fn request_browse(client: &reqwest::Client, channel_id: &str) -> Option<serde_json::Value> {
    let mut res = client.post("https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")
        .body(format!("{{\r\n  \"context\": {{\r\n    \"client\": {{\r\n      \"clientName\": \"WEB\",\r\n      \"clientVersion\": \"2.20201210.01.00\",\r\n      \"originalUrl\": \"https://www.youtube.com/\",\r\n      \"platform\": \"DESKTOP\",\r\n      \"clientFormFactor\": \"UNKNOWN_FORM_FACTOR\",\r\n      \"newVisitorCookie\": true\r\n    }}\r\n  }},\r\n  \"browseId\": \"{}\",\r\n  \"params\": \"EgZ2aWRlb3M%3D\"\r\n }}", channel_id)
        )
        .send().ok()?;

    let mut body = String::new();
    res.read_to_string(&mut body).ok()?;

    Some(serde_json::from_str(&body).expect("Unable to parse"))
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
