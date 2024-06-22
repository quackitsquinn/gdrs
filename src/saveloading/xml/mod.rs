mod rawlevel;

use log::trace;
pub use rawlevel::RawLevel;
use xml::{reader::XmlEvent, EventReader};

#[derive(thiserror::Error, Debug)]
pub enum XmlParseError {
    #[error("An error occurred while parsing the XML: {0}")]
    XmlError(#[from] xml::reader::Error),
}

const LEVEL_DATA_DEPTH: usize = 5;
const LEVEL_END_DEPTH: usize = 3;

#[derive(Debug, PartialEq)]
enum ParseState {
    Begin,
    InKey,
    OutKey,
    InValue,
    SeekingKey,
    ParsedLevel,
}

pub fn parse_xml_to_level_list(xml: &str) -> Result<Vec<RawLevel>, XmlParseError> {
    let mut parser = EventReader::from_str(xml);
    let mut levels: Vec<RawLevel> = Vec::new();
    let mut current_level = RawLevel::new();
    let mut current_depth = 0;
    let mut current_key = String::new();
    let mut state = ParseState::Begin;

    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                current_depth += 1;
                trace!(
                    "state: {:?}, name: {:?}, depth: {}",
                    state,
                    name.local_name,
                    current_depth
                );

                if name.local_name == "k" && current_depth == LEVEL_DATA_DEPTH {
                    state = ParseState::InKey;
                    continue;
                }
                // Booleans are handled differently then everything else.
                if state == ParseState::OutKey {
                    if name.local_name == "t" {
                        current_level.key_value(&current_key, "t".to_string());
                        state = ParseState::SeekingKey;
                    } else if name.local_name == "f" {
                        current_level.key_value(&current_key, "f".to_string());
                        state = ParseState::SeekingKey;
                    } else if ["i", "s", "r"].contains(&name.local_name.as_str()) {
                        state = ParseState::InValue;
                    }
                    continue;
                }
            }
            XmlEvent::EndElement { name } => {
                current_depth -= 1;
                trace!(
                    "state: {:?}, name: {:?}, depth: {}",
                    state,
                    name.local_name,
                    current_depth
                );

                if state != ParseState::Begin
                    && state != ParseState::ParsedLevel
                    && current_depth == LEVEL_END_DEPTH
                {
                    trace!("Adding level: {:?}", current_level);
                    levels.push(current_level);
                    current_level = RawLevel::new();
                    state = ParseState::ParsedLevel;
                }
                if name.local_name == "k" && state == ParseState::InKey {
                    state = ParseState::OutKey;
                }
            }
            XmlEvent::Characters(data) => {
                trace!(
                    "state: {:?}, data: {}",
                    state,
                    &data[0..100.min(data.len())]
                );
                if state == ParseState::InKey {
                    current_key = data;
                } else if state == ParseState::InValue {
                    trace!("key: {}, valuelen: {}", current_key, data.len());
                    current_level.key_value(&current_key, data);
                    state = ParseState::SeekingKey;
                }
            }
            _ => {}
        }
    }

    Ok(levels)
}
#[cfg(test)]
mod tests {
    use log::info;

    use crate::setup_logging;

    use super::*;

    #[test]
    fn test_parse_xml_to_level_list() {
        setup_logging!("xml_parse_test");
        let xml = include_str!("../../../res/CCLocalLevels.dat.xml");
        let levels = parse_xml_to_level_list(xml);
        assert!(
            levels.is_ok(),
            "Failed to parse XML: {}",
            levels.err().unwrap()
        );
        for level in levels.unwrap() {
            info!("Level: {:?}", level);
        }
    }
}
