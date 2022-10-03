//GraphStreaming API의 최소화된 구현 https://github.com/gephi/gephi/wiki/GraphStreaming
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct Gephi {
    pub hostname: String,
    pub workspace: String,
    pub enable: bool,
}

impl Gephi {
    pub async fn add_node(&self, id: usize, label: &str) -> reqwest::Result<reqwest::Response> {
        return Client::new()
            .post(format!(
                "http://{}/{}?operation=updateGraph",
                self.hostname, self.workspace
            ))
            .body(format!(r#"{{"an":{{{}:{{"label":"{}"}}}}}}"#, id, label))
            .send()
            .await;
    }

    pub async fn add_edge(
        &self,
        id: usize,
        source: usize,
        target: usize,
        weight: u32,
    ) -> reqwest::Result<reqwest::Response> {
        return Client::new()
            .post(format!(
                "http://{}/{}?operation=updateGraph",
                self.hostname, self.workspace
            ))
            .body(format!(
                r#"{{"ae":{{{}:{{"source":{},"target":{},"directed":true,"weight":{}}}}}}}"#,
                id, source, target, weight
            ))
            .send()
            .await;
    }
}
