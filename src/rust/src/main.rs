use binary_tree::BinaryTree;

mod binary_tree;
fn main() {
    run_binary_tree();
}

fn run_binary_tree() {
    let mut bt = BinaryTree::new("a");
    let root = bt.get_key();
    println!("root val is {}", root);
    let left = bt.get_left();
    println!("left child is {:#?}", left);
    let right = bt.get_right();
    println!("right child is {:#?}", right);
    bt.insert_left_tree("b");
    bt.insert_right_tree("e");
    let left = bt.get_left();
    println!("left child is {:#?}", left);
    let right = bt.get_right();
    println!("right child is {:#?}", right);
    println!("binary_tree is {:#?}", bt);
}
