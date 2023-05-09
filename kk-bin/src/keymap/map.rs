use std::{collections::HashMap, sync::Arc};

use kk_core::DocumentMode;

use crate::commands::{ArcCommand, Command};

use super::input::KeyInput;

/// used for matching
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum KeymapNodeInputType {
    Skip,
    Key(KeyInput),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeymapNode {
    key: KeymapNodeInputType,
    commands: Vec<ArcCommand>,
}
type ArcKeymapNode = Arc<KeymapNode>;

impl std::hash::Hash for KeymapNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}


impl PartialEq for KeymapNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for KeymapNode {}

impl KeymapNode {
    pub fn new_root() -> Self {
        Self {
            key: KeymapNodeInputType::Skip,
            commands: vec![],
        }
    }
    pub fn new_input(key: KeyInput) -> Self {
        Self {
            key: KeymapNodeInputType::Key(key),
            commands: vec![],
        }
    }
    pub fn push_cmd(&mut self, cmd: Arc<Command>) {
        self.commands.push(cmd)
    }
    pub fn run_cmds(&self) -> anyhow::Result<()> {
        for cmd in &self.commands {
            cmd.exec()?
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeymapTree {
    map: HashMap<ArcKeymapNode, ArcKeymapTree>,
}
type ArcKeymapTree = Arc<KeymapTree>;
impl KeymapTree {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// assumes that nodes are sorted in order of the input being matched
    pub fn add_node(&mut self, node: ArcKeymapNode) {
        self.map.insert(node, Arc::new(KeymapTree::new()));
    }

    pub fn has_key_map(&self, key: &KeyInput) -> bool { 
        if self.map.contains_key()
    }
}

#[derive(Debug, Clone)]
pub struct Keymap {
    mode: DocumentMode,
    map: HashMap<DocumentMode, ArcKeymapTree>,
}
impl Keymap {
    pub fn new() -> Self {
        Self {
            mode: DocumentMode::Normal,
            map: HashMap::from([
                (DocumentMode::Normal, Arc::new(KeymapTree::new())),
                (DocumentMode::Insert, Arc::new(KeymapTree::new())),
            ]),
        }
    }

    // peeks and checks whether a sequence of keymapping exists
    pub fn map_exits(&self, doc_mode: DocumentMode, keys: &[KeyInput]) -> bool {
        let tree = self.map.get(&doc_mode).unwrap().clone();
        for key in keys {
        }
        true
    }
    /// takes the document mode and the KeymapNodes to insert,
    /// todo: arc cloning does not feel natural
    pub fn insert_mapping(
        &mut self,
        doc_mode: DocumentMode,
        nodes: Vec<ArcKeymapNode>,
    ) -> anyhow::Result<()> {
        if let Some(root_tree) = self.map.get_mut(&doc_mode) {
            let mut current_tree_ptr = root_tree;
            let last_element = nodes.len() - 1;
            for (i, node) in nodes.into_iter().enumerate() {
                // we have already a command bound to this key, continue down the tree
                if let Some(pos) = current_tree_ptr
                    .children
                    .iter()
                    .position(|e| e.node == node)
                {
                    // should continue down the tree
                    {
                        let tree_node = Arc::make_mut(current_tree_ptr);
                        current_tree_ptr = tree_node.children.get_mut(pos).unwrap();
                    }
                    // last item, we should push the commands to the tree node
                    if i == last_element {
                        let tree_node = Arc::make_mut(&mut current_tree_ptr);
                        let child_node = Arc::make_mut(&mut tree_node.node);
                        for cmd in &node.commands {
                            child_node.push_cmd(cmd.clone())
                        }
                    }
                } else {
                    // node does not exits yet, insert it
                    let tree_node = Arc::make_mut(current_tree_ptr);
                    tree_node
                        .children
                        .push(Arc::new(KeymapTree::new_node(node)));
                    current_tree_ptr = tree_node.children.last_mut().unwrap();
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use crate::{commands::Command, keymap::input::KeyInput};

    use super::{Keymap, KeymapNode};

    #[test]
    fn node_creation_eq() {
        let n = KeymapNode::new_root();
        let l = KeymapNode::new_input(KeyInput::from_str("a").unwrap());
        assert!(n.run_cmds().is_ok());
        assert!(l.run_cmds().is_ok());
    }

    #[test]
    fn insert_keymaps_eq() {
        let mut keymap = Keymap::new();
        let space = Arc::new(KeymapNode::new_input(KeyInput::from_str("space").unwrap()));
        let a = Arc::new(KeymapNode::new_input(KeyInput::from_str("a").unwrap()));
        let b = Arc::new(KeymapNode::new_input(KeyInput::from_str("b").unwrap()));
        let c = Arc::new(KeymapNode::new_input(KeyInput::from_str("c").unwrap()));
        let vec_space_a = vec![space, a];
        let vec_b = vec![b];
        let vec_c = vec![c];

        keymap
            .insert_mapping(kk_core::DocumentMode::Normal, vec_space_a)
            .unwrap();
        keymap
            .insert_mapping(kk_core::DocumentMode::Normal, vec_b)
            .unwrap();
        keymap
            .insert_mapping(kk_core::DocumentMode::Normal, vec_c)
            .unwrap();

        println!("{:#?}", keymap);
    }
}
