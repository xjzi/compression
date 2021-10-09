use std::collections::BinaryHeap;
use bitvec::prelude::*;
use super::node::Node;
use std::convert::TryInto;
use std::iter;

struct Out<'a> {
    map: &'a mut Vec<BitVec>,
    buf: &'a mut BitVec<Lsb0, u8>, 
    code: &'a mut BitVec
}

fn traverse_node(out: &mut Out, node: &Node) {
    if let Some(val) = node.val {
        //Node will only have a value if it is a leaf
        //Add value to hashmap
        out.map[val as usize] = out.code.clone();
        
        //Write node to buffer
        out.buf.push(true);
        out.buf.append(&mut BitVec::<Lsb0, u8>::from_element(val));
    } else {
        //Node will always have two children if it doesn't have a value
        //Write node to buffer
        out.buf.push(false);

        //Add false to code and call on left child
        out.code.push(false);
        traverse_node(out, &node.left.as_ref().unwrap());

        //Replace with true and call on right child
        *out.code.last_mut().unwrap() = true;
        traverse_node(out, &node.right.as_ref().unwrap());

        //Remove the extra added bit from the code
        out.code.pop();
    }
}

pub fn parse_tree(root: &Node) -> (BitVec<Lsb0, u8>, Vec<BitVec>) {
    //Will contain the serialized tree
    let mut buf: BitVec<Lsb0, u8> = BitVec::new();

    //Vector of 256 bitvecs
    let mut map: Vec<BitVec> = iter::repeat_with(|| BitVec::new())
        .take(256)
        .collect();

    let mut out = Out {
        map: &mut map,
        buf: &mut buf,
        code: &mut BitVec::new()
    };

    //Recursively populates map and serializes tree
    traverse_node(&mut out, &root);

    (buf, map)
}

pub fn get_tree(freq: &[u32]) -> Node {
    let mut heap: BinaryHeap<Box<Node>> = BinaryHeap::new();

    //Removes bytes that are used 0 times 
    let freq = freq.iter().enumerate().filter(|(_count, val)| **val != 0);

    //Initially populate binary tree with nodes
    for (count, val) in freq {
        let node = Node {
          left: None,
          right: None,
          val: Some(count.try_into().unwrap()),
          freq: *val
        };
        heap.push(Box::from(node));
    }

    //Combine nodes until there's one node left
    loop {
        //There will always be one node to pop
        let i = heap.pop().unwrap();

        //There might be another node to pop
        if let Some(j) = heap.pop() {
            //Push a parent node with the sum of their frequencies
            let node = Node {
                freq: i.freq + j.freq,
                left: Some(Box::from(i)),
                right: Some(Box::from(j)),
                val: None
            };
            heap.push(Box::from(node));
        } else { 
            //If there is only one node, then the tree is complete
            break *i 
        }
    }
}

pub fn get_freq(input: &[u8]) -> [u32; 256] {
    let mut map: [u32; 256] = [0; 256]; 
    for val in input {
        map[*val as usize] = map[*val as usize] + 1;
    }
    map
}

pub fn code(bytes: &[u8], buf: &mut BitVec<Lsb0, u8>, map: &Vec<BitVec>) {
    for byte in bytes {
        let code = &map[*byte as usize];
        buf.append(&mut code.clone());
    }
}

pub fn compress(bytes: &[u8]) -> BitVec<Lsb0, u8> {
    let freq = get_freq(&bytes);
    let tree = get_tree(&freq);
    let (mut buf, map) = parse_tree(&tree);
    code(bytes, &mut buf, &map);
    buf
} 

