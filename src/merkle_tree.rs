use std::Vec;
use sha2::{Sha256};

struct MerkleTree {
    root: Node
}

enum Node {
    LeafNode{hash: String, data: &u8},
    InternalNode{hash: String, children: [&Node; 2]}
}

impl Node {
    fn new(children: [&Node; 2]){
        let concat_hash = children.fold("", |acc, x| concat!(acc, x.hash));
        let hash = Sha256::digest(concat_hash);
        Node::InternalNode{hash, children}
    }
    fn new(data: &u8){
        let hash = Sha256::digest(data);
        Node::LeafNode{hash, data}
    }
}


impl MerkleTree {
    pub fn new(data: &mut Vec<u8>) -> MerkleTree{
        /*
        Construct the merkle tree from array of bytes.
        We first initialize the leaf nodes and then pass 
        the hashed nodes to be recursively hashed by build_tree.
        Hashing is done upon initialization of the node.
         */
        if data.len() % 2 != 0 {
            data.append(0)
        }
        let hash_arr = Vec<String>::new();
        for item in data {
            let leaf_0 = LeafNode::new(item);
        }
        let root = build_tree(hash_arr);
        MerkleTree(root)
    }
    fn build_tree(hash_arr: Vec<String>) -> Hashable{
        if hash_arr.len() == 1 {
            return hash_array[0];
        } 
        else {
            let new_hash_arr = Vec<String>::new();
            for chunk in hash_arr.chunks(2){
                let node = InternalNode::new(chunk);
                new_hash_arr.push(node);
                }
            return build_tree(new_hash_arr);
        }
    }
    fn delete(){}
    fn insert(){}
    fn proof(){}

}