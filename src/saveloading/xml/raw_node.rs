
/// An unparsed XML node.
pub enum RawXmlNode {
    /// An integer node.
    Integer(String),
    /// A float node.
    Float(String),
    /// A string node.
    String(String),
    /// A boolean node.
    Boolean(String),
    /// An integer array node.
    IntArray(String),
}

impl RawXmlNode {
    /// Creates a new raw xml node from a key id and a value.
    pub fn new(key_id: &str, value: &str) -> Self {
        match key_id {
            "i" => RawXmlNode::Integer(value.to_string()),
            "r" => RawXmlNode::Float(value.to_string()),
            "s" => RawXmlNode::String(value.to_string()),
            "t" => RawXmlNode::Boolean("true".to_string()),
            "f" => RawXmlNode::Boolean("false".to_string()),
            // For some reason, int arrays are stored as dicts. I don't know why.
            "d" => RawXmlNode::IntArray(value.to_string()),
            _ => panic!("Unknown key id: {}", key_id),
        }
    }
}