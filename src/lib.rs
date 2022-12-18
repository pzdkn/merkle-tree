use std::str;
use sha2::{Sha256, Digest};

#[derive(Clone)]
pub struct MerkleTree<'a> {
    pub root: Box<Node<'a>>,
}
#[derive(Clone)]
pub enum Node<'a> {
    LeafNode {
        hash: String,
        data: &'a u8,
    },
    InternalNode {
        hash: String,
        children: [Box<Node<'a>>; 2],
    },
}


impl<'a> Node<'a>{
    /// Create a new internal node. The hash of the internal node is the hashes of its children. 
    /// The children and their hashes are sorted by ascending order.
    fn new_internal(children: [Box<Node<'a>>; 2]) -> Node<'a>{
        let [left, right] = children;
        let sorted_children: [Box<Node>; 2];
        if left.get_hash() > right.get_hash() {
            sorted_children = [right, left];
        } else {
            sorted_children = [left, right]
        }
        let concat_hash = sorted_children.iter().fold("".to_string(), |acc, x| acc + x.get_hash());
        let hash = format!("{:X}", Sha256::digest(concat_hash));
        Node::InternalNode{hash, children: sorted_children}
    }
    /// Create a new leaf node. The hash of the leaf node is the hashed data sample.
    fn new_leaf(data: &'a u8) -> Node{
        let data_str = format!("{:X}", data);
        let hash = format!("{:X}", Sha256::digest(data_str));
        Node::LeafNode{hash, data}
    }

    pub fn get_hash(&self) -> &str {
        match self {
            Node::InternalNode{hash, children} => {return  hash;} 
            Node::LeafNode { hash, data } => {return hash;}
        }
    }

    pub fn get_data(&self) -> Option<&u8> {
        match self {
            Node::InternalNode { hash, children } => {None}
            Node::LeafNode { hash, data } => {Some(data)}
        }
    }

    // Retrieve the children hashes in sorted order.
    pub fn get_children_hashes<'b>(children: &'b[Box<Node<'a>>; 2]) -> [&'b str; 2] {
        [&children[0].get_hash(), &children[1].get_hash()]
    }
}


impl<'a>  MerkleTree<'a> {

    /// Construct the merkle tree from array of bytes. We first initialize the leaf nodes and then pass the 
    /// hashed nodes to be recursively hashed by build_tree. Hashing is done upon initialization of the node.
    pub fn new(data: &Vec<u8>) -> MerkleTree{
    
        let mut leafs = Vec::new();
        for item in data {
            let leaf = Node::new_leaf(item);
            leafs.push(Box::new(leaf));
        }
        if data.len() % 2 != 0 {
            leafs.push(Box::new(Node::new_leaf(&0))); 
        }
        let root = MerkleTree::build_tree(leafs);
        MerkleTree{root}
    }

    /// Build tree layer by layer. In each recursion level, take pairs of two and contstruct a parent node from them.
    /// Returns the root node.
    fn build_tree(mut nodes: Vec<Box<Node<'a>>>) -> Box<Node<'a>>{
        if nodes.len() == 1 {
            let root = nodes.pop().unwrap();
            return root;
        } 
        else {
            let mut parent_nodes = Vec::new();
            let half_size = (nodes.len() as i32 / 2) as i32;
            for _ in 0..half_size{
                let node_pair = [nodes.pop().unwrap(), nodes.pop().unwrap()];
                let node = Box::new(Node::new_internal(node_pair));
                parent_nodes.push(node);
            }
            return MerkleTree::build_tree(parent_nodes);
        }
    }

    /// Returns the merkle proof for the given target data. 
    /// The merkle proof is a sequence of hashes that can be used to verify that the target data is in the tree.
    pub fn proof<'b>(&self, target: &'b u8) -> Option<Vec<& str>>{
        let target_node = Node::new_leaf(target);
        MerkleTree::build_proof(&self.root, &target_node)
    }

    fn build_proof<'b>(cur_node: &'a Box<Node>, target_node: &'b Node) -> Option<Vec<&'a str>> {
        match &**cur_node {
            Node::InternalNode{hash: _,  children} => {
                let [first_child, second_child] = children;
                let first_proof = MerkleTree::build_proof(first_child, target_node);
                let second_proof = MerkleTree::build_proof(second_child, target_node);
                let mut cur_proof = Vec::new();

                if let Some(sub_proof) = first_proof {
                    cur_proof.extend(sub_proof.iter());
                    cur_proof.push(second_child.get_hash());
                    return Some(cur_proof);
                } else if let Some(sub_proof) = second_proof {
                    cur_proof.push(first_child.get_hash());
                    cur_proof.extend(sub_proof.iter());
                    return Some(cur_proof);
                } else {
                    None
                }
            },
            Node::LeafNode{hash: _, data: _} => {
                if cur_node.get_hash() == target_node.get_hash() {
                    Some(vec![cur_node.get_hash()])
                } else {
                    None
                }
            }
        }
    }

    pub fn get_leafs(&self) -> Vec<&Node>{
        MerkleTree::get_leafs_rec(&self.root)
    }

    fn get_leafs_rec(node: &'a Node) -> Vec<&'a Node<'a>>{
        let mut leafs = Vec::new();
        match node {
            Node::InternalNode { hash: _, children } => {
                for child in children{
                    leafs.extend(MerkleTree::get_leafs_rec(&child).iter());
                }
            }
            Node::LeafNode { hash: _, data: _ } => {
                leafs.push(node);
            }
        }
        leafs
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    /// Verify that all data entered into the tree are contained inside the tree.
    #[test]
    fn test_leafs_contains_data(){
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut set: HashSet<u8> = HashSet::new();
        set.extend(data.iter());

        let mtree = MerkleTree::new(&data);
        let leafs = mtree.get_leafs();
        let leaf_data: Vec<u8> = leafs.iter().map(|x| *x.get_data().unwrap()).collect();
        let mut leaf_set: HashSet<u8> = HashSet::new();
        leaf_set.extend(leaf_data.iter());

        assert!(set.is_subset(&leaf_set));
        assert!(set.is_superset(&leaf_set));
    }
    /// Verify that the proof length is correct. In each layer you add one additional hash to the tree, excep at the leaf layer.
    /// In the leaf layer you have two. So proof length is log_2(data_len) + 1
    fn test_proof_len(){
        let data = vec![0, 1 , 2, 3, 4, 5, 6, 7];
        let mtree = MerkleTree::new(&data);
        let proofs = mtree.proof(&1).unwrap();

        assert_eq!(proofs.len(), 4);
    }
}