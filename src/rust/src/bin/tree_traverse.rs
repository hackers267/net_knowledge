use demo::binary_tree::BinaryTree;

fn main() {
    run_order();
}
fn run_order() {
    let mut bt = BinaryTree::new('a');
    bt.insert_left_tree('b');
    bt.insert_right_tree('c');
    bt.get_left_mut().unwrap().insert_left_tree('d');
    bt.get_left_mut().unwrap().insert_right_tree('e');
    bt.get_right_mut().unwrap().insert_left_tree('f');
    bt.get_right_mut().unwrap().insert_right_tree('g');
    println!("binary_tree is {:#?}", bt);
    println!("preorder result is");
    bt.preorder();
    println!("inorder result is");
    bt.inorder();
    println!("postorder result is");
    bt.postorder();
}
