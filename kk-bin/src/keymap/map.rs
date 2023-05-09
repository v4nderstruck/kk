use std::{
    collections::HashMap,
    sync::{Arc},
};

use kk_core::DocumentMode;

use crate::commands::KCommand;

use super::{
    input::KeyInput,
    tree::{ArcKeymapTree, KeyInputTypes, KeymapNode, KeymapTree},
};

#[derive(Debug)]
pub struct Keymap {
    active_mode: DocumentMode,
    state: Option<Arc<KeymapTree>>,
    maps: HashMap<DocumentMode, ArcKeymapTree>,
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            active_mode: DocumentMode::Normal,
            state: None,
            maps: HashMap::new(),
        }
    }

    pub fn load_keymap_tree(&mut self, doc_mod: DocumentMode, tree: ArcKeymapTree) {
        self.maps.insert(doc_mod, tree);
    }

    pub fn merge(&mut self, _map: HashMap<DocumentMode, ArcKeymapTree>) {
        todo!()
    }

    pub fn get(&mut self, key: KeyInput) -> Vec<&'static KCommand> {
        let key_node = KeymapNode::new(KeyInputTypes::MATCH(key));
        let all_node = KeymapNode::new(KeyInputTypes::MATCH_ALL);
        let none_node = KeymapNode::new(KeyInputTypes::MATCH_NONE);
        if self.state.is_some() {
            let state = self.state.as_ref().unwrap().clone();
            let cmds = match state.get_fun(&key_node) {
                Some((c, new_state)) => {
                    match new_state {
                        Some(s) => self.state = Some(s),
                        None => self.state = None,
                    }
                    c
                }
                None => match state.get_fun(&all_node) {
                    Some((c, _new_state)) => {
                        self.state = None;
                        c
                    }
                    None => match state.get_fun(&none_node) {
                        Some((c, _new_state)) => {
                            self.state = None;
                            c
                        }
                        None => vec![],
                    },
                },
            };
            cmds
        } else {
            let mode_tree = self.maps.get(&self.active_mode).unwrap();
            let cmds = match mode_tree.get_fun(&key_node) {
                Some((c, new_state)) => {
                    match new_state {
                        Some(s) => self.state = Some(s),
                        None => self.state = None,
                    }
                    c
                }
                None => match mode_tree.get_fun(&all_node) {
                    Some((c, _new_state)) => {
                        self.state = None;
                        c
                    }
                    None => match mode_tree.get_fun(&none_node) {
                        Some((c, _new_state)) => {
                            self.state = None;
                            c
                        }
                        None => vec![],
                    },
                },
            };
            cmds
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use kk_core::DocumentMode;

    use crate::{
        commands::KCommand,
        keymap::{
            input::KeyInput,
            tree::{KeyInputTypes, KeymapNode, KeymapTree},
        },
    };

    use super::Keymap;

    fn setup() -> KeymapTree {
        let mut k = KeymapTree::new();
        let space = KeyInputTypes::MATCH(KeyInput::from_str("space").unwrap());
        let a = KeyInputTypes::MATCH(KeyInput::from_str("a").unwrap());
        let b = KeyInputTypes::MATCH(KeyInput::from_str("b").unwrap());
        let c = KeyInputTypes::MATCH(KeyInput::from_str("c").unwrap());
        let d = KeyInputTypes::MATCH(KeyInput::from_str("d").unwrap());

        k.insert_chain(vec![space.clone(), a.clone()], vec![&KCommand::escape]);
        k.insert_chain(vec![space.clone(), b.clone()], vec![&KCommand::nop]);
        k.insert_chain(vec![c.clone()], vec![&KCommand::nop]);
        k.insert_chain(vec![d.clone()], vec![&KCommand::nop]);
        k.insert_single(KeymapNode::new_with_commands(
            KeyInputTypes::MATCH_ALL,
            vec![&KCommand::error],
        ));
        k
    }

    #[test]
    fn touch_one_key_success() {
        let mut keymap = Keymap::new();
        keymap.load_keymap_tree(DocumentMode::Normal, Arc::new(setup()));
        let cmds = keymap.get(KeyInput::from_str("c").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "nop");
    }

    #[test]
    fn touch_one_key_notmapped() {
        let mut keymap = Keymap::new();
        keymap.load_keymap_tree(DocumentMode::Normal, Arc::new(setup()));
        let cmds = keymap.get(KeyInput::from_str("9").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "error");
    }

    #[test]
    fn touch_key_combination_success() {
        let mut keymap = Keymap::new();
        keymap.load_keymap_tree(DocumentMode::Normal, Arc::new(setup()));
        let cmds = keymap.get(KeyInput::from_str("space").unwrap());
        assert_eq!(cmds.len(), 0);
        let cmds = keymap.get(KeyInput::from_str("a").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "escape");
    }

    #[test]
    fn touch_key_independent_sequence_success() {
        let mut keymap = Keymap::new();
        keymap.load_keymap_tree(DocumentMode::Normal, Arc::new(setup()));
        let cmds = keymap.get(KeyInput::from_str("d").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "nop");
        let cmds = keymap.get(KeyInput::from_str("c").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "nop");
    }

    #[test]
    fn touch_key_independent_sequence_with_combination_success() {
        let mut keymap = Keymap::new();
        keymap.load_keymap_tree(DocumentMode::Normal, Arc::new(setup()));
        let cmds = keymap.get(KeyInput::from_str("d").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "nop");
        let cmds = keymap.get(KeyInput::from_str("space").unwrap());
        assert_eq!(cmds.len(), 0);
        let cmds = keymap.get(KeyInput::from_str("a").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "escape");
        let cmds = keymap.get(KeyInput::from_str("d").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "nop");
        let cmds = keymap.get(KeyInput::from_str("c").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "nop");
    }

    #[test]
    fn touch_key_independent_sequence_some_fail() {
        let mut keymap = Keymap::new();
        keymap.load_keymap_tree(DocumentMode::Normal, Arc::new(setup()));
        let cmds = keymap.get(KeyInput::from_str("d").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "nop");
        let cmds = keymap.get(KeyInput::from_str("0").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "error");
        let cmds = keymap.get(KeyInput::from_str("k").unwrap());
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].name, "error");
    }
}
