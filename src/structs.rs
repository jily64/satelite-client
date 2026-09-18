


pub mod structs {
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    pub struct ConfigPayload {
        pub url: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ClintConfig {
        pub cfg_strings: Vec<String>,
    }
}