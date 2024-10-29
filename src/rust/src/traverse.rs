use crate::binary_tree::BinaryTree;

impl<T> BinaryTree<T>
where
    T: Clone + std::fmt::Debug,
{
    /// 前序遍历
    pub fn preorder(&mut self) {
        println!("val is {:?}", &self.get_key());
        if self.get_left().is_some() {
            self.get_left().unwrap().preorder()
        }
        if self.get_right().is_some() {
            self.get_right().unwrap().preorder()
        }
    }
}

impl<T> BinaryTree<T>
where
    T: Clone + std::fmt::Debug,
{
    /// 后序遍历
    pub fn postorder(&mut self) {
        if self.get_left().is_some() {
            self.get_left().unwrap().postorder()
        }
        if self.get_right().is_some() {
            self.get_right().unwrap().postorder()
        }
        println!("val is {:?}", &self.get_key())
    }
}

impl<T> BinaryTree<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn inorder(&mut self) {
        if self.get_left().is_some() {
            self.get_left().unwrap().inorder()
        }
        println!("val is {:?}", &self.get_key());
        if self.get_right().is_some() {
            self.get_right().unwrap().inorder()
        }
    }
}
