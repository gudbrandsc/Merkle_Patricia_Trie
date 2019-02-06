extern crate crypto;

use std::collections::HashMap;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

enum Node {
    Branch([String; 17]),
    Flag((Vec<u8>, String)),
    Null(),
}

//#[derive(Debug)]
struct MerklePatriciaTrie {
    db: HashMap<String, Node>,
    root: String,
}

impl MerklePatriciaTrie {
    fn get(&mut self, key: &str) -> String {
        // TODO
    }

    fn insert(&mut self, key: &str, new_value: &str) {
        // TODO
    }

    fn delete(&mut self, key: &str) {
        // TODO
    }
}
/*
fn compact_encode(hex_array: Vec<u8>) -> Vec<u8> {
    //println!("{:?}",hex_array );
    let mut mut_hex_array = hex_array.clone();

    let term = {
        if hex_array[hex_array.len() - 1] == 16 {
            1
        } else {
            0
        }
    };

    if term == 1 {
        mut_hex_array.remove(hex_array.len()-1);
        // println!("{:?}", mut_hex_array);
    }

    let oddlen:u8 = (mut_hex_array.len() % 2) as u8;
    let flags = 2 * term + oddlen;

    if oddlen == 1 {
        mut_hex_array.insert(0, flags);
    } else {
        mut_hex_array.insert(0, flags);
        mut_hex_array.insert(1, 0);

    }
    // let mut o = String::from("");

    let mut i = 0;

    let mut result : Vec<u8> = Vec::new();
    while i != mut_hex_array.len() {
        let value = 16 * mut_hex_array[i] + mut_hex_array[i + 1];
        result.push(value);
        println!("{}", value);

        i = i + 2;
    }
    return result;
}
*/
// If Leaf, ignore 16 at the end
/*fn compact_decode(encoded_arr: Vec<u8>) -> Vec<u8> {
    // TODO
}
/*
fn test_compact_encode() {
    assert_eq!(compact_decode(compact_encode(vec![1, 2, 3, 4, 5])),
               vec![1, 2, 3, 4, 5]);

    assert_eq!(compact_decode(compact_encode(vec![0, 1, 2, 3, 4, 5])),
               vec![0, 1, 2, 3, 4, 5]);

    assert_eq!(compact_decode(compact_encode(vec![0, 15, 1, 12, 11, 8, 16])),
               vec![0, 15, 1, 12, 11, 8]);

    assert_eq!(compact_decode(compact_encode(vec![15, 1, 12, 11, 8, 16])),
               vec![15, 1, 12, 11, 8]);
}
*/

fn hash_node(node: &Node) -> String {
    let mut hasher = Sha3::sha3_256();
    match node {
        Node::Branch(branch) => {
            let mut input = String::from("branch_");
            for each in branch {
                input += &*each;
            }
            hasher.input_str(&*input);
        },
        Node::Flag((encoded_prefix, value)) => {hasher.input_str(&*value);},
        Node::Null() => {hasher.input_str("");},
    }
    String::from("HashStart_") + &*(hasher.result_str()) + "_HashEnd"
}