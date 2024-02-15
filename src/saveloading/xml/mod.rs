pub mod node;
pub(super) mod raw_node;
pub mod data_tree;

use std::{collections::HashMap, pin};

pub use node::XmlNode;
use quick_xml::{events::Event, Reader};
pub use raw_node::RawXmlNode;

use self::data_tree::SaveTree;

pub fn parse_xml_tree(tree: &str) {
    let mut dtree: SaveTree<RawXmlNode> = SaveTree::new();
    let mut current_depth = 0;
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut last_name = String::new();
    let mut is_key = false;
    let mut xml_parser = Reader::from_str(tree);
    let mut buf = Vec::new();
    loop {
        match xml_parser.read_event_into(&mut buf){
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8(e.name().0.to_vec()).unwrap();
                match name.as_str() {
                    "k" => {
                        is_key = true;
                    }
                    _ => {
                        if !is_key {
                        last_name = name.clone();
                        }
                    }
                }
    
                current_depth += 1;
                current_key = name;
            }
            Ok(Event::Text(e)) => {
                current_value = String::from_utf8(e.to_vec()).unwrap();
                if is_key {
                    is_key = false;
                    let last_key = dtree.get_selected_node().unwrap();
                    let xmlnode = RawXmlNode::new(dbg!(&last_name), dbg!(&current_value));
                    if let Ok(node) = xmlnode{
                        // Add a child here...
                    } else {
                        println!("Failed to parse node: {:?}", xmlnode);
                    
                    }
                }
                
            }
            Ok(Event::End(ref e)) => {
                //let name = String::from_utf8(e.name().0.to_vec()).unwrap();
                current_depth -= 1;
            }

            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", xml_parser.buffer_position(), e),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::saveloading::xml::parse_xml_tree;

    #[test]
    fn test_parse_xml_tree() {
        let xml = include_str!("../../../res/CCLocalLevels.dat.xml");
        parse_xml_tree(xml);
    }
}