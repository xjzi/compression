use crate::node::Node;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use bitvec::prelude::*;

fn traverse_node(map: &mut HashMap<u8, BitVec>, node: &Node, code: &mut BitVec) {
    //Node will only have a value if it is a leaf
    if let Some(val) = node.val {
        map.insert(val, code.clone());
    } else {
        //Node will always have two children if it doesn't have a value
        code.push(false);
        traverse_node(map, &node.left.as_ref().unwrap(), code);
        *code.last_mut().unwrap() = true;
        traverse_node(map, &node.right.as_ref().unwrap(), code);
        code.pop();
    }
}

pub fn huffman(freq: &HashMap<u8, u32>) ->  HashMap<u8, BitVec> {
    let mut heap: BinaryHeap<Box<Node>> = BinaryHeap::new();

    //Initially populate binary tree with nodes
    for (key, val) in freq.iter() {
       let node = Node {
          left: None,
          right: None,
          val: Some(*key),
          freq: *val
        };
        heap.push(Box::from(node));
    }
    
    let root = loop {
        let i = heap.pop().unwrap();
        if let Some(j) = heap.pop() {
            let node = Node {
                freq: i.freq + j.freq,
                left: Some(Box::from(i)),
                right: Some(Box::from(j)),
                val: None
            };
            heap.push(Box::from(node));
        } else { break i }
    };

    let mut map: HashMap<u8, BitVec> = HashMap::new();
    traverse_node(&mut map, &root, &mut BitVec::new());
    map
}

