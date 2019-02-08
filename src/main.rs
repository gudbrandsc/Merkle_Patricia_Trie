//include!("merkle_patricia_trie.rs");
extern crate crypto;
#[warn(unused_imports)]
#[warn(unused_variables)]
#[allow(dead_code)]
use std::collections::HashMap;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

#[allow(dead_code)]
#[derive(Debug)]

enum Node {
    Branch([String; 17]), //Only for branch
    Flag((Vec<u8>, String)), //Extention and leaf
    Null(),
}

#[derive(Debug)]
struct MerklePatriciaTrie {
    db: HashMap<String, Node>,
    root: String,
}


fn main() {
    // test_compact_encode();
/*    let mut val1 = vec![2,0x10];
    println!("{:?}",val1 );

    let mut encoded_value = compact_encode(vec![2,0x10]);
    println!("Encoded {:?}", encoded_value );

    println!("Encoded {:?}",ascii_to_hex( encoded_value ));

*/


        let mut mpt = MerklePatriciaTrie {
            db: HashMap::new(),
            root: String::from("")
        };

        mpt.insert("a", "apple");
        mpt.insert("ab", "absolute");


}
#[allow(dead_code)]
fn string_to_vec_u8(key: &str) -> Vec<u8> {
    let mut result : Vec<u8> = Vec::new();
    for value in key.chars() {
        let current_value = value.to_digit(16).unwrap();
        result.push(current_value as u8);
    }
    return result
}

fn string_to_ascii(key: &str) -> Vec<u8> {
    let char_list: Vec<char> = key.chars().collect();
    let mut ascii_vector : Vec<u8> = Vec::new();
    let mut i = 0;

    while i != char_list.len() {
        ascii_vector.push(char_list[i] as u8);
        i = i + 1;
    }
    return ascii_vector
}

fn ascii_to_hex(encoded_arr: Vec<u8>) -> String {
    let mut hex_string = String::from("");
    let mut i = 0;
    let mut zeros = false;
    while i != encoded_arr.len() {
        if encoded_arr[i] != 0 {
            zeros = true;
        }
        if zeros {
            let hex_val = format!("{:x}", encoded_arr[i]);
            hex_string.push_str(&hex_val);
        }
        i = i + 1;
    }
    return hex_string
}


#[allow(dead_code)]


impl MerklePatriciaTrie {



  /*  fn update(&mut self, old_hash: &str) {
        let root_hash = self.root.clone();
        let mut current_hash = old_hash;
        let mut updated_node_hash = hash_node(self.db.get(current_hash).unwrap());

        while root_hash != current_hash {

            match current_node {
                Node::Branch( branch) => {

                },
                Node::Flag((encoded_prefix, value)) => {
                    //Todo if node hash is equal to root then set current hash = root and break;
                },
                Node::Null() => {},
            }



        }




        println!("root: {:?} ", root_hash )
    }*/

    fn recusive_insert(&mut self, key: &str, new_value: &str, next_node_hash: &str) -> String {
        let mut node = self.db.get_mut(next_node_hash).unwrap();
        match node {
            Node::Branch(branch) => {
            },
            Node::Flag((encoded_prefix, value)) => {
                let mut current_node_prefix_decoded =  compact_decode(encoded_prefix.clone());
                let mut new_node_prefix = string_to_vec_u8(&ascii_to_hex(string_to_ascii(&key)));
                println!("{:?}", current_node_prefix_decoded);
                println!("{:?}", new_node_prefix);





                //If node is leaf then check if they have any matching prefix
                //

            },
            Node::Null() => {
            }
        }
        let val = String::from("hasah");
        val
    }





    fn insert(&mut self, key: &str, new_value: &str) {
        if self.root == "" {
            let mut path_vec = string_to_vec_u8(&ascii_to_hex(string_to_ascii(&key)));
            path_vec.push(0x10);
            let leaf_node_path = compact_encode(path_vec);
            println!("Inserted root leaf node..");

            let leaf_node = Node::Flag((leaf_node_path, new_value.to_string()));
            self.root = hash_node(&leaf_node);
            self.db.insert(hash_node(&leaf_node), leaf_node);

        }else {
            Self::recusive_insert(self, key, new_value, &self.root.clone());
        }
}

