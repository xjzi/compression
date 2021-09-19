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

fn code_huffman(freq: &HashMap<u8, u32>) ->  BitVec {
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

    BitVec::new()
}
