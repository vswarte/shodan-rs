use std::collections::{HashMap, hash_map};

#[derive(Default)]
pub struct ParameterBag(HashMap<String, String>);

impl ParameterBag {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self.0.insert(key.as_ref().to_string(), value.into());
    }

    pub fn set_optional(&mut self, key: impl AsRef<str>, value: Option<impl Into<ParameterValue>>) {
        if let Some(value) = value {
            self.set(key.as_ref(), value.into().0);
        }
    }

    pub fn pairs(&self) -> hash_map::Iter<'_, String, String> {
        self.0.iter()
    }
}

pub struct ParameterValue(String);

impl From<u32> for ParameterValue {
    fn from(value: u32) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ParameterValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ParameterValue {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<bool> for ParameterValue {
    fn from(value: bool) -> Self {
        Self(if value { "true".into() } else { "false".into() })
    }
}
