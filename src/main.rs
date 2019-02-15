include!("merkle_patricia_trie.rs");

use std::fmt;
use std::io;
use std::io::{Read, Write};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    test_mpt_ext();
    test_mpt_leaf();
    test_mpt_delete_basic();
    test_mpt_branch();
    test_mpt_leaf_basic();
    test_mpt_ext_basic();
}

fn test_mpt_ext() {
    let mut mpt = MerklePatriciaTrie::new();
    let mut inserted_trie = String::new();
    let mut v = String::new();
    mpt = MerklePatriciaTrie::new();
    mpt.insert("p", "apple");
    mpt.insert("aa", "banana");
    mpt.insert("ap", "orange");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_011.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_011.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);
    v = mpt.get("aa");
    assert_eq!(v, "banana");

    mpt = MerklePatriciaTrie::new();
    mpt.insert("p", "apple");
    mpt.insert("aa", "banana");
    mpt.insert("ap", "orange");
    inserted_trie = mpt.order_nodes();
    mpt.insert("ba", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_013.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_013.txt"));
    mpt.delete("ba");
    assert_eq!(mpt.order_nodes(), inserted_trie);
    v = mpt.get("p");
    assert_eq!(v, "apple");

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aaa", "apple");
    mpt.insert("aap", "banana");
    mpt.insert("bb", "right leaf");
    inserted_trie = mpt.order_nodes();
    mpt.insert("aa", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_030.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_030.txt"));
    mpt.delete("aa");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("p", "apple");
    mpt.insert("aaa", "banana");
    mpt.insert("aap", "orange");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_031.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_031.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("p", "apple");
    mpt.insert("aaa", "banana");
    mpt.insert("aap", "orange");
    inserted_trie = mpt.order_nodes();
    mpt.insert("ba", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_033.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_033.txt"));
    mpt.delete("ba");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aa", "apple");
    mpt.insert("ap", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_111.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_111.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aa", "apple");
    mpt.insert("ap", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("bc", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_113.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_113.txt"));
    mpt.delete("bc");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("p", "apple");
    mpt.insert("aaaa", "banana");
    mpt.insert("aaap", "orange");
    inserted_trie = mpt.order_nodes();
    mpt.insert("a", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_140.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_140.txt"));
    mpt.delete("a");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aaa", "apple");
    mpt.insert("aap", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_131.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_131.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aaa", "apple");
    mpt.insert("aap", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("bc", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_133.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_133.txt"));
    mpt.delete("bc");
    assert_eq!(mpt.order_nodes(), inserted_trie);
}

fn test_mpt_leaf() {
    let mut inserted_trie = String::new();
    let mut mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("b", "banana");
    mpt.insert("a", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_000.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_000.txt"));
    mpt.delete("a");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/delete_basic_1.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("b", "banana");
    mpt.insert("ab", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_002.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_002.txt"));
    mpt.delete("ab");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/basic_0.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("p", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_011.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_011.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("p", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("bc", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_013.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_013.txt"));
    mpt.delete("bc");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("bab", "apple");
    mpt.insert("aa", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_040.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_040.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aab", "apple");
    mpt.insert("app", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("ac", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_031.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_031.txt"));
    mpt.delete("ac");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aab", "apple");
    mpt.insert("app", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("ace", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_033.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_033.txt"));
    mpt.delete("ace");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("p", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("a", "apple");
    mpt.insert("a", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_100.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_100.txt"));
    mpt.delete("a");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("p", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("abc", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_104.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_104.txt"));
    mpt.delete("abc");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_111.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_111.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    inserted_trie = mpt.order_nodes();
    mpt.insert("bc", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_113.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_113.txt"));
    mpt.delete("bc");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("ap", "apple");
    inserted_trie = mpt.order_nodes();
    mpt.insert("b", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_131.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_131.txt"));
    mpt.delete("b");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("ap", "apple");
    inserted_trie = mpt.order_nodes();
    mpt.insert("bp", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_133.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/leaf_133.txt"));
    mpt.delete("bp");
    assert_eq!(mpt.order_nodes(), inserted_trie);
}

fn test_mpt_delete_basic() {
    let mut mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("b", "banana");
    mpt.delete("a");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/delete_basic_1.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("b", "banana");
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/delete_basic_0.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aa", "apple");
    mpt.insert("abb", "banana");
    mpt.delete("a");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/delete_basic_2.txt"));
}

fn test_mpt_branch() {
    let mut mpt = MerklePatriciaTrie::new();
    let mut inserted_trie = String::new();
    mpt.insert("aa", "apple");
    mpt.insert("ap", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("a", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_nv_np.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_nv_np.txt"));
    mpt.delete("a");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "old");
    mpt.insert("aa", "apple");
    mpt.insert("ap", "banana");
    mpt.insert("a", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_v_np.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_v_np.txt"));
    mpt.delete("a");
    let mut expected_mpt = MerklePatriciaTrie::new();
    expected_mpt.insert("aa", "apple");
    expected_mpt.insert("ap", "banana");
    assert_eq!(mpt.order_nodes(), expected_mpt.order_nodes());

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("b", "banana");
    inserted_trie = mpt.order_nodes();
    mpt.insert("c", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_nv_p.txt"));
    mpt.delete("cc");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_nv_p.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), inserted_trie);

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aa", "apple");
    mpt.insert("ap", "banana");
    mpt.insert("a", "old");
    inserted_trie = mpt.order_nodes();
    mpt.insert("aA", "new");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_v_p.txt"));
    mpt.delete("c");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/branch_v_p.txt"));
    mpt.delete("aA");
    assert_eq!(mpt.order_nodes(), inserted_trie);
}

fn test_mpt_leaf_basic() {
    let mut mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("b", "banana");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/basic_0.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("p", "banana");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/basic_1.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/basic_2.txt"));
}

fn test_mpt_ext_basic() {
    let mut mpt = MerklePatriciaTrie::new();
    mpt.insert("a", "apple");
    mpt.insert("b", "banana");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_basic_1.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aa", "apple");
    mpt.insert("ap", "banana");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_basic_2.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aa", "apple");
    mpt.insert("ab", "banana");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_basic_3.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("aaa", "apple");
    mpt.insert("aap", "banana");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_basic_4.txt"));

    mpt = MerklePatriciaTrie::new();
    mpt.insert("p", "apple");
    mpt.insert("aa", "banana");
    mpt.insert("ap", "orange");
    assert_eq!(mpt.order_nodes(), read_file("src/mpt_tests/ext_basic_5.txt"));
}

fn read_file(path: &str) -> String {
    let mut f = File::open(path).expect("File not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("read failed");
    content
}