  /*  fn recursive_insert(mut self, mut key: &str, new_value: &str, node_ref: &str)  {
        match node_value {
            Node::Branch( branch) => {
                let mut list_index = key_hex_value.chars().next().unwrap().to_digit(8).unwrap();
                key_hex_value = key_hex_value[1..].to_string();
                list_index = list_index + 1;

            },
            Node::Flag((encoded_prefix, value)) => {
                /* let encoded_value = encoded_prefix.clone();
                 let decoded_value = compact_decode(encoded_value);
                 let nibble_string = ascii_to_hex(decoded_value);
                 let mut full_match = true;
                 let mut count = 0;
                 for i in nibble_string.chars() {
                     if key_hex_value.chars().next().unwrap() == i {
                         key_hex_value = key_hex_value[1..].to_string();
                     } else {
                         full_match = false;
                         println!("Go to next to find {:?}", key_hex_value);
                     }
                     count = count + 1;
                 }
                 if count == nibble_string.len() && full_match {
                     println!("Full match {:?}", value);
                     Self::recursive_insert(self, key, new_value, value);

                 } else {
                     println!("not full match need to update");
                 }*/
            },
            Node::Null() => {},
        }
        println!("{:?}", key_hex_value);


    }*/
}





fn root_is_leaf_node(node: &Node) -> bool {
    match node {
        Node::Branch(branch) => {
            return false;
        },
        Node::Flag((encoded_prefix, value)) => {
            return true
        },
        Node::Null() => {
            return false;
        }
    }
}







    #[allow(dead_code)]

    fn test_compact_encode() {
         assert_eq!(compact_decode(compact_encode(vec![1, 2, 3, 4, 5])),
                     vec![1, 2, 3, 4, 5]);

         assert_eq!(compact_decode(compact_encode(vec![ 0, 15, 1, 12, 11, 8, 10])),
                     vec![ 0, 15, 1, 12, 11, 8, 10]);

           assert_eq!(compact_decode(compact_encode(vec![0, 15, 1, 12, 11, 8])),
                      vec![0, 15, 1, 12, 11, 8]);

         assert_eq!(compact_decode(compact_encode(vec![15, 1, 12, 11, 8, 16])),
                     vec![15, 1, 12, 11, 8]);

    }
    fn get_u8_bytes(v: u32) -> (u8) {
        (v & 0x000000ff >>  0) as u8
    }

    #[allow(dead_code)]

// If Leaf, ignore 16 at the end
    fn compact_decode(encoded_arr: Vec<u8>) -> Vec<u8> {

        let mut result : Vec<u8> = Vec::new();
        let mut i = 0;
        let mut nibble = true;

        while i != encoded_arr.len() {
            let hex_val = format!("{:x}", encoded_arr[i]);
            println!("Values : {:?}",hex_val);

            for less_than_nine in hex_val.chars() {
                let current_value = less_than_nine.to_digit(16).unwrap();
                if i == 0 {
                    if encoded_arr[i] == 0 || encoded_arr[i] == 2{
                        println!("I should push : {:?}",encoded_arr[i]);
                        result.push(get_u8_bytes(current_value));
                        result.push(get_u8_bytes(current_value));
                        result.push(get_u8_bytes(current_value));

                        println!("result push : {:?}",result);

                    }else {
                        result.push(get_u8_bytes(current_value));
                    }
                }else {
                    result.push(get_u8_bytes(current_value));

                }



            }
            i = i + 1;
        }
        println!("Result remove {:?}", result);

        if result[0] == 0 {
            result.remove(0);
            println!("{:?}", result);
            result.remove(0);

        } else if result[0] == 2 {
            result.remove(0);
            result.remove(0);
        }else {
            result.remove(0);

        }

        result
    }

    #[allow(dead_code)]

    fn compact_encode(hex_array: Vec<u8>) -> Vec<u8> {
        let mut mut_hex_array:Vec<u8>  = hex_array.clone();
        let mut result : Vec<u8> = Vec::new();

        let term = {
            if hex_array[hex_array.len() - 1] == 16 {
                1
            } else {
                0
            }
        };

        if term == 1 {
            mut_hex_array.remove(hex_array.len()-1);
        }

        let oddlen:u8 = (mut_hex_array.len() % 2) as u8;
        let mut  flags = 2 * term + oddlen;
        if oddlen == 1 {
            mut_hex_array = vector_front_appender(mut_hex_array,flags);
        } else {
            mut_hex_array = vector_front_appender(mut_hex_array,0);
            mut_hex_array = vector_front_appender(mut_hex_array,flags);
        }
        println!("encoded : {:?}", mut_hex_array);

        let mut i = 0;
        while i != mut_hex_array.len()  {
            let value = 16 * mut_hex_array[i]  + mut_hex_array[i + 1];

            result.push(value);

            i = i + 2;
        }
        //println!("encoded : {:?}", result);
        return result;
    }

    fn vector_front_appender(vec_array: Vec<u8>, new_value: u8) -> Vec<u8> {
        let mut result : Vec<u8> = Vec::new();
        result.push(new_value);

        for v in vec_array {
            result.push(v);
        }
        result
    }

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