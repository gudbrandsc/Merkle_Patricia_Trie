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
    let mut mpt = MerklePatriciaTrie {
        db: HashMap::new(),
        root: String::from("")
    };



    mpt.insert("a", "apple");
    mpt.insert("ab", "absolute");
   // mpt.insert("sd1", "gudbrand");

  /*  println!("{:?}", mpt.get("a"));
    println!("{:?}", mpt.get("ab"));
    println!("{:?}", mpt.get("sd1"));
*/
    //   check_node_type_test(mpt.db.get(&mpt.root).unwrap());
  //  check_node_type_test(mpt.db.get("HashStart_5597c95e8b6d318124b37b6b065493258b88b07f1d94fd0e42024b13fb181ef8_HashEnd").unwrap());
    //check_node_type_test(mpt.db.get("HashStart_42a990655bffe188c9823a2f914641a32dcbb1b28e8586bd29af291db7dcd4e8_HashEnd").unwrap());

    //println!("{:?}", mpt.db.len())


}
/*
fn check_node_type_test(node: &Node) {
    match node {
        Node::Branch(branch) => {
            println!("Branch ")
        },
        Node::Flag((encoded_prefix, value)) => {
            let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()))[0];
            if first_nibbel == 3 || first_nibbel == 4 {
                println!("Leaf")

            }else {
                println!("Exstention ");

            }
        },
        Node::Null() => {},
    }
}*/


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


        fn check_if_full_match(key: &Vec<u8>, node_prefix: &Vec<u8>) -> usize{
            let mut index = 0;

            for v in node_prefix {
                if *v != key[index]{
                }
                index = index + 1;
            }
            index
        }

        fn recusive_insert(&mut self, key: &Vec<u8>, new_value: &str, next_node_hash: &str) -> String {
            let node = self.db.get_mut(next_node_hash).unwrap();
            let mut return_value_hash =  String::from("");
            let mut nibbel_key = key.clone();
           // println!("Path: {:?}", zero_index);

            match node {
                Node::Branch(branch) => {
                    let branch_copy = branch.clone();
                    let zero_index = nibbel_key[0] as usize;
                    if branch[zero_index] == "" {
                        let mut nibbel_key:Vec<u8> = nibbel_key[1..].to_vec();
                        nibbel_key.push(0x10);
                        let leaf_node = Node::Flag((compact_encode(nibbel_key), new_value.to_string()));


                        let mut branch_copy = branch.clone();

                        branch_copy[zero_index] = hash_node(&leaf_node);





                    } else {
                        // call insert on next node
                        //Update current node hash
                        // return current node hash
                        //nibbel_key = nibbel_key[1..].to_vec();

                    }


                },
                Node::Flag((encoded_prefix, value)) => {
                    let encoded_prefix_copy = encoded_prefix.clone();
                    let value_copy = value.clone();
                    let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix_copy.clone()))[0];

                    let mut current_node_nibbel_decoded =  compact_decode(encoded_prefix.clone());
                    let current_node_hash = hash_node(&Node::Flag((compact_encode(current_node_nibbel_decoded.clone()), value_copy.clone())));

                    //Cheack if leaf node
                    if first_nibbel == 2 || first_nibbel == 3 {
                        println!("Found leaf node with nibbel: {:?}", current_node_nibbel_decoded);
                        if current_node_nibbel_decoded[0] == key[0]{
                            let mut common_prefix = true;
                            let mut ext_node_nibbel: Vec<u8> = Vec::new();

                            while common_prefix {
                                if !(current_node_nibbel_decoded.len() == 0 || nibbel_key.len() == 0) {
                                    if current_node_nibbel_decoded[0] == nibbel_key[0] {
                                        ext_node_nibbel.push(nibbel_key[0]);
                                        current_node_nibbel_decoded = current_node_nibbel_decoded[1..].to_vec();
                                        nibbel_key = nibbel_key[1..].to_vec();
                                    } else {
                                        common_prefix = false;
                                    }
                                } else {
                                    common_prefix = false;
                                }
                            }
                            println!("Found common nibbel for EXT: {:?}", ext_node_nibbel);
                            let mut branch_array: [String; 17] = Default::default();

                            if current_node_nibbel_decoded.len() == 0 {
                                println!("Existing node nibbel length =  {:?}", current_node_nibbel_decoded.len());
                                println!("Create branch node with value of the exsisting leaf node");
                                branch_array[16] = value_copy;
                                let leaf_node_zero_index = nibbel_key[0] as usize;
                                nibbel_key.remove(0);
                                nibbel_key.push(0x10);
                                println!("Value of branch node = {:?}", value.to_string());
                                println!("Created leaf in branch index {:?} with value = {:?} and nibbel : {:?} ", leaf_node_zero_index, new_value.to_string(), nibbel_key);
                                let leaf_node = Node::Flag((compact_encode(nibbel_key), new_value.to_string()));
                                branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
                                self.db.insert(hash_node(&leaf_node),leaf_node);

                            } else if nibbel_key.len() == 0 {
                                println!("New node nibbel length =  {:?}", nibbel_key.len());
                                println!("Create branch node with value of the new leaf node");
                                branch_array[16] =  new_value.to_string();
                                let leaf_node_zero_index = current_node_nibbel_decoded[0] as usize;
                                current_node_nibbel_decoded.remove(0);
                                current_node_nibbel_decoded.push(0x10);
                                println!("Value of branch node = {:?}", new_value.to_string());
                                println!("Created leaf in branch index {:?} with value = {:?} and nibbel : {:?} ", leaf_node_zero_index, new_value.to_string(), current_node_nibbel_decoded);
                                let leaf_node = Node::Flag((compact_encode(current_node_nibbel_decoded), value_copy.to_string()));
                                branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
                            } else {
                                println!("Both new node and existing node has more nibbels -> creating two nodes and branch");

                                let mut leaf_node_zero_index = current_node_nibbel_decoded[0] as usize;
                                current_node_nibbel_decoded.remove(0);
                                println!("Created node with nibbel = {:?}", current_node_nibbel_decoded);
                                current_node_nibbel_decoded.push(0x10);
                                let mut leaf_node = Node::Flag((compact_encode(current_node_nibbel_decoded), value_copy.to_string()));
                                println!("Inserting node to branch index = {:?}", leaf_node_zero_index);
                                branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
                                //TODO insert to db
                                //Add second leaf
                                leaf_node_zero_index = nibbel_key[0] as usize;
                                nibbel_key.remove(0);
                                println!("Created node with nibbel = {:?}", nibbel_key);
                                nibbel_key.push(0x10);
                                println!("Inserting node to branch index = {:?}", leaf_node_zero_index);
                                leaf_node = Node::Flag((compact_encode(nibbel_key), new_value.to_string()));
                                branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
                            }
                            println!("Created extension node with nibbel = {:?}", ext_node_nibbel);

                            let branch_node = Node::Branch(branch_array);
                            let exstention_node = Node::Flag((compact_encode(ext_node_nibbel), hash_node(&branch_node)));
                            return_value_hash = hash_node(&exstention_node);

                        } else {
                            let mut branch_array: [String; 17] = Default::default();

                            if current_node_nibbel_decoded.len() == 0 {
                                branch_array[16] = value_copy.to_string(); //Put value in branch value
                                let node_copy = Node::Flag((compact_encode(current_node_nibbel_decoded.clone()), value_copy.to_string()));

                                let current_node_hash = hash_node(&node_copy);
                                self.db.remove(&current_node_hash); //Remove old leaf node


                            }else {
                                let current_node_branch_extender = current_node_nibbel_decoded[0] as usize;
                                let node_copy = Node::Flag((compact_encode(current_node_nibbel_decoded.clone()), value_copy.to_string()));

                                let current_node_hash = hash_node(&node_copy);

                                self.db.remove(&current_node_hash);
                                let mut current_node_new_prefix:Vec<u8> = current_node_nibbel_decoded[1..].to_vec();
                                current_node_new_prefix.push(0x10);

                                let leaf_node = Node::Flag((compact_encode(current_node_nibbel_decoded), value_copy.to_string()));
                                let leaf_node_hash = hash_node(&leaf_node);
                                branch_array[current_node_branch_extender] = leaf_node_hash.clone();

                                self.db.insert(leaf_node_hash, leaf_node);


                                let new_node_branch_extender = key[0] as usize;
                                let mut new_node_prefix:Vec<u8> = key[1..].to_vec();
                                new_node_prefix.push(0x10);

                                let new_leaf_node = Node::Flag((compact_encode(new_node_prefix), new_value.to_string()));
                                let new_leaf_node_hash = hash_node(&new_leaf_node);
                                branch_array[new_node_branch_extender] = new_leaf_node_hash.clone();
                                self.db.insert(new_leaf_node_hash, new_leaf_node);
                               // println!("Root: {:?}", branch_array);

                                let branch_node = Node::Branch(branch_array);
                                let return_value_hash = hash_node(&branch_node);


                                self.db.insert(return_value_hash.clone(), branch_node);

                                if current_node_hash == self.root {
                                    self.root = return_value_hash.clone();
                                }
                                return return_value_hash;

                            }

                        }
                    } else {
                        //Check if ext nibbel matches
                        //if true then call recursive and update node after it returns
                        //Else create a exstetion node if possible
                        //Todo its an exstention node so
                    }
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
                println!("Inserted root leaf node with nibbel: {:?}",path_vec);
                path_vec.push(0x10);
                let leaf_node_path = compact_encode(path_vec);

                let leaf_node = Node::Flag((leaf_node_path, new_value.to_string()));
                self.root = hash_node(&leaf_node);

                self.db.insert(hash_node(&leaf_node), leaf_node);

            }else {
                let new_node_prefix = string_to_vec_u8(&ascii_to_hex(string_to_ascii(&key)));
                println!("Inserted new node with path: {:?}",new_node_prefix);

                Self::recusive_insert(self, &new_node_prefix, new_value, &self.root.clone());
            }
        }

        fn get(&mut self, key: &str) -> String {
            let mut key_prefix = string_to_vec_u8(&ascii_to_hex(string_to_ascii(&key)));
            let mut node_hash = self.root.clone();
            let mut done = false;
            let mut result = String::from("");

            while !done {
                let node = self.db.get_mut(&node_hash).unwrap();
                match node {
                    Node::Branch(branch) => {
                        if key_prefix.len() == 0 {
                            result = branch[16].clone();
                            done = true;
                        }

                        if branch[key_prefix[0] as usize] != "" {
                            node_hash = branch[key_prefix[0] as usize].clone();
                            key_prefix = key_prefix[1..].to_vec();
                        }else {
                            result = String::from("");
                            done = true;

                        }
                    },
                    Node::Flag((encoded_prefix, value)) => {
                        let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()))[0];
                        let node_prefix_decoded =  compact_decode(encoded_prefix.clone());
                        if first_nibbel == 3 || first_nibbel == 4 {
                            if node_prefix_decoded == key_prefix {
                                result = value.clone();
                                done = true;

                            }else {
                                result = String::from("");
                                done = true;

                            }
                        }else {
                            for c in node_prefix_decoded {
                                if c == key_prefix[0] {
                                    key_prefix = key_prefix[1..].to_vec();
                                } else {
                                    result = String::from("")
                                }
                            }
                            node_hash = value.clone();
                        }
                    },
                    Node::Null() => {},
                }

            }
            return result;

        }

    }



    fn get_u8_bytes(v: u32) -> (u8) {
        (v & 0x000000ff >>  0) as u8
    }

    #[allow(dead_code)]

