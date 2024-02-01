use std::collections::HashMap;

use super::node;



/// A tree of nodes that can be used to represent data in a tree structure
/// Used to load the GD save data.
pub struct Node<T> {
    /// The name of the node
    pub name: String,
    /// The value of the node
    pub value: Option<T>,
    /// The children of the node
    pub children: HashMap<String, Node<T>>,
}

impl<T> Node<T> {
    /// Create a new node with the given name. This node will have no value and no children.
    pub fn new_empty(name: String) -> Node<T> {
        Node {
            name,
            value: None,
            children: HashMap::new(),
        }
    }

    /// Create a new node with the given name and value. This node will have no children.
    pub fn new_value(name: String, value: T) -> Node<T> {
        Node {
            name,
            value: Some(value),
            children: HashMap::new(),
        }
    }
    /// Create a child to this node with the given name and value. This node will have no children.
    /// The child will be returned.
    pub fn add_child(&mut self, name: String) -> &Node<T> {
        let node = Node::new_empty(name);
        self.children.entry(name).or_insert(node)
    }

    /// Create a child to this node with the given name and value. This node will have no children.
    /// The child will be returned as mutable.
    pub fn add_child_mut(&mut self, name: String) -> &mut Node<T> {
        let node = Node::new_empty(name);
        self.children.entry(name).or_insert(node)
    }
    /// Returns a reference to the child with the given name, if it exists.
    pub fn get_child(&self, name: &str) -> Option<&Node<T>> {
        self.children.get(name)
    }
    /// Returns a mutable reference to the child with the given name, if it exists.
    pub fn get_child_mut(&mut self, name: &str) -> Option<&mut Node<T>> {
        self.children.get_mut(name)
    }
}