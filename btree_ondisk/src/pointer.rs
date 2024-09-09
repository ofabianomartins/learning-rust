use std::{cell::RefCell, rc::Rc};

use std::fmt;

use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::convert::From;
use std::convert::TryFrom;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Offset(pub usize);

/// Converts an array of length len(usize) to a usize as a BigEndian integer.
impl TryFrom<[u8; PTR_SIZE]> for Offset {
    type Error = Error;

    fn try_from(arr: [u8; PTR_SIZE]) -> Result<Self, Self::Error> {
        Ok(Offset(usize::from_be_bytes(arr)))
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Key(pub String);


#[derive(Clone, Eq, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

impl Ord for KeyValuePair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for KeyValuePair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for KeyValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl KeyValuePair {
    pub fn new(key: String, value: String) -> KeyValuePair {
        KeyValuePair { key, value }
    }
}

// NodeType Represents different node types in the BTree.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NodeType {
    /// Internal nodes contain a vector of pointers to their children and a vector of keys.
    Internal(Vec<Offset>, Vec<Key>),

    /// Leaf nodes contain a vector of Keys and values.
    Leaf(Vec<KeyValuePair>),

    Unexpected,
}

// Converts a byte to a NodeType.
impl From<u8> for NodeType {
    fn from(orig: u8) -> NodeType {
        match orig {
            0x01 => NodeType::Internal(Vec::<Offset>::new(), Vec::<Key>::new()),
            0x02 => NodeType::Leaf(Vec::<KeyValuePair>::new()),
            _ => NodeType::Unexpected,
        }
    }
}

// Converts a NodeType to a byte.
impl From<&NodeType> for u8 {
    fn from(orig: &NodeType) -> u8 {
        match orig {
            NodeType::Internal(_, _) => 0x01,
            NodeType::Leaf(_) => 0x02,
            NodeType::Unexpected => 0x03,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node { 
    pub node_type: NodeType,

    pub keys: Vec<u8>,
    pub values:: Vec<u8>,
    pub is_root: bool,
    pub parent_offset: Option<Offset>,
}

impl Node {
    pub fn new(is_root: bool, parent_offset: Option<Offset>, value: i32) -> Self {
        Self { is_root, parent_offset, node_type: NodeType::Leaf(Vec::new()) }
    }
}

#[derive(Debug, Clone, Display)]
pub struct Btree {
    root: Link,
    order: usize
}

impl fmt::Display for Btree {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.root)
    }
}

impl Btree {
    pub fn new(order: usize) -> Self {
        Btree { root: None, order: order }
    }

    pub fn push(&mut self, value: u8) {

    }

    pub fn find(&mut self, value: u8) -> bool {
        return false;
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_left() {
        let mut queue = Btree::new(2);

        queue.push(1);
        queue.push(3);
        queue.push(4);
        queue.push(5);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), None);
    }
}
