
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
    /// An integer array node. This is stored as a vector because of how the array is stored in the save.
    /// This will make it easier to deal with through fastxml.
    IntArray(Vec<i32>),
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
            _ => panic!("Unknown key id: {}", key_id),
        }
    }

    pub fn new_arr(value: Vec<i32>) -> Self {
        Self::IntArray(value)
    }
}