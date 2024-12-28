use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeMetadata {
    pub id: String,
    pub challenge: ChallengeDetails,
    pub aliases: Aliases,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeDetails {
    pub title: String,
    pub category: String,
    pub difficulty: String,
    pub description: String,
    pub hint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Aliases {
    pub src: String,
    pub exploit: String,
    pub test: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub creator: String,
    pub date_created: String,
    pub version: String,
}
