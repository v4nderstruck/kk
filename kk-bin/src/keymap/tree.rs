use std::{collections::HashMap, hash::Hash, sync::Arc};

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
    commands: Vec<KCommand>,
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
    pub fn new_with_commands(key: KeyInputTypes, command: Vec<KCommand>) -> Self {
        Self {
            key,
            commands: command,
        }
    }
}

pub type ArcKeymapTree = Arc<KeymapTree>;
#[derive(Debug, Clone)]
pub struct KeymapTree {
    nodes: HashMap<KeymapNode, ArcKeymapTree>,
}

impl KeymapTree {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn insert_single(&mut self, node: KeymapNode) {
        self.nodes
            .insert(node, Arc::new(KeymapTree::new()));
    }

    /// Inserst with commands inserted on the last node
    /// todo: performance??
    pub fn insert_chain(&mut self, keys: Vec<KeyInputTypes>, commands: Vec<KCommand>) {
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
                    search_tree = Arc::make_mut(tree);
                }
                false =>  {
                    // does not exist, insert
                    search_tree.insert_single(node.clone());
                    let tree = search_tree.nodes.get_mut(&node).unwrap();
                    search_tree = Arc::make_mut(tree);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use std::{str::FromStr, sync::Arc};

    use arc_swap::ArcSwapAny;

    use crate::keymap::input::KeyInput;

    use super::{KeymapTree, KeymapNode, KeyInputTypes};

    #[test]
    fn insert_single() {
        let mut k = KeymapTree::new();
        let n = KeymapNode::new(super::KeyInputTypes::MATCH_ALL);
        k.insert_single(n);

        assert!(k.nodes.contains_key(&KeymapNode::new(super::KeyInputTypes::MATCH_ALL)));
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

        let spaced = k.nodes.get(&KeymapNode::new(space)).unwrap();
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

        let spaced = k.nodes.get(&KeymapNode::new(space)).unwrap();
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

        let spaced = k.nodes.get(&KeymapNode::new(space.clone())).unwrap();
        let space_keys = spaced.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(space_keys.len(), 1);
        assert!(space_keys.contains(&KeymapNode::new(a.clone())));

        let ad = spaced.nodes.get(&KeymapNode::new(a.clone())).unwrap();
        let ad_keys = ad.nodes.keys().cloned().collect::<Vec<_>>();
        assert_eq!(ad_keys.len(), 1);
        assert!(ad_keys.contains(&KeymapNode::new(b.clone())));
    }

}
