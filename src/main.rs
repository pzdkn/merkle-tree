use merkle_tree::{MerkleTree};
use sha2::{Sha256, Digest};

fn main() {
    // Setting up data
    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mtree = MerkleTree::new(&data);
    let target : u8 = 7;

    // Constructing the proof
    let merkle_proof = mtree.proof(&target).unwrap();
    print!("Lisitng all proofs needed to verify that {target} is within the merkel tree \n");
    for proof in &merkle_proof{
        print!("proof: {proof} \n")
    }

    // Verifying the proof
    let mut my_hash = format!("{}", merkle_proof[0]);
    for proof in &merkle_proof[1..] {
        if my_hash > proof.to_string() {
            my_hash = format!("{:X}", Sha256::digest(proof.to_string() + &my_hash));
        } else {
            my_hash = format!("{:X}", Sha256::digest(my_hash + proof));
        }

    }

    let root_hash =  mtree.root.get_hash();
    println!("Recomputed hash: {my_hash}");
    println!("Root hash: {root_hash}");
}