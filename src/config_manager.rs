pub mod config_manager {
    use base64::{Engine as _, engine::{general_purpose}};

    pub async fn get_cfg(url: &String) -> Option<String> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await;
        
        match response {
            Ok(resp) => Some(resp.text().await.unwrap_or_else(|_| String::from("Failed to read response text"))),
            Err(_) => None,
        }
    }
    pub fn decode_base64(encoded: &str) -> Option<String> {
        let bytes = general_purpose::STANDARD
            .decode(encoded).unwrap();

        return match String::from_utf8(bytes) {
            Ok(decoded_string) => Some(decoded_string),
            Err(_) => None,
        };
    }
}