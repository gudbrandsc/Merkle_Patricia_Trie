//include!("merkle_patricia_trie.rs");
extern crate hex;
#[warn(unused_variables)]
fn main() {
   //let test1 = String::from_utf8(compact_encode(vec![1, 2, 3, 4, 5 ])).unwrap();
   // println!("Result: {}",test1 );
   // let test2 = String::from_utf8(compact_encode(vec![1, 6, 1])).unwrap();
   // println!("Result: {}",test2 );
   // println!("{:?}",compact_decode(vec![17, 97]));
    test_compact_encode();

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
fn get_u8_bytes(v: u32) -> (u8) {
    (v & 0x000000ff >>  0) as u8
}


// If Leaf, ignore 16 at the end
fn compact_decode(encoded_arr: Vec<u8>) -> Vec<u8> {

    let mut result : Vec<u8> = Vec::new();
    let mut i = 0;

    while i != encoded_arr.len() {
        let hex_val = format!("{:x}", encoded_arr[i]);

        for less_than_nine in hex_val.chars() {
            let current_value = less_than_nine.to_digit(16).unwrap();
            result.push(get_u8_bytes(current_value));
        }
        i = i + 1;
    }
    if result[0] != 0 {
        result.remove(0);
    }

    result


}


fn compact_encode(hex_array: Vec<u8>) -> Vec<u8> {
    let mut mut_hex_array:Vec<u8>  = hex_array.clone();

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
        mut_hex_array.insert(0, flags);
    } else {
        mut_hex_array.insert(0, flags);
        mut_hex_array.insert(1, 0);
    }

    let mut i = 0;
    let mut result : Vec<u8> = Vec::new();
    while i != mut_hex_array.len() {
        let value = 16 * mut_hex_array[i] + mut_hex_array[i + 1];
        result.push(value);

        i = i + 2;
    }
    return result;
}