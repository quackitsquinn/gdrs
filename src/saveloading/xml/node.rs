use std::error::Error;

use crate::saveloading::xml::raw_node::RawXmlNode;

pub enum XmlNode {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    IntArray(Vec<i32>),
}

impl XmlNode {
    pub fn get_integer(&self) -> Option<i32> {
        match self {
            XmlNode::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_float(&self) -> Option<f32> {
        match self {
            XmlNode::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        match self {
            XmlNode::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            XmlNode::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get_int_array(&self) -> Option<&Vec<i32>> {
        match self {
            XmlNode::IntArray(a) => Some(a),
            _ => None,
        }
    }
}

/// An error that can occur while parsing a node.
/// This is a wrapper around any error that can occur while parsing a node.
// TODO: make this a enum with a variant for each error.
#[derive(thiserror::Error, Debug)]
#[error("Failed to parse node")]
pub struct ParseError(#[from] Box<dyn Error>);

impl TryFrom<RawXmlNode> for XmlNode {
    type Error = ParseError;

    fn try_from(value: RawXmlNode) -> Result<Self, Self::Error> {
        /// Adds an arm to the match statement that parses a node.
        macro_rules! parse {
            ($node:ident, $type:ty, $node_name:ident) => {
                match $node.parse::<$type>() {
                    Ok(v) => Ok(XmlNode::$node_name(v)),
                    Err(e) => Err(ParseError(Box::new(e))),
                }
            };
        }
        match value {
            RawXmlNode::Integer(i) => parse!(i, i32, Integer),
            RawXmlNode::Float(f) => parse!(f, f32, Float),
            RawXmlNode::String(s) => Ok(XmlNode::String(s)),
            RawXmlNode::Boolean(b) => parse!(b, bool, Boolean),
            RawXmlNode::IntArray(a) => Ok(XmlNode::IntArray(a)),
        }
    }
}
