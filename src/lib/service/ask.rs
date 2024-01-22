use crate::domain::clip::field::{self, Password};
use crate::Shortcode;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClip {
    pub shortcode: Shortcode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: Shortcode::from(shortcode),
            password: field::Password::default(),
        }
    }
}

impl From<Shortcode> for GetClip {
    fn from(value: Shortcode) -> Self {
        Self {
            shortcode: value,
            password: Password::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClip {
    pub shortcode: field::Shortcode,
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}
