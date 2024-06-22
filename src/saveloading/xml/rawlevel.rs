use std::fmt::{self, Formatter};

/// A raw level. This has no processing done on it, other than being extracted from the XML.
///
/// This struct does not contain all of the fields that a level can have. Most are either unused or easy to assume.
#[derive(Default, PartialEq, Clone)]
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
    /// id: k18
    /// Attempt count.
    pub attempts: Option<String>,
    // id: k19
    pub normal_mode: Option<String>,
    // id: k20
    pub practice_mode: Option<String>,
    // id: k42
    pub original: Option<String>,
    // id: k43
    // TODO: See if this is still used in 2.2
    pub two_player: Option<String>,
}

impl RawLevel {
    /// Creates a new `RawLevel` with all fields set to `None`.
    pub fn new() -> Self {
        Self::default()
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
            "k18" => self.attempts = Some(value),
            "k19" => self.normal_mode = Some(value),
            "k20" => self.practice_mode = Some(value),
            "k42" => self.original = Some(value),
            "k43" => self.two_player = Some(value),
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
            .field("attempts", &self.attempts)
            .field("normal_mode", &self.normal_mode)
            .field("practice_mode", &self.practice_mode)
            .field("original", &self.original)
            .field("two_player", &self.two_player)
            .finish()
    }
}
