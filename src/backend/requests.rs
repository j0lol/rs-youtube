use ureq::Agent;

pub struct Request {
    pub(crate) url: String,
    pub(crate) body: serde_json::Value,
}

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
