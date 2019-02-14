extern crate crypto;

use std::collections::HashMap;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

enum Node {
    Branch([String; 17]),
    Flag((Vec<u8>, String)),
    Null(),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Null() => write!(f, "[Null Node]"),
            Node::Branch(branch) => {
                write!(f, "Branch[");
                for i in 0..16 {
                    write!(f, "{}=\"{}\", ", i, branch[i]);
                }
                write!(f, "value={}]", branch[16])
            },
            Node::Flag((encoded_prefix, value)) => {
                write!(f, "{}<{:?}, value=\"{}\">",
                       if is_ext_node(encoded_prefix.to_vec()) {"Ext"} else {"Leaf"},
                       compact_decode(encoded_prefix.to_vec()),
                       value)
            }
        }
    }
}

impl Node {
    fn clone(&self) -> Node {
        match self {
            Node::Flag((prefix, value)) => Node::Flag((prefix.to_vec(), value.to_string())),
            Node::Branch(branch) => {
                let mut value: [String; 17] = empty_branch_value();
                for i in 0..17 {
                    value[i] = branch[i].to_string();
                }
                Node::Branch(value)
            },
            _ => Node::Null(),
        }
    }
}

#[derive(Debug)]
struct MerklePatriciaTrie {
    db: HashMap<String, Node>,
    root: String,
}

fn check_node_type(node: &Node)  {
    match node {
        Node::Branch(branch) => {
            println!("Its branch");
            println!("{:?}", branch);

        },
        Node::Flag((encoded_prefix, value)) => {
            println!("Its Flag");
            let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()));

            println!("{:?}",first_nibbel);
            if first_nibbel[0] == 2 || first_nibbel[0] == 3 {
                println!("Value {:?}",value);

            }


        },
        Node::Null() => {
            println!("Its null")
        },
    }
}

