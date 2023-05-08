use std::{collections::HashMap, sync::Arc};

use kk_core::DocumentMode;

use crate::commands::Command;

use super::input::KeyInput;

/// used for matching
pub enum KeymapNodeInputType {
    Skip,
    Key(KeyInput),
}
pub struct KeymapNode {
    key: KeymapNodeInputType,
    commands: Vec<Command>,
}
type ArcKeymapNode = Arc<KeymapNode>;

pub enum KeymapTreeNodeType {
    Subtree(KeymapTree),
    Leaf(ArcKeymapNode),
}
pub struct KeymapTree {
    node: ArcKeymapNode,
    children: Vec<KeymapTreeNodeType>,
}
pub struct Keymap {
    mode: DocumentMode,
    map: HashMap<DocumentMode, KeymapTree>,
}

impl KeymapNode {
    pub fn new_root() -> Self {
        Self {
            key: KeymapNodeInputType::Skip,
            commands: vec![],
        }
    }
    pub fn new_node(key: KeyInput) -> Self {
        Self {
            key: KeymapNodeInputType::Key(key),
            commands: vec![],
        }
    }
    pub fn run_cmds(&self) -> anyhow::Result<()> {
        for cmd in &self.commands {
            cmd.exec()?
        }
        Ok(())
    }
}

impl KeymapTree {
    pub fn new_root() -> Self {
        Self {
            node: Arc::new(KeymapNode::new_root()),
            children: Vec::new(),
        }
    }
    /// assumes that nodes are sorted in order of the input being matched
    pub fn add_node(&mut self, node: KeymapTreeNodeType) {
        self.children.push(node)
    }
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            mode: DocumentMode::Normal,
            map: HashMap::from([
                (DocumentMode::Normal, KeymapTree::new_root()),
                (DocumentMode::Insert, KeymapTree::new_root())
            ]),
        }
    }
    pub fn insert_mapping(
        &mut self,
        doc_mode: DocumentMode,
        nodes: Vec<ArcKeymapNode>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use crate::{commands::Command, keymap::input::KeyInput};

    use super::KeymapNode;

    #[test]
    fn node_creation_eq() {
        let n = KeymapNode::new_root();
        let l = KeymapNode::new_node(KeyInput::from_str("a").unwrap());
        assert!(n.run_cmds().is_ok());
        assert!(l.run_cmds().is_ok());
    }

    #[test]
    fn tree_creation_eq() {
    }
}
