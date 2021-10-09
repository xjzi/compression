use super::node::Node; 
use bitvec::prelude::*;

pub fn build_tree(buf: &mut BitVec<Lsb0, u8>) -> Node {
    //Pop the has_value flag from the front of the buffer
    if buf.remove(0) {
        //If the value flag is true, then the node is a leaf node with a value
        let val: BitVec = buf.drain(..8).collect();
        let val = val.load::<u8>();
        return Node {
            left: None,
            right: None,
            val: Some(val),
            freq: 0
        };
    } else {
        //If it's not a leaf node, then it will have two children following
        return Node {
            left: Some(Box::from(build_tree(buf))),
            right: Some(Box::from(build_tree(buf))),
            val: None,
            freq: 0
        }
    }
}

pub fn decode(buf: BitVec<Lsb0, u8>, tree: &Node) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let mut node: &Node = &tree;

    //Follow the bits on the tree until we reach a leaf node
    for bit in buf {
        node = match bit {
            false => &node.left.as_ref().unwrap(),
            true => &node.right.as_ref().unwrap()
        };
        if let Some(val) = node.val {
            //Add decoded value to vector
            vec.push(val);

            //Reset tree at root
            node = &tree;
        }
    }
    vec
}

pub fn decompress(mut bits: BitVec<Lsb0, u8>) -> Vec<u8> {
    //build_tree pops all bits associated with the tree
    let tree = build_tree(&mut bits);
    decode(bits, &tree)
}
