use std::{cell::RefCell, rc::Rc};

use indextree::{Arena, Node, NodeId};

use super::XmlNode;

pub struct KeyValue<T>(pub String, pub Option<T>);

/// A thin wrapper around an `Arena<XmlNode>` to represent a tree of XML nodes.
/// Contains some helper methods to make it easier to work with the tree.
///
/// This uses a selection paradigm, where a node can be selected and then children can be added to it.
/// This prevents any weirdness with rc and refcell, and makes it easier to work with the tree.
pub struct SaveTree<T> {
    /// The root of the tree. This is an `Arena` of `KeyValue<T>`.
    pub root: Arena<KeyValue<T>>,
    /// The currently selected node, if there is one.
    selected_node: Option<NodeId>,
}
impl<T> SaveTree<T> {
    pub fn new() -> Self {
        let mut arena: Arena<KeyValue<T>> = Arena::new();
        let tree = SaveTree {
            root: arena,
            selected_node: None,
        };
        tree
    }
    /// Adds a child to the currently selected node, or to the root if there is no selected node.
    /// Does not select the new node.
    pub fn add_child(&mut self, name: String, value: Option<T>, select: bool) {
        // Side note, I really like the way this is implemented. also maybe the first time I used .take()?
        if let Some(selected) = self.selected_node.take() {
            selected.append_value(KeyValue(name, value), &mut self.root);
        } else {
            if select {
                self.selected_node = Some(self.root.new_node(KeyValue(name, value)));
                return;
            }
            self.root.new_node(KeyValue(name, value));
        }
    }
    /// Adds a child to the currently selected node, or to the root if there is no selected node.
    /// Selects the new node.
    pub fn add_child_select(&mut self, name: String, value: Option<T>) {
        // Side note, I really like the way this is implemented. also maybe the first time I used .take()?
        if let Some(selected) = self.selected_node.take() {
            self.selected_node = Some(selected.append_value(KeyValue(name, value), &mut self.root));
        } else {
            self.selected_node = Some(self.root.new_node(KeyValue(name, value)));
        }
    }
    /// Selects a node, if it exists.
    /// Returns true if the node exists and was selected, false otherwise.
    pub fn select_node(&mut self, node: NodeId) -> bool {
        if self.root.get(node.clone()).is_some() {
            self.selected_node = Some(node);
            true
        } else {
            false
        }
    }

    /// Returns the selected nodeid, if there is one.
    pub fn get_selected_nodeid(&self) -> Option<NodeId> {
        self.selected_node
    }

    /// Returns the selected node, if there is one.
    pub fn get_selected_node(&self) -> Option<&Node<KeyValue<T>>> {
        if let Some(selected) = self.selected_node {
            self.root.get(selected)
        } else {
            None
        }
    }

    /// Deselects the currently selected node.
    pub fn deselect(&mut self) {
        self.selected_node = None;
    }
    /// Selects the parent of the currently selected node, if there is one.
    pub fn select_parent(&mut self) -> bool {
        if let Some(node) = self.get_selected_node() {
            if let Some(parent) = node.parent() {
                self.selected_node = Some(parent);
                return true;
            }
        }
        false
    }
    /// Returns a reference to the inner `Arena` of the tree.
    pub fn inner(&self) -> &Arena<KeyValue<T>> {
        &self.root
    }
    /// Returns a mutable reference to the inner `Arena` of the tree.
    pub fn inner_mut(&mut self) -> &mut Arena<KeyValue<T>> {
        &mut self.root
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_tree_creation() {
        let tree: super::SaveTree<u32> = super::SaveTree::new();
        assert!(tree.root.is_empty());
    }

    pub fn test_tree_add_child() {
        let mut tree: super::SaveTree<u32> = super::SaveTree::new();
        tree.add_child_select("test".to_string(), Some(5));
    }
}
