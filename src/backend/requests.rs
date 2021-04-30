use ureq::Agent;

pub struct Request {
    pub(crate) url: String,
    pub(crate) body: serde_json::Value,
    pub(crate) header: Option<(String, String)>,
}

// TODO: Put body in a smart pointer to reduce copies
// Box - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so

pub fn request(request: Request, agent: Option<&Agent>) -> Option<serde_json::Value> {
    match agent {
        None => match request.header {
            Some((header_header, header_value)) => {
                let resp = ureq::post(&request.url)
                    .set(header_header.as_str(), header_value.as_str())
                    .send_json(request.body)
                    .ok()?;
                let body: serde_json::Value = resp.into_json().ok()?;
                Some(body)
            }
            None => {
                let resp = ureq::post(&request.url).send_json(request.body).ok()?;
                let body: serde_json::Value = resp.into_json().ok()?;
                Some(body)
            }
        },
        Some(agent) => match request.header {
            Some((header_header, header_value)) => {
                let resp = agent
                    .post(&request.url)
                    .set(header_header.as_str(), header_value.as_str())
                    .send_json(request.body)
                    .ok()?;
                let body: serde_json::Value = resp.into_json().ok()?;
                Some(body)
            }
            None => {
                let resp = agent.post(&request.url).send_json(request.body).ok()?;
                let body: serde_json::Value = resp.into_json().ok()?;
                Some(body)
            }
        },
    }
}
