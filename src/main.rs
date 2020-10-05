mod huffman;
mod random;
use huffman::{HuffmanTree, TreeItem};
use std::collections::HashMap;

fn get_char_counts(data: &String) -> HashMap<char, u32> {
    let mut output: HashMap<char, u32> = HashMap::new();
    for s in data.chars() {
        if let Some(entry) = output.get_mut(&s) {
            *entry += 1;
        } else {
            output.insert(s, 1);
        }
    }
    output
}

fn main() {
    let text = random::text().to_owned();
    let char_counts = get_char_counts(&text);
    let mut data: Vec<TreeItem<char>> = vec![];
    for (key, val) in char_counts.iter() {
        data.push(TreeItem::new(key, *val));
    }
    let mut tree = HuffmanTree::new(&data);
    tree.build();
    println!("Original Data Size: {} bits", tree.original_data_size());
    println!("Encoded Data Size: {} bits", tree.encoded_data_size());
}
