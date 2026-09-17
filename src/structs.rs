


pub mod structs {
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    pub struct ConfigPayload {
        pub url: String,
    }
}