use crate::huffman::node::Node; 
use bitvec::prelude::*;

pub fn build_tree(buf: &mut BitVec<Lsb0, u8>) -> Node {
    //If the next bit is true
    if buf.remove(0) {
        let val: BitVec = buf.drain(..8).collect();
        let val = val.load::<u8>();
        return Node {
            left: None,
            right: None,
            val: Some(val),
            freq: 0
        };
    } else {
        return Node {
            left: Some(Box::from(build_tree(buf))),
            right: Some(Box::from(build_tree(buf))),
            val: None,
            freq: 0
        }
    }
}
