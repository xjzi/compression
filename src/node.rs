use std::cmp::Ord;
use std::cmp::Ordering;

pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub val: Option<u8>,
    pub freq: u32
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        //Backwards ordering for min heap
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Node {}

