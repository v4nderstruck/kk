use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Weak},
};

use arc_swap::ArcSwapAny;

use crate::commands::KCommand;

use super::input::KeyInput;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyInputTypes {
    MATCH_NONE,      // does not match any key, used for root node, should match last
    MATCH(KeyInput), // match exactly one key, should match first
    MATCH_ALL,       // matches all keys, should match second
}

#[derive(Debug, Clone)]
pub struct KeymapNode {
    key: KeyInputTypes,
    commands: Vec<&'static KCommand>,
}

impl PartialEq for KeymapNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for KeymapNode {}

impl Hash for KeymapNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

impl KeymapNode {
    pub fn new(key: KeyInputTypes) -> Self {
        Self {
            key,
            commands: Vec::new(),
        }
    }
    pub fn new_with_commands(key: KeyInputTypes, command: Vec<&'static KCommand>) -> Self {
        Self {
            key,
            commands: command,
        }
    }
    pub fn get_cmds(&self) -> Vec<&'static KCommand> {
        self.commands.to_owned()
    }
}

pub type ArcKeymapTree = Arc<KeymapTree>;
#[derive(Debug, Clone)]
pub struct KeymapTree {
    pub nodes: HashMap<KeymapNode, Option<ArcKeymapTree>>,
}

impl KeymapTree {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// looks into the keys in that node, will find the match and return the commads as well as the
    /// next subtree as Option with None indicating its a leaf
    pub fn get_fun(
        &self,
        node: &KeymapNode,
    ) -> Option<(Vec<&'static KCommand>, Option<ArcKeymapTree>)> {
        match self.nodes.get_key_value(node) {
            Some((key, v)) => Some((key.get_cmds(), v.clone())),
            None => None,
        }
    }

    pub fn insert_single(&mut self, node: KeymapNode) {
        self.nodes.insert(node, None);
    }

    /// Inserst with commands inserted on the last node
    /// todo: performance due to make_mut??
    pub fn insert_chain(&mut self, keys: Vec<KeyInputTypes>, commands: Vec<&'static KCommand>) {
        let last_item = keys.len() - 1;
        let mut search_tree = self;
        for (i, key) in keys.into_iter().enumerate() {
            // node to be inserted
            let node = if i == last_item {
                KeymapNode::new_with_commands(key, commands.clone()) // todo: moved in loop bs
            } else {
                KeymapNode::new(key)
            };
            // check whether node already exists
            match search_tree.nodes.contains_key(&node) {
                true => {
                    // exists, go down the tree
                    let tree = search_tree.nodes.get_mut(&node).unwrap();
                    match tree {
                        Some(subtree) => search_tree = Arc::make_mut(subtree),
                        None => {
                            if i != last_item {
                                tree.replace(Arc::new(KeymapTree::new()));
                                search_tree = Arc::make_mut(tree.as_mut().unwrap());
                            } else {
                                break;
                            }
                        }
                    }
                }
                false => {
                    // does not exist, insert
                    search_tree.insert_single(node.clone());
                    let tree = search_tree.nodes.get_mut(&node).unwrap();
                    if i != last_item {
                        tree.replace(Arc::new(KeymapTree::new()));
                        search_tree = Arc::make_mut(tree.as_mut().unwrap());
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::keymap::input::KeyInput;

    use super::{KeyInputTypes, KeymapNode, KeymapTree};

    #[test]
    fn insert_single() {
        let mut k = KeymapTree::new();
        let n = KeymapNode::new(super::KeyInputTypes::MATCH_ALL);
        k.insert_single(n);

        assert!(k
            .nodes
            .contains_key(&KeymapNode::new(super::KeyInputTypes::MATCH_ALL)));
    }
    #[test]
    fn insert_chain() {
        let mut k = KeymapTree::new();
        let space = KeyInputTypes::MATCH(KeyInput::from_str("space").unwrap());
        let a = KeyInputTypes::MATCH(KeyInput::from_str("a").unwrap());
        let b = KeyInputTypes::MATCH(KeyInput::from_str("b").unwrap());
        let c = KeyInputTypes::MATCH(KeyInput::from_str("c").unwrap());

        k.insert_chain(vec![space.clone(), a.clone()], vec![]);
        k.insert_chain(vec![space.clone(), b.clone()], vec![]);
        k.insert_chain(vec![c.clone()], vec![]);

        let keys = k.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&KeymapNode::new(space.clone())));
        assert!(keys.contains(&KeymapNode::new(c)));

        let spaced = k
            .nodes
            .get(&KeymapNode::new(space))
            .unwrap()
            .as_ref()
            .unwrap();
        let space_keys = spaced.nodes.keys().cloned().collect::<Vec<_>>();

        assert_eq!(space_keys.len(), 2);
        assert!(space_keys.contains(&KeymapNode::new(a.clone())));
        assert!(space_keys.contains(&KeymapNode::new(b.clone())));
    }

    #[test]
    fn insert_chain_twice() {
        let mut k = KeymapTree::new();
        let space = KeyInputTypes::MATCH(KeyInput::from_str("space").unwrap());
        let a = KeyInputTypes::MATCH(KeyInput::from_str("a").unwrap());

        k.insert_chain(vec![space.clone(), a.clone()], vec![]);
        k.insert_chain(vec![space.clone(), a.clone()], vec![]);

        let keys = k.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&KeymapNode::new(space.clone())));

        let spaced = k
            .nodes
            .get(&KeymapNode::new(space))
            .unwrap()
            .as_ref()
            .unwrap();
        let space_keys = spaced.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(space_keys.len(), 1);
        assert!(space_keys.contains(&KeymapNode::new(a.clone())));
    }

    #[test]
    fn insert_chain_extend() {
        let mut k = KeymapTree::new();
        let space = KeyInputTypes::MATCH(KeyInput::from_str("space").unwrap());
        let a = KeyInputTypes::MATCH(KeyInput::from_str("a").unwrap());
        let b = KeyInputTypes::MATCH(KeyInput::from_str("b").unwrap());
        k.insert_chain(vec![space.clone(), a.clone()], vec![]);
        k.insert_chain(vec![space.clone(), a.clone(), b.clone()], vec![]);

        let keys = k.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&KeymapNode::new(space.clone())));

        let spaced = k
            .nodes
            .get(&KeymapNode::new(space.clone()))
            .unwrap()
            .as_ref()
            .unwrap();
        let space_keys = spaced.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(space_keys.len(), 1);
        assert!(space_keys.contains(&KeymapNode::new(a.clone())));

        let ad = spaced
            .nodes
            .get(&KeymapNode::new(a.clone()))
            .unwrap()
            .as_ref()
            .unwrap();
        let ad_keys = ad.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(ad_keys.len(), 1);
        assert!(ad_keys.contains(&KeymapNode::new(b.clone())));
    }
}