fn create_node_copy(node: &Node) -> Node {
    match node {
        Node::Branch(branch) => {
            let mut branch_copy = branch.clone();
            return Node::Branch(branch_copy)
        },
        Node::Flag((encoded_prefix, value)) => {
            return Node::Flag((encoded_prefix.clone(),value.clone()));

        },
        Node::Null() => {
            return Node::Null();
        },
    }
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
    fn new() -> MerklePatriciaTrie {
        let mut mpt = MerklePatriciaTrie {db: HashMap::new(), root: String::new()};
        mpt.db.insert(String::new(), Node::Null());
        mpt
    }

    fn clone(&self) -> MerklePatriciaTrie {
        let mut hashmap = HashMap::new();
        for (k, v) in self.db.iter() {
            hashmap.insert((*k).to_string(), (*v).clone());
        }
        MerklePatriciaTrie {db: hashmap, root: self.root.to_string()}
    }
    fn mpt_to_string(&self) -> String {
        let mut content: String = String::new();
        content = content + &*format!("ROOT={}\n", self.root);
        for (hash, node) in &self.db {
            content = content + &*format!("{}: {:?}\n", hash, node);
        }
        content
    }

    fn print(&self) {
        println!("{}", self.mpt_to_string());
    }

    fn order_nodes(&self) -> String {
        let raw_content = self.mpt_to_string();
        let content: Vec<&str> = raw_content.split("\n").collect();
        let mut queue: Vec<&str> = Vec::new();
        let mut temp1 = content[0];
        let mut temp2: Vec<&str> = temp1.split("HashStart").collect();
        temp1 = temp2[1];
        temp2 = temp1.split("HashEnd").collect();
        temp1 = temp2[0];
        queue.push(temp1);
        let mut i = -1;
        let mut rs = String::new();
        while let Some(cur_hash) = queue.pop() {
            i += 1;
            println!("cur={}", cur_hash);
            let mut line: &str = "";
            for each in &content {
                if each.starts_with(&*(format!("HashStart{}HashEnd", cur_hash))) {
                    temp2 = each.split("HashEnd: ").collect();
                    line = temp2[1];
                    rs = rs + each + "\n";
                    rs = rs.replace(&*(format!("HashStart{}HashEnd", cur_hash)), &*(format!("Hash{}", i)));
                }
            }
            temp2 = line.split("HashStart").collect();
            let mut flag = true;
            for each in temp2 {
                if flag {
                    flag = false;
                    continue
                }
                let temp3: Vec<&str> = each.split("HashEnd").collect();
                queue.push(temp3[0]);
            }
        }
        rs
    }
    fn create_ext_node_from_branch_index(&mut self, mut existing_leaf_nibbel: Vec<u8>, new_value: &str, mut new_leaf_nibbel: Vec<u8>, existing_value: &str, current_node_hash: &str ) -> String {
        let mut common_prefix = true;
        let mut ext_node_nibbel: Vec<u8> = Vec::new();

        while common_prefix {
            if !(existing_leaf_nibbel.len() == 0 || new_leaf_nibbel.len() == 0) {
                if existing_leaf_nibbel[0] == new_leaf_nibbel[0] {
                    ext_node_nibbel.push(new_leaf_nibbel[0]);
                    existing_leaf_nibbel = existing_leaf_nibbel[1..].to_vec();
                    new_leaf_nibbel = new_leaf_nibbel[1..].to_vec();
                } else {
                    common_prefix = false;
                }
            } else {
                common_prefix = false;
            }
        }

        let mut branch_array: [String; 17] = Default::default();

        if existing_leaf_nibbel.len() == 0 {
            println!("Existing node nibbel length =  {:?}", existing_leaf_nibbel.len());
            println!("Create branch node with value of the exsisting leaf node");
            branch_array[16] = existing_value.to_string();
            let leaf_node_zero_index = new_leaf_nibbel[0] as usize;
            new_leaf_nibbel.remove(0);
            new_leaf_nibbel.push(0x10);
            println!("Value of branch node = {:?}", existing_value.to_string());
            println!("Created leaf in branch index {:?} with value = {:?} and nibbel : {:?} ", leaf_node_zero_index, new_value.to_string(), new_leaf_nibbel);

            let leaf_node = Node::Flag((compact_encode(new_leaf_nibbel.clone()), new_value.to_string()));
            branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
            self.db.insert(hash_node(&leaf_node),leaf_node);

        } else if new_leaf_nibbel.len() == 0 {
            println!("New node nibbel length =  {:?}", new_leaf_nibbel.len());
            println!("Create branch node with value of the new leaf node");
            branch_array[16] =  new_value.to_string();
            let leaf_node_zero_index = existing_leaf_nibbel[0] as usize;
            existing_leaf_nibbel.remove(0);
            existing_leaf_nibbel.push(0x10);
            println!("Value of branch node = {:?}", new_value.to_string());
            println!("Created leaf in branch index {:?} with value = {:?} and nibbel : {:?} ", leaf_node_zero_index, existing_value.to_string(), existing_leaf_nibbel);
            let leaf_node = Node::Flag((compact_encode(existing_leaf_nibbel), existing_value.to_string()));
            branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
            self.db.insert(hash_node(&leaf_node),leaf_node);
        } else {
            println!("Both new node and existing node has more nibbels -> creating two nodes and branch");

            let mut leaf_node_zero_index = existing_leaf_nibbel[0] as usize;
            existing_leaf_nibbel.remove(0);
            existing_leaf_nibbel.push(0x10);
            println!("In to encode: {:?}", existing_leaf_nibbel );
            println!("Created leaf node with nibbel = {:?}", (compact_encode(existing_leaf_nibbel.clone())));
            let mut leaf_node = Node::Flag((compact_encode(existing_leaf_nibbel), existing_value.to_string()));

            println!("Inserting  leaf node to branch index = {:?}", leaf_node_zero_index);
            branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
            self.db.insert(hash_node(&leaf_node),leaf_node);

            leaf_node_zero_index = new_leaf_nibbel[0] as usize;
            new_leaf_nibbel.remove(0);
            new_leaf_nibbel.push(0x10);
            println!("Created node with nibbel = {:?}", new_leaf_nibbel);
            println!("Inserting node to branch index = {:?}", leaf_node_zero_index);
            leaf_node = Node::Flag((compact_encode(new_leaf_nibbel.clone()), new_value.to_string()));
            branch_array[leaf_node_zero_index] = hash_node(&leaf_node);
            self.db.insert(hash_node(&leaf_node),leaf_node);
        }
        println!("{:?}", branch_array);
        //self.db.remove(current_node_hash);
        let branch_node = Node::Branch(branch_array);
        let branch_hash = hash_node(&branch_node);


        if ext_node_nibbel.len() != 0 {
            println!("Created extension node with nibbel = {:?}", ext_node_nibbel);
            let exstention_nibbel = compact_encode(ext_node_nibbel.clone());
            let exstention_node = Node::Flag((exstention_nibbel, hash_node(&branch_node)));
            let exstention_node_hash = hash_node(&exstention_node);
            self.db.insert(exstention_node_hash.clone(), exstention_node);
            self.db.insert(branch_hash.clone(),branch_node);
            if current_node_hash == self.root {
                println!("Root changed to new exstention node");
                self.root = exstention_node_hash.clone();
            }
            return exstention_node_hash;

        }else {
            println!("Created a branch node");
            self.db.insert(branch_hash.clone(),branch_node);
            if current_node_hash == self.root {
                println!("Root changed to new Branch node");
                self.root = branch_hash.clone();
            }
            return branch_hash;

        }

    }

    fn recusive_insert(&mut self, key: &Vec<u8>, new_value: &str, next_node_hash: &str) -> String {
        println!("--------------------RECUSIVE INSERT METHOD HER!!!--------------------");
        let real_node = self.db.get_mut(next_node_hash).unwrap();
        let node = create_node_copy(real_node);
        let mut return_value_hash =  String::from("");
        let mut nibbel_key = key.clone();
        let mut node_hash_ref_value_copy = String::from("");
        let mut call_recursive = false;

        match node {
            Node::Branch(branch) => {
                println!("I found a branch ");
                let mut branch_copy = branch.clone();
                let zero_index = nibbel_key[0] as usize;
                if branch[zero_index] == "" {
                    println!("There is an empty slot for me at index; {:?}", zero_index);

                    let mut nibbel_key:Vec<u8> = nibbel_key[1..].to_vec();
                    nibbel_key.push(0x10);
                    println!("Creating leaf with nibbel {:?} and value {:?}", nibbel_key, new_value.to_string() );

                    let leaf_node = Node::Flag((compact_encode(nibbel_key), new_value.to_string()));

                    branch_copy[zero_index] = hash_node(&leaf_node);
                    println!("Branch copy= {:?}", branch_copy);
                    self.db.insert(hash_node(&leaf_node),leaf_node);

                    println!("Branch had space at index {:?}, insert new leaf leaf, updating my hash and return ", zero_index);
                    let branch_node = Node::Branch(branch_copy);
                    let branch_hash = hash_node(&branch_node);
                    // TODO self.db.remove(&hash_node(&Node::Branch(branch)));
                    self.db.insert(branch_hash.clone(),branch_node);
                    if next_node_hash == self.root {
                        self.root = branch_hash.clone();
                    }
                    return branch_hash;
                } else {
                    println!("Index is not empty {:?} ", zero_index);
                    let nibbel_key_copy = vector_front_appender(nibbel_key.clone(), zero_index as u8);
                    let leaf_node_copy = create_node_copy(self.db.get_mut(&branch[zero_index]).unwrap());
                    let leaf_node_copy_hash = hash_node(&leaf_node_copy);
                    match leaf_node_copy {
                        Node::Branch(branch) => {},
                        Node::Flag((encoded_prefix, value)) => {
                            let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()))[0];

                            if first_nibbel == 2 ||  first_nibbel == 3 {
                                println!("The node stored in this index is a leaf");
                                let mut encoded_prefix_copy = encoded_prefix.clone();
                                encoded_prefix_copy = compact_decode(encoded_prefix_copy);
                                println!("nibbel of leaf {:?}", encoded_prefix_copy);

                                let mut updated_branch_array = branch_copy.clone();

                                nibbel_key = nibbel_key[1..].to_vec();
                                println!("Updating branch index:{:?} ", zero_index);
                                // self.db.remove(&hash_node(&Node::Branch(branch_copy)));

                                updated_branch_array[zero_index] = Self::create_ext_node_from_branch_index(self, encoded_prefix_copy, new_value, nibbel_key, &value, &leaf_node_copy_hash);

                                let branch_node = Node::Branch(updated_branch_array);
                                let branch_hash = hash_node(&branch_node);
                                println!("Removed old branch");
                                println!("Insert new branch");

                                self.db.insert(branch_hash.clone(), branch_node);

                                if next_node_hash == self.root {
                                    println!("Branch is root so update");

                                    self.root = branch_hash.clone()
                                }
                                //TODO remove old branch node hash
                                return branch_hash;
                            } else {
                                println!("The node stored in this index is a exstention node");
                                nibbel_key = nibbel_key[1..].to_vec();
                                println!("Call recursive with nibbel: {:?}", nibbel_key);
                                branch_copy[zero_index] = Self::recusive_insert(self, &nibbel_key.clone(), new_value, &leaf_node_copy_hash);
                                //Todo remove old hash
                                let updated_branch_node = Node::Branch(branch_copy);
                                let updated_branch_hash = hash_node(&updated_branch_node);
                                self.db.insert(updated_branch_hash.clone(),updated_branch_node);
                                return updated_branch_hash;

                            }
                        },
                        Node::Null() => {}
                    }
                }
            },
            Node::Flag((encoded_prefix, value)) => {
                let encoded_prefix_copy = encoded_prefix.clone();
                let value_copy = value.clone();
                let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix_copy.clone()))[0];
                let mut current_node_nibbel_decoded =  compact_decode(encoded_prefix.clone());
                let current_node_hash = hash_node(&Node::Flag((compact_encode(current_node_nibbel_decoded.clone()), value_copy.clone())));
                node_hash_ref_value_copy = value.clone();

                //Cheack if leaf node
                if first_nibbel == 2 || first_nibbel == 3 {
                    println!("Found leaf node with nibbel: {:?}", current_node_nibbel_decoded);
                    println!("Can we add exstender  {:?} = {:?}", current_node_nibbel_decoded[0],key[0]);
                    if current_node_nibbel_decoded[0] == key[0]{
                        return_value_hash = Self::create_ext_node_from_branch_index(self, current_node_nibbel_decoded, new_value, nibbel_key, &value, &current_node_hash);
                        if next_node_hash == self.root {
                            self.root = return_value_hash.clone();
                        }
                    } else {
                        println!("They are not matching so I need an branch for them");

                        let mut branch_array: [String; 17] = Default::default();

                        if current_node_nibbel_decoded.len() == 0 {
                            println!("The current node has no more nibbels left and must be stored in branch:{:?} ", value);

                            branch_array[16] = value_copy.to_string(); //Put value in branch value
                            let node_copy = Node::Flag((compact_encode(current_node_nibbel_decoded.clone()), value_copy.to_string()));

                            let current_node_hash = hash_node(&node_copy);
                            // self.db.remove(&current_node_hash); //Remove old leaf node


                        }else {
                            let current_node_branch_extender = current_node_nibbel_decoded[0] as usize;
                            let node_copy = Node::Flag((compact_encode(current_node_nibbel_decoded.clone()), value_copy.to_string()));
                            let current_node_hash = hash_node(&node_copy);

                            // self.db.remove(&current_node_hash);
                            let mut current_node_new_prefix:Vec<u8> = current_node_nibbel_decoded[1..].to_vec();
                            current_node_new_prefix.push(0x10);

                            println!("Creating leaf wiht nibbel :{:?} and value {:?} ",current_node_new_prefix, value);

                            let leaf_node = Node::Flag((compact_encode(current_node_new_prefix), value_copy.to_string()));
                            let leaf_node_hash = hash_node(&leaf_node);
                            branch_array[current_node_branch_extender] = leaf_node_hash.clone();
                            println!("Adding leaf to branch index {:?} ",current_node_branch_extender);
                            self.db.insert(leaf_node_hash, leaf_node);


                            let new_node_branch_extender = key[0] as usize;
                            let mut new_node_prefix:Vec<u8> = key[1..].to_vec();
                            new_node_prefix.push(0x10);
                            println!("Creating leaf wiht nibbel :{:?} and value {:?} ",new_node_prefix, new_value);

                            let new_leaf_node = Node::Flag((compact_encode(new_node_prefix), new_value.to_string()));
                            let new_leaf_node_hash = hash_node(&new_leaf_node);
                            branch_array[new_node_branch_extender] = new_leaf_node_hash.clone();
                            println!("Adding leaf to branch index {:?} ",new_node_branch_extender);

                            self.db.insert(new_leaf_node_hash, new_leaf_node);

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
                    println!("Insert trough an exstention node");
                    println!("nibbel is now:{:?} ", nibbel_key);

                    let mut full_match = true;
                    let mut nibbel_key_copy = nibbel_key.clone();
                    let mut i = 0;
                    //Todo make sure that nibbel_key is longer than current else we need to exstend
                    for c in current_node_nibbel_decoded.clone() {
                        if nibbel_key[i] != c {
                            full_match = false;
                            nibbel_key_copy.remove(0);
                        }
                        i = i + 1;
                    }

                    if full_match {
                        println!("Full match call recursive");
                        nibbel_key_copy = nibbel_key_copy[i..].to_vec();

                        nibbel_key = nibbel_key_copy;
                        println!("nibbel is now:{:?} ", nibbel_key);

                        let mut return_value = Self::recusive_insert(self, &nibbel_key.clone(), new_value, &node_hash_ref_value_copy);
                        println!("Now the extension node has to update");

                        let updated_node = Node::Flag((encoded_prefix_copy, return_value));
                        let updated_node_hash = hash_node(&updated_node);
                        if current_node_hash == self.root {
                            println!("Exstenstion node is root -> update root");
                            println!("{:?}",updated_node_hash);

                            self.root = updated_node_hash.clone();
                        }
                        self.db.insert(updated_node_hash.clone(), updated_node);
                        // self.db.remove(&current_node_hash);

                        return updated_node_hash
                    } else {
                        if current_node_nibbel_decoded[0] == nibbel_key[0]{

                            let updated_ext_node = Self::create_ext_node_from_branch_index(self, current_node_nibbel_decoded, new_value, nibbel_key, &value, &current_node_hash);

                            if self.root == current_node_hash {
                                self.root = updated_ext_node.clone();
                            }

                            return updated_ext_node;
                        } else {
                            println!("No partal match with ext node");
                            let mut branch_array: [String; 17] = Default::default();
                            let new_zero_index = nibbel_key[0] as usize;
                            let ext_node_zero_index = current_node_nibbel_decoded[0] as usize;
                            let mut ext_node_nibbel = current_node_nibbel_decoded;
                            nibbel_key.remove(0);
                            ext_node_nibbel.remove(0);
                            if ext_node_nibbel.len() == 0 {
                                println!("Ext node is empty add value too branch index: {:?}  ", ext_node_zero_index);
                                branch_array[ext_node_zero_index] = value.clone();
                                // self.db.remove(&current_node_hash);

                            } else {
                                println!("Creating new Ext node with nibbel: {:?} at index {:?}", ext_node_nibbel, ext_node_zero_index);
                                let updated_ext_node = Node::Flag((compact_encode(ext_node_nibbel), value.clone()));
                                let updated_ext_node_hash = hash_node(&updated_ext_node);
                                //  self.db.remove(&current_node_hash);

                                self.db.insert(updated_ext_node_hash.clone(), updated_ext_node);
                                branch_array[ext_node_zero_index] = updated_ext_node_hash;
                            }
                            nibbel_key.push(0x10);
                            println!("Creating leaf with nibbel: {:?} and value: {:?} ", nibbel_key,new_value);
                            let mut new_leaf_node = Node::Flag((compact_encode(nibbel_key),new_value.to_string()));
                            let mut new_leaf_node_hash = hash_node(&new_leaf_node);
                            branch_array[new_zero_index] = new_leaf_node_hash.clone();
                            self.db.insert(new_leaf_node_hash, new_leaf_node);
                            println!("Creating branch");
                            let mut new_branch = Node::Branch(branch_array);

                            let new_branch_hash = hash_node(&new_branch);
                            self.db.insert(new_branch_hash.clone(), new_branch);
                            println!("{:?}", current_node_hash);
                            println!("{:?}", new_branch_hash);

                            if current_node_hash == self.root {
                                self.root = new_branch_hash.clone();
                            }

                            return new_branch_hash;
                        }
                    }
                }
            },
            Node::Null() => {
            }
        }

        return String::from("ss")
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
            println!("Inserted new node with path: {:?} and key: {:?}",new_node_prefix, key);

            Self::recusive_insert(self, &new_node_prefix, new_value, &self.root.clone());
        }
    }
    fn delete(&mut self, key: &str) -> String {
        let key_nibbels = string_to_vec_u8(&ascii_to_hex(string_to_ascii(&key)));
        println!("-- Delete key {:?} with nibbels: {:?} -- ", key, key_nibbels);
        let root = Self::recursive_delete(self, key_nibbels, &self.root.clone());
        println!("Back to delete");
        if root != "path_not_found" {
            println!("update root hash to : {:?}", root);
            self.root = root.clone();
        }
        return root;
    }

    fn check_if_empty_ext(node: &Node) -> String {
        match node {
            Node::Branch(branch) => {
                return String::from("");
            },
            Node::Flag((encoded_prefix, value)) => {
                let nibbels = compact_decode(encoded_prefix.clone());
                if nibbels.len() == 0 {
                    return value.to_string();
                }
                return String::from("");
            },
            Node::Null() => {
                return String::from("");
            }
        }

    }

    fn recursive_delete(&mut self, mut key: Vec<u8>, next_node_hash: &str) -> String {
        let mut node = self.db.get(next_node_hash).clone().unwrap();
        match node {
            Node::Branch(branch) => {
                println!("Found branch");

                let mut branch_copy = branch.clone();

                if key.len() == 0 {
                    println!("Key has no more nibbels.");
                    //TODO make into function
                    if branch_copy[16] == ""{
                        return String::from("path_not_found");
                    } else {
                        println!("Value stored in branch");
                        println!("{:?}", branch_copy[16]);
                        branch_copy[16] = String::from("");

                        let mut stored_hashes = 0;
                        let mut stored_hash_index = 0;
                        let mut i = 0;
                        for value in &branch_copy {
                            i = i + 1;
                            if value != ""{
                                stored_hash_index = i;
                                stored_hashes = stored_hashes + 1;
                            }
                        }
                        // If branch stores more than 1 node
                        if stored_hashes > 1 {
                            println!("Branch array > 1");
                            println!("Update branch array and return");

                            let updated_branch = Node::Branch(branch_copy);
                            self.db.remove(next_node_hash);
                            //TODO remove branch
                            let updated_branch_hash = hash_node(&updated_branch);
                            self.db.insert(updated_branch_hash.clone(),updated_branch);
                            return updated_branch_hash;
                        } else {
                            println!("Branch array <= 1");
                            println!("Update remove branch array and return");
                            let next_node_hash = &branch_copy[stored_hash_index];
                            let node_at_index = self.db.get(next_node_hash).clone().unwrap();

                            match node_at_index {
                                Node::Branch(branch) => {
                                    let mut new_ext_nibbel = Vec::new();
                                    new_ext_nibbel.push(stored_hash_index as u8);
                                    let new_ext_node = Node::Flag((compact_encode(new_ext_nibbel), next_node_hash.to_string()));
                                    let new_ext_node_hash = hash_node(&new_ext_node);
                                    self.db.insert(new_ext_node_hash.clone(), new_ext_node);
                                    //self.db.remove(&next_node_hash);
                                    //Todo remove old branch hash
                                    return new_ext_node_hash;
                                },
                                Node::Flag((encoded_prefix, value)) => {
                                    let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()))[0];
                                    let mut new_node_nibbel = compact_decode(encoded_prefix.clone());
                                    new_node_nibbel = vector_front_appender(new_node_nibbel, stored_hash_index as u8);
                                    if first_nibbel == 2 || first_nibbel == 3 {
                                        println!("Node stored on index {:?} was a leaf node", stored_hash_index);
                                        new_node_nibbel.push(0x10);
                                        println!("Creating new leaf with nibbel: {:?} and value {:?}", new_node_nibbel, value);
                                    } else {
                                        println!("Node stored on index {:?} was a ext node", stored_hash_index);
                                        println!("Creating new ext with nibbel: {:?}", new_node_nibbel);
                                    }
                                    let new_node = Node::Flag((compact_encode(new_node_nibbel), value.to_string()));
                                    let new_node_hash = hash_node(&new_node);
                                    self.db.insert(new_node_hash.clone(),new_node);
                                    // self.db.remove(&next_node_hash);
                                    //TODO remove old branch
                                    return new_node_hash;
                                },
                                Node::Null() => {
                                },
                            }
                        }
                    }
                } else {
                    // Todo After remove check the size of the branch
                    // If only index 16 then create leaf
                    // if only one index and its a leaf create leaf
                    // if its a branch then crete a ext with the index as nibbel
                    // if its a ext then create updated exstension
                    println!("Nibbels left of key...{:?}", key);

                    let branch_index_value = key[0] as usize;
                    let branch_index_hash_value  = branch_copy[branch_index_value].clone();
                    key.remove(0);
                    println!("Checking branch index; {:?}", branch_index_value);

                    if branch_index_hash_value != ""{
                        println!("Index was not empty: {:?}",branch_index_hash_value);

                        let node_at_index = self.db.get_mut(&branch_index_hash_value).unwrap();

                        match node_at_index {
                            Node::Branch(branch) => {
                                println!("Node stored at index is a branch");
                                let new_hash_for_index = Self::recursive_delete(self, key, &branch_index_hash_value);
                                if new_hash_for_index != "path_not_found" {
                                    let return_node = self.db.get(&new_hash_for_index).clone().unwrap();
                                    if Self::check_if_empty_ext(return_node) == "" {
                                        println!("The return was empty ext so remove it an get the value");
                                        branch_copy[branch_index_value] = Self::check_if_empty_ext(return_node);
                                    } else {
                                        branch_copy[branch_index_value] = new_hash_for_index;
                                    }
                                }
                                let mut updated_branch = Node::Branch(branch_copy);
                                let updated_branch_hash = hash_node(&updated_branch);
                                self.db.insert(updated_branch_hash.clone(),updated_branch);
                                //TODO remove old branch
                                return updated_branch_hash;
                            },
                            Node::Flag((encoded_prefix, value)) => {

                                let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()))[0];
                                let decoded_nibbel = compact_decode(encoded_prefix.clone());

                                if first_nibbel == 2 ||first_nibbel == 3 {
                                    println!("Node stored at index is a Leaf with value : {:?}", value);

                                    let mut i = 0;
                                    let mut full_match = true;
                                    for c in decoded_nibbel.clone() {
                                        if key[i] != c {
                                            full_match = false;
                                        }
                                        i = i + 1;
                                    }
                                    if full_match {
                                        println!("Leaf had full match");
                                        println!("branch array: {:?}",branch_copy);

                                        branch_copy[branch_index_value] = String::from("");
                                        println!("branch array: {:?}", branch_copy);

                                        self.db.remove(&branch_index_hash_value);
                                        let mut updated_branch = Node::Branch(branch_copy);
                                        let updated_branch_hash = hash_node(&updated_branch);
                                        self.db.insert(updated_branch_hash.clone(),updated_branch);
                                        //TODO remove old branch
                                        return updated_branch_hash;
                                    } else {
                                        return String::from("path_not_found");
                                    }
                                } else {
                                    println!("Node stored at index is a ext");
                                    let new_hash_for_index = Self::recursive_delete(self, key, &branch_index_hash_value);
                                    println!("Branch got return {:?}", new_hash_for_index);
                                    if new_hash_for_index == "path_not_found" {
                                        return String::from("path_not_found");

                                    } else {
                                        println!("Updating index {:?} to hold hash",branch_index_value );
                                        branch_copy[branch_index_value] = new_hash_for_index;
                                        let mut updated_branch = Node::Branch(branch_copy);
                                        let updated_branch_hash = hash_node(&updated_branch);
                                        self.db.insert(updated_branch_hash.clone(), updated_branch);
                                        //TODO remove old branch
                                        println!("Returning updated hash");
                                        return updated_branch_hash;
                                    }
                                }
                            },
                            Node::Null() => {}
                        }

                    } else {
                        return String::from("path_not_found");
                    }
                }

            },
            Node::Flag((encoded_prefix, value)) => {
                let mut node_nibbel = compact_decode(encoded_prefix.clone());
                println!("Found ext with nibbel {:?}", node_nibbel);

                let mut i = 0;
                let mut full_match = true;
                for c in node_nibbel.clone() {
                    if key[i] != c {
                        full_match = false;
                    }
                    i = i + 1;
                }
                key = key[i..].to_vec();
                if full_match {
                    println!("Key matched ext nibbels call recurisive : {:?}", key);
                    let new_hash_for_ext = Self::recursive_delete(self, key, &value.to_string());

                    if new_hash_for_ext == "path_not_found" {
                        return String::from("path_not_found");
                    }{
                        println!("EXT: some value deleted so I have to update Ext value");

                        let new_node = self.db.get(&new_hash_for_ext).clone().unwrap();
                        match new_node {
                            Node::Branch(branch) => {
                                println!("the value I hold is to a branch. My ext: {:?}", node_nibbel);
                                let new_ext_node = Node::Flag((compact_encode(node_nibbel),new_hash_for_ext));
                                let new_ext_node_hash = hash_node(&new_ext_node);
                                self.db.insert(new_ext_node_hash.clone(), new_ext_node);
                                return new_ext_node_hash;
                            },
                            Node::Flag((encoded_prefix, value)) => {
                                println!("the value I hold is to a Flag node. My ext: {:?}", node_nibbel);

                                let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()))[0];
                                if first_nibbel == 2 || first_nibbel == 3 {
                                    let mut new_nibbel_leaf = compact_decode(encoded_prefix.clone());
                                    for c in node_nibbel {
                                        new_nibbel_leaf = vector_front_appender(new_nibbel_leaf, c);
                                    }

                                    new_nibbel_leaf.push(0x10);
                                    let new_leaf_node = Node::Flag((compact_encode(new_nibbel_leaf), value.to_string()));
                                    //Todo remove old ext and old leaf
                                    let new_leaf_hash = hash_node(&new_leaf_node);
                                    self.db.insert(new_leaf_hash.clone(),new_leaf_node);
                                    return new_leaf_hash;
                                } else {
                                    let mut new_nibbel_ext = compact_decode(encoded_prefix.clone());
                                    for c in node_nibbel {
                                        new_nibbel_ext = vector_front_appender(new_nibbel_ext, c);
                                    }
                                    println!("the value I hold is to a ext. My ext: {:?}", new_nibbel_ext);

                                    let new_ext_node = Node::Flag((compact_encode(new_nibbel_ext), value.to_string()));
                                    let new_ext_hash = hash_node(&new_ext_node);
                                    self.db.insert(new_ext_hash.clone(),new_ext_node);
                                    return new_ext_hash;
                                }
                            },
                            Node::Null() => {
                            },
                        }
                    }
                } else {
                    return String::from("path_not_found");
                }


            },
            Node::Null() => {
            },
        }
        return String::from("");

    }

    fn get(&mut self, key: &str) -> String {
        println!("Seearch for key: {:?}", key.to_string());
        let mut key_prefix = string_to_vec_u8(&ascii_to_hex(string_to_ascii(&key)));
        let mut node_hash = self.root.clone();
        let mut done = false;
        let mut result = String::from("");

        while !done {
            let node = self.db.get_mut(&node_hash).unwrap();
            println!("Get node with hash {:?}", node_hash);

            match node {
                Node::Branch(branch) => {
                    println!("Found branch");
                    if key_prefix.len() == 0 {
                        println!("Found the value in branch {:?}", branch.len() );
                        return branch[16].clone();

                    }

                    if branch[key_prefix[0] as usize] != "" {
                        println!("Branch holds ref on index: {:?}", key_prefix[0]);
                        node_hash = branch[key_prefix[0] as usize].clone();

                        key_prefix = key_prefix[1..].to_vec();
                    }else {
                        println!("Branch has no hash ref stored at index: {:?}", key_prefix[0]);
                        result = String::from("");
                        done = true;
                    }
                },
                Node::Flag((encoded_prefix, value)) => {
                    println!("Its a ");
                    let first_nibbel = string_to_vec_u8(&ascii_to_hex(encoded_prefix.clone()))[0];
                    let node_prefix_decoded =  compact_decode(encoded_prefix.clone());
                    println!("Its a {:?} ", first_nibbel);

                    println!("Mathc with: {:?} : {:?}", node_prefix_decoded, key_prefix);

                    if first_nibbel == 2 || first_nibbel == 3 {
                        println!("Found leaf node.");

                        if node_prefix_decoded == key_prefix {
                            println!("Got full match for the rest of the nibbel!!");
                            result = value.clone();
                            done = true;

                        }else {
                            println!("The rest of the nibbel did not have full match with leaf nibbel");
                            result = String::from("");
                            done = true;
                        }
                    }else {
                        println!("Found ext node.");
                        println!("Nibbels left: {:?}", key_prefix);
                        println!("Nibbels ext left: {:?}", node_prefix_decoded);

                        if key_prefix.len() == 0 || node_prefix_decoded.len() == 0 {
                            break;
                        }
                        for c in node_prefix_decoded {
                            println!("Mathc prefix 0 ==  {:?} : {:?}", c, key_prefix[0]);

                            if c == key_prefix[0] {
                                key_prefix.remove(0);
                                node_hash = value.clone();

                            } else {
                                println!("Nibbel of ext node with value: {:?} did not match nibbel of key:{:?}",c,key_prefix[0]);
                                result = String::from("");
                                done = true;
                                break;
                            }

                        }

                        // println!("loop round two with hash: {:?} ", node_hash);
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

    for val in encoded_arr {
        let mut value = val/16;
        result.push(value);
        value = val%16;
        result.push(value);
    }

    if result[0] == 0 || result[0] == 2 {
        result.remove(0);
        result.remove(0);
    } else {
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

fn is_ext_node(encoded_arr: Vec<u8>) -> bool {
    (encoded_arr[0] as i32) / 16 < 2
}

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

fn empty_branch_value() -> [String; 17] {
    [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(), String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(), String::new(), String::new()]
}