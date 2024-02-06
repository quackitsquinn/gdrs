
/// An unparsed XML node.
#[derive(Debug, Clone)]
pub enum RawXmlNode {
    /// An integer node.
    Integer(String),
    /// A float node.
    Float(String),
    /// A string node.
    String(String),
    /// A boolean node.
    Boolean(String),
    /// An integer array node. This is stored as a vector because of how the array is stored in the save.
    /// This will make it easier to deal with through fastxml.
    IntArray(Vec<i32>),
}
/// An error that can occur while parsing a node.
#[derive(thiserror::Error, Debug)]
#[error("Failed to parse node")]
pub struct UnknownTypeError(String);

impl RawXmlNode {
    /// Creates a new raw xml node from a key id and a value.
    pub fn new(key_id: &str, value: &str) -> Result<Self, UnknownTypeError> {
        match key_id {
            "i" => Ok(RawXmlNode::Integer(value.to_string())),
            "r" => Ok(RawXmlNode::Float(value.to_string())),
            "s" => Ok(RawXmlNode::String(value.to_string())),
            "t" => Ok(RawXmlNode::Boolean("true".to_string())),
            "f" => Ok(RawXmlNode::Boolean("false".to_string())),
            _ => Err(UnknownTypeError(key_id.to_string())),
        }
    }

    pub fn new_arr(value: Vec<i32>) -> Self {
        Self::IntArray(value)
    }
}