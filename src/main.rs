mod node;

use std::collections::HashMap;
use std::collections::BinaryHeap;
use bitvec::prelude::*;
use node::Node;

fn main() {
    let text = include_bytes!("alice.txt");
    println!("The file is {} bytes.", text.len());
    let freq = get_freq(text);
    let res = code_huffman(&freq);
    let mut res: Vec<(u8, BitVec)> = res.into_iter().collect();
    res.sort_by_key(|a| freq.get(&a.0).unwrap());
    for (val, code) in res.iter() {
        println!("{} | freq: {} code: {}.", *val as char, freq.get(val).unwrap(), code);
    }
}

fn compress_rle(input: &[u8]) -> Vec<u8> {
    let mut output : Vec<u8> = Vec::new();
    let mut run = input[0];
    let mut run_len = 0;
    for cur in input {
        if *cur == run {
            run_len = run_len + 1;
        }
        else {
            output.append(&mut vec![run_len, run]);
            run = *cur;
            run_len = 1;
        }
    }
    output.append(&mut vec![run_len, run]);
    output
}

fn get_freq(input: &[u8]) -> HashMap<u8, u32> {
    let mut map: HashMap<u8, u32> = HashMap::new();
    for val in input {
        let entry = map.entry(*val).or_insert(0);
        *entry = *entry + 1;
    }
    map
}

fn code_huffman(freq: &HashMap<u8, u32>) ->  HashMap<u8, BitVec> {
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
