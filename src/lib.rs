use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub sender: User,
    pub recipient: Room,
    pub time_sent: DateTime<Utc>,
    pub time_recieved: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Room {
    pub id: String,
    pub title: String,
    pub description: String,
    pub time_created: DateTime<Utc>,
    pub members: Vec<String>,
    pub messages: Vec<String>,
}

/// A user for chats, mainly used for authentication
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeySet {
    pub private: String,
    pub public: String
}

pub struct Platform {
    pub id: String,
}