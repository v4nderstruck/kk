# Design DOC

`keymap` is responsible for mapping keys and sequences of keys to sequences of
commands. 

-- heavily "inspired" by helix

## Changes

- 09.05.2023: Created initial Draft

## Classes

The general idea is to structure the keymapping as a tree-like structure
where you will follow down the tree for sequences of key inputs.

```mermaid
classDiagram
note for KeyInput "Represents a single KeyInput"
note for KeymapNode "Represents relationship from KeyInput to Commands (possibly
multiple)"
note for KeymapTree "Represents sequenced Keybinding capabilities. Can hold many
unique KeymapNodes and maps them the subtrees\n"
note for Keymap "Represents the entire Keymap. Can hold many KeymapTrees for
each document mode. Used in Parsing of actual Input."

KeymapNode *-- "1" KeyInput
KeymapNode *-- "*" Command
KeymapTree *-- "*" KeymapNode
KeymapTree *-- "*" KeymapTree

Keymap *-- "*" KeymapTree

class KeyInput {
    +code: Key
    +modifiers: Modifier
}

class KeymapNode {
    +key: KeyInput
    +commands: Command[]
}
class Command

class KeymapTree {
    +nodes: HashMap~KeymapNode|Option~KeymapTree~~ 
}

class Keymap {
    +active_mode: DocumentMode
    +state: Option~KeymapTree~
    +maps: HashMap~DocumentMode|KeymapTree~
}
```

## Behavior

**Construction of Keymap**

Implemented using serde. Config format in TOML.

```toml
[keys.normal]
"j" = "move_down" 
"CTRL-c" = "exit" # Note: Modifiers may be different in Code
```

```mermaid
stateDiagram-v2
    direction LR
    [*] --> TOMLValues: parsing from config (serde)
    [*] --> Keymap: default mapping
    TOMLValues --> ParsedKeymap: validation (serde)
    ParsedKeymap --> Keymap: Merge
```

**Parsing Key inputs**

State transitions based on the tree and current state. At each state, the
commands are executed. Will follow the longest match approach!

```mermaid
stateDiagram-v2
    direction LR
    [*] --> None: initial state
    None --> KeymapTree(a): Key input "a"
    KeymapTree(a) --> None : Key input not in subtree of "a"
    KeymapTree(a) --> KeymapTree(b) : Key input "b" subsequently after "a"
``` 

## Implementation Quirks

- Use `Arc`(from `arc_swap`) to allow for multiple references to the same KeymapTree
- Use `HashMap` for O(1) lookup of KeymapTree (and to avoid Tree Traversal
Implementation hehe)
- A subtree `None` denotes that it is the leaf
- Use `serde` for config deserialization
- Add Whitecard Character matching to allow for "any" key input