// If Leaf, ignore 16 at the end
    fn compact_decode(encoded_arr: Vec<u8>) -> Vec<u8> {

        let mut result : Vec<u8> = Vec::new();
        let mut i = 0;

        while i != encoded_arr.len() {
            let hex_val = format!("{:x}", encoded_arr[i]);

            for less_than_nine in hex_val.chars() {
                let current_value = less_than_nine.to_digit(16).unwrap();
                if i == 0 {
                    if encoded_arr[i] == 0 || encoded_arr[i] == 2{
                        result.push(get_u8_bytes(current_value));
                        result.push(get_u8_bytes(current_value));
                        result.push(get_u8_bytes(current_value));


                    }else {
                        result.push(get_u8_bytes(current_value));
                    }
                }else {
                    result.push(get_u8_bytes(current_value));

                }



            }
            i = i + 1;
        }

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
        let flags = 2 * term + oddlen;
        if oddlen == 1 {
            mut_hex_array = vector_front_appender(mut_hex_array,flags);
        } else {
            mut_hex_array = vector_front_appender(mut_hex_array,0);
            mut_hex_array = vector_front_appender(mut_hex_array,flags);
        }

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
            Node::Flag((_encoded_prefix, value)) => {hasher.input_str(&*value);},
            Node::Null() => {hasher.input_str("");},
        }
        String::from("HashStart_") + &*(hasher.result_str()) + "_HashEnd"
    }