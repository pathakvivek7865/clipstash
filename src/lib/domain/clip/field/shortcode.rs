use crate::domain::clip::ClipError;
use derive_more::From;
use rocket::{UriDisplayPath, UriDisplayQuery};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, From, UriDisplayPath, UriDisplayQuery,
)]
pub struct Shortcode(String);

impl Shortcode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = ['a', 'b', 'c', 'd', '1', '2', '3', '4'];

        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling arrays should have values"),
            )
        }

        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for Shortcode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Shortcode> for String {
    fn from(value: Shortcode) -> Self {
        value.0
    }
}

impl From<&str> for Shortcode {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl FromStr for Shortcode {
    type Err = ClipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

use rocket::request::FromParam;
impl<'r> FromParam<'r> for Shortcode {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Self::from(param))
    }
}
