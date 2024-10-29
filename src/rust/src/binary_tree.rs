type Link<T> = Option<Box<BinaryTree<T>>>;

#[derive(Clone, Debug)]
pub struct BinaryTree<T: Clone> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BinaryTree<T>
where
    T: Clone,
{
    /// 创建新的BinaryTree
    pub fn new(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }

    /// 新的子节点作为根节点的左子节点
    ///
    /// # Arguments
    /// - key: 新子节点的值
    pub fn insert_left_tree(&mut self, key: T) {
        self.left = if self.left.is_none() {
            let node = BinaryTree::new(key);
            BinaryTree::create_link(node)
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            BinaryTree::create_link(node)
        };
    }
    /// 新的子节点作为根节点的右子节点
    ///
    /// # Arguments
    /// - key: 新子节点的值
    pub fn insert_right_tree(&mut self, key: T) {
        self.right = if self.right.is_none() {
            let node = BinaryTree::new(key);
            BinaryTree::create_link(node)
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            BinaryTree::create_link(node)
        };
    }
}

impl<T> BinaryTree<T>
where
    T: Clone,
{
    pub fn get_key(&self) -> &T {
        &self.key
    }
    pub fn set_key(&mut self, key: T) {
        self.key = key;
    }
    pub fn get_left(&self) -> Link<T> {
        self.left.clone()
    }
    pub fn get_left_mut(&mut self) -> Option<&mut Box<BinaryTree<T>>> {
        self.left.as_mut()
    }
    pub fn get_right(&self) -> Link<T> {
        self.right.clone()
    }
    pub fn get_right_mut(&mut self) -> Option<&mut Box<BinaryTree<T>>> {
        self.right.as_mut()
    }
    fn create_link(node: BinaryTree<T>) -> Link<T> {
        Some(Box::new(node))
    }
}
