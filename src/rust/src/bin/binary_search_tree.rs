use demo::binary_search_tree::{BinarySearchTree, MaxMin, Order};

fn main() {
    let mut bst = BinarySearchTree::new();
    bst.insert(5, 5);
    bst.insert(4, 4);
    bst.insert(6, 6);
    bst.insert(7, 7);
    bst.insert(3, 3);
    println!("binary search tree is {:#?}", bst);
    println!("前序遍历的结果是:");
    bst.preorder();
    println!("中序遍历的结果是:");
    bst.inorder();
    println!("后序遍历的结果是:");
    bst.postorder();
    println!("最大值是:{:?}", bst.max());
    println!("最小值是:{:?}", bst.min());
    println!("在二叉搜索树bst中找到6:{}", bst.search(6));
    println!("在二叉搜索树bst中找到8:{}", bst.search(8));
    println!("在二叉搜索树bst中找到2:{}", bst.search(2));
    println!("在二叉搜索树bst中找到键6的值为:{:?}", bst.get(6));
    println!("在二叉搜索树bst中找到键8的值为:{:?}", bst.get(8));
    println!("在二叉搜索树bst中找到键2的值为:{:?}", bst.get(2));
    bst.delete(5);
    println!("binary search tree is {:#?}", bst);
    bst.delete(8);
    println!("binary search tree is {:#?}", bst);
}
