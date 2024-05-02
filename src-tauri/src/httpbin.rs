use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)] // allow the structure to be used as an object after request
pub struct Dustbin {
    pub origin: String
}