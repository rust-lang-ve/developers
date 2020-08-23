use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Developer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub linked_in: Option<String>,
    pub github: Option<String>,
    pub gitlab: Option<String>,
    pub reddit: Option<String>,
    pub location: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
}
