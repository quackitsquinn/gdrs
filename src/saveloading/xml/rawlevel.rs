use std::fmt::{self, Formatter};

/// A raw level. This has no processing done on it, other than being extracted from the XML.
pub struct RawLevel {
    /// The id of the level.
    /// id: k1
    pub level_id: Option<String>,
    /// id: k2
    pub level_name: Option<String>,
    /// id: k3
    pub description_b64: Option<String>,
    /// id: k4
    /// The inner level string. Typically *huge*.
    pub level_string: Option<String>,
    /// id: k5
    pub creator: Option<String>,
    /// id: k6
    pub user_id: Option<String>,
    /// id: k8
    /// The official song id. Will be `None` if the song is custom.
    pub song_id: Option<String>,
}

impl RawLevel {
    /// Creates a new `RawLevel` with all fields set to `None`.
    pub fn new() -> Self {
        Self {
            level_id: None,
            level_name: None,
            description_b64: None,
            level_string: None,
            creator: None,
            user_id: None,
            song_id: None,
        }
    }
    /// Sets a key-value pair on the level. This is used to set the fields of the level.
    pub fn key_value(&mut self, key: &str, value: String) {
        match key {
            "k1" => self.level_id = Some(value),
            "k2" => self.level_name = Some(value),
            "k3" => self.description_b64 = Some(value),
            "k4" => self.level_string = Some(value),
            "k5" => self.creator = Some(value),
            "k6" => self.user_id = Some(value),
            "k8" => self.song_id = Some(value),
            _ => log::warn!("Unknown key: {}", key),
        }
    }
}

impl fmt::Debug for RawLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawLevel")
            .field("level_id", &self.level_id)
            .field("level_name", &self.level_name)
            .field("description_b64", &self.description_b64)
            .field(
                "level_string.len",
                &self.level_string.as_ref().map(|s| s.len()),
            )
            .field("creator", &self.creator)
            .field("user_id", &self.user_id)
            .field("song_id", &self.song_id)
            .finish()
    }
}
