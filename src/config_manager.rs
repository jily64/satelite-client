pub mod config_manager {
    pub async fn get_cfg(url: &String) -> Option<String> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await;
        
        match response {
            Ok(resp) => Some(resp.text().await.unwrap_or_else(|_| String::from("Failed to read response text"))),
            Err(_) => None,
        }
    }
    pub fn decode_base64(encoded: &str) -> Option<String> {
        let engine = base64::Engine::new(base64::engine::general_purpose::STANDARD);
        match base64::Engine::decode(encoded) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(decoded_string) => Some(decoded_string),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}