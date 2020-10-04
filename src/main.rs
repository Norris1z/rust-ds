mod binary_tree;
use binary_tree::BinaryTree;

fn main() {
    let head = 9324;
    let numbers = vec![
        101, 319, 395, 450, 603, 746, 841, 1431, 1474, 1845, 1913, 2214, 2322, 2324, 2624, 2792,
        2859, 2922, 3712, 4032, 4076, 4243, 9, 4379, 5043, 9, 5360, 5426, 5504, 6462, 6559, 6569,
        6599, 6862, 7102, 7216, 7318, 9, 603, 7403, 7431, 7900, 7930, 8200, 8359, 8602, 8808, 8850,
        8998, 9018, 9324, 9475, 9812, 9867, 603, 9, 10000000,
    ];
    let mut tree = BinaryTree::new(&head);
    numbers.iter().for_each(|number: &u32| {
        tree.insert_node(number);
    });
    let value = 9;
    println!("{:?}", tree.get_value_count(&value));
    println!("{:?}", tree.to_vec_ascending());
    println!("{:?}", tree.to_vec_descending());
}
