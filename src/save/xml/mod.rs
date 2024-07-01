mod rawlevel;
mod xml_writer;

use std::error;

use log::trace;
pub use rawlevel::RawLevel;
use xml::{reader::XmlEvent, EventReader, EventWriter};

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct XmlParseError(#[from] xml::reader::Error);

const LEVEL_DATA_DEPTH: usize = 5;
const LEVEL_END_DEPTH: usize = 3;

#[derive(Debug, PartialEq)]
enum ParseState {
    Begin,
    InKey,
    OutKey,
    InValue(String),
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
                        current_level.key_value(&current_key, "t".to_string(), "bool");
                        state = ParseState::SeekingKey;
                    } else if name.local_name == "f" {
                        current_level.key_value(&current_key, "f".to_string(), "bool");
                        state = ParseState::SeekingKey;
                    } else if ["i", "s", "r"].contains(&name.local_name.as_str()) {
                        state = ParseState::InValue(name.local_name.to_string());
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
                } else if let ParseState::InValue(xml_type) = state {
                    trace!("key: {}, valuelen: {}", current_key, data.len());
                    current_level.key_value(&current_key, data, &xml_type);
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
    use std::{fs::File, io::Write};

    use log::info;
    use xml::writer::XmlEvent;

    use crate::{save::raw_loader::decode_save_bytes, setup_logging};

    use super::*;

    #[test]
    fn test_parse_xml_to_level_list() {
        setup_logging!("xml_parse_test");
        let xml = decode_save_bytes(include_bytes!("../../../res/CCLocalLevels_22.dat").to_vec())
            .unwrap();
        //include_str!("../../../res/CCLocalLevels.dat.xml");
        let levels = parse_xml_to_level_list(xml.as_str());
        assert!(
            levels.is_ok(),
            "Failed to parse XML: {}",
            levels.err().unwrap()
        );
        let mut levels = levels.unwrap();
        for level in &levels {
            info!("Level: {:#?}", level);
        }
        File::create("test.xml")
            .unwrap()
            .write_all(xml_writer::xml_from_level_list(&levels).as_bytes())
            .unwrap();
    }
}
