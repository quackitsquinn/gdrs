use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    io::Write,
};

use xml::writer::XmlEvent;
#[derive(Debug, PartialEq, Clone)]
pub struct UnknownField {
    pub key: String,
    pub value: String,
    pub ty: String,
}

impl UnknownField {
    pub fn new(key: String, value: String, ty: String) -> Self {
        Self { key, value, ty }
    }
}

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
    pub extra: Vec<UnknownField>, // ty, value
}

impl RawLevel {
    /// Creates a new `RawLevel` with all fields set to `None`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets a key-value pair on the level. This is used to set the fields of the level.
    pub fn key_value(&mut self, key: &str, value: String, ty: &str) {
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
            _ => {
                self.extra
                    .push(UnknownField::new(key.to_string(), value, ty.to_string()));
            }
        }
    }
    pub fn write_xml<T>(&self, w: &mut xml::EventWriter<T>)
    where
        T: Write,
    {
        macro_rules! write_field {
            ($key: expr, $xml_type: expr, $value: expr) => {
                if let Some(value) = $value.as_ref() {
                    let key = XmlEvent::start_element("k");
                    let txt = XmlEvent::characters($key);
                    let end = XmlEvent::end_element();
                    w.write(key).unwrap();
                    w.write(txt).unwrap();
                    w.write(end).unwrap();
                    if $xml_type == "bool" {
                        // Either write <T/> or <F/> </ />
                        if value == &"t" {
                            let val = XmlEvent::start_element("t");
                            let end = XmlEvent::end_element();
                            w.write(val).unwrap();
                            w.write(end).unwrap();
                        } else {
                            let val = XmlEvent::start_element("f");
                            let end = XmlEvent::end_element();
                            w.write(val).unwrap();
                            w.write(end).unwrap();
                        }
                    } else {
                        let val = XmlEvent::start_element($xml_type);
                        let txt = XmlEvent::characters(&value);
                        let end = XmlEvent::end_element();
                        w.write(val).unwrap();
                        w.write(txt).unwrap();
                        w.write(end).unwrap();
                    }
                }
            };

            // Allow for chaining
            ($key: expr, $xml_type: expr, $value: expr, $($rest: tt)*) => {
                write_field!($key, $xml_type, $value);
                write_field!($($rest)*);
            };
        }

        write_field!(
            "k1",
            "i",
            self.level_id,
            "k2",
            "s",
            self.level_name,
            "k3",
            "s",
            self.description_b64,
            "k4",
            "s",
            self.level_string,
            "k5",
            "s",
            self.creator,
            "k6",
            "s",
            self.user_id,
            "k8",
            "s",
            self.song_id,
            "k18",
            "s",
            self.attempts,
            "k19",
            "s",
            self.normal_mode,
            "k20",
            "s",
            self.practice_mode,
            "k42",
            "s",
            self.original,
            "k43",
            "s",
            self.two_player
        );

        for field in &self.extra {
            // Skip the recent tab field. This is an array that the parser does not handle, and afaik is not used.
            if field.key == "kI6" {
                continue;
            }
            write_field!(&field.key, field.ty.as_str(), Some(field.value.as_str()));
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
            .field("extras ", &self.extra)
            .finish()
    }
}
