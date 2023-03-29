use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct YewduxStore {
    pub username: String,
    pub password: String,
    pub token: String,
}