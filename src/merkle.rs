#[cfg(feature = "serialization-serde")]
extern crate serde_json;

use ring::digest::{Algorithm, Context, SHA512};

use hashutils::{HashUtils, Hashable};
use merkletree::MerkleTree;
use proof::Positioned;

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

fn test_from_str_vec() {
    let values = vec!["one", "two", "three", "four"];

    let hashes = vec![
        digest.hash_leaf(&values[0].as_bytes()),
        digest.hash_leaf(&values[1].as_bytes()),
        digest.hash_leaf(&values[2].as_bytes()),
        digest.hash_leaf(&values[3].as_bytes()),
    ];

    let count = values.len();
    let tree = MerkleTree::from_vec(digest, values);

    let h01 = digest.hash_nodes(&hashes[0], &hashes[1]);
    let h23 = digest.hash_nodes(&hashes[2], &hashes[3]);

    let root_hash = digest.hash_nodes(&h01, &h23);

    assert_eq!(tree.count(), count);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}
}