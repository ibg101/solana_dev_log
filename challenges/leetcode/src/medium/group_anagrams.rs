use std::collections::HashMap;


pub struct Solution;

impl Solution {
    pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map: HashMap<String, Vec<String>> = HashMap::with_capacity(strings.len());
        
        for string in strings {
            // keeping initial string by making a deep copy
            let mut bytes_vec: Vec<u8> = string.clone().into_bytes(); 
            bytes_vec.sort();
            let key: String = unsafe { String::from_utf8_unchecked(bytes_vec) };

            if let Some(v) = hash_map.get_mut(&key) {
                v.push(string); 
            } else {
                hash_map.insert(key, vec![string]);
            }
        }
        
        hash_map.into_values().collect()
    }
}