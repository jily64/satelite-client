


pub mod config_manager {
    use base64::{Engine as _, engine::{general_purpose}};
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, fs::OpenOptions, io::Write};
    use std::fs::File;
    
    use serde_json;
    
    use crate::structs;
    use structs::structs::{ClintConfig};

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

    pub fn save_cfg(cfg_strings: &Vec<&str>, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Здесь использованы HashMaps так как здесь не требует создавать отдельную структуру для хранения сией шняги.

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        let mut cfgs: HashMap<String, String> = HashMap::new();
        for i in 0..cfg_strings.len() {
            cfgs.insert(format!("cfg_{}", i), cfg_strings[i].to_string());
        }

        let json = serde_json::to_string(&cfgs)?;
        writeln!(&file, "{}", json)?;
        Ok(())
    }
}