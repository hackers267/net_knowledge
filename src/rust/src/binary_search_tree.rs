use std::{cmp::Ordering, fmt::Debug};

type Link<T, U> = Option<Box<BinarySearchTree<T, U>>>;

#[derive(Clone, Debug, Default)]
pub struct BinarySearchTree<T, U>
where
    T: Ord,
{
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}

impl<T, U> BinarySearchTree<T, U>
where
    T: Ord + Clone,
    U: Clone,
{
    /// 构建新的BinarySearchTree
    ///
    /// # 返回值
    /// 新构造的二叉搜索树
    pub fn new() -> Self {
        Self {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }
    /// 判断二叉搜索树是可为空
    ///
    /// # 返回值
    /// - true 为空
    /// - false 不为空
    pub fn is_empty(&self) -> bool {
        self.key.is_none()
    }
    /// 在二叉搜索树中插入新值
    ///
    /// # 参数
    /// - key 新值的键
    /// - value 新值的值
    pub fn insert(&mut self, key: T, value: U) {
        match &self.key {
            Some(ref k) if k == &key => self.val = Some(value),
            Some(ref k) if &key < k => match self.left.as_mut() {
                Some(node) => node.insert(key, value),
                None => {
                    let mut node = BinarySearchTree::new();
                    node.insert(key, value);
                    self.left = Some(Box::new(node));
                }
            },
            Some(ref k) if &key > k => match self.right.as_mut() {
                Some(node) => node.insert(key, value),
                None => {
                    let mut node = BinarySearchTree::new();
                    node.insert(key, value);
                    self.right = Some(Box::new(node));
                }
            },
            Some(_) => unreachable!(),
            None => {
                self.key = Some(key);
                self.val = Some(value);
            }
        }
    }
    /// 查找二叉搜索树中是否有某个键
    ///
    /// # 参数
    /// - key 需要查找的键
    ///
    /// # 返回值
    /// - true 找到该键
    /// - false 在二叉搜索树中没有该键
    pub fn search(&self, key: T) -> bool {
        match &self.key {
            Some(k) if k == &key => true,
            Some(k) if &key < k => matches!(&self.left,Some(node) if node.search(key)),
            Some(k) if &key > k => matches!(&self.right, Some(node) if node.search(key)),
            Some(_) => unreachable!(),
            None => false,
        }
    }
    /// 在二叉搜索树中查找某个键对应的值
    ///
    /// # 参数
    /// - key 需要查找的键
    ///
    /// # 返回值
    /// - None 在二叉搜索树中找不到该键对应的值
    /// - Some(&U) 在二叉搜索树中找到的值
    pub fn get(&self, key: T) -> Option<&U> {
        match &self.key {
            Some(k) => match k.cmp(&key) {
                Ordering::Less => match &self.right {
                    Some(node) => node.get(key),
                    None => None,
                },
                Ordering::Equal => self.val.as_ref(),
                Ordering::Greater => match &self.left {
                    Some(node) => node.get(key),
                    None => None,
                },
            },
            None => None,
        }
    }
    /// 删除二叉搜索树中的指定项
    ///
    /// # 参数
    /// - key 需要删除的键
    ///
    /// # 返回值
    /// - None 无法删除，因为没有找到对应的key
    /// - Some(&U) 删除成功，并返回删除的key对应的值
    pub fn delete(&mut self, key: T) -> Option<U> {
        match &self.key {
            Some(k) => match key.cmp(k) {
                Ordering::Less => match self.left.as_mut() {
                    Some(node) => node.delete(key),
                    None => None,
                },
                Ordering::Equal => {
                    let val = self.val.take();
                    match self.left.as_mut() {
                        Some(node) => {
                            let (next_key, next_val) = node.max();
                            let key = next_key.unwrap().clone();
                            let val = next_val.unwrap().clone();
                            self.key = Some(key.clone());
                            self.val = Some(val);
                            node.delete(key);
                        }
                        None => {
                            if let Some(node) = self.right.as_mut() {
                                let (next_key, next_val) = node.min();
                                let key = next_key.unwrap().clone();
                                let val = next_val.unwrap().clone();
                                self.key = Some(key.clone());
                                self.val = Some(val);
                                node.delete(key);
                            }
                        }
                    };
                    val
                }
                Ordering::Greater => match self.right.as_mut() {
                    Some(node) => node.delete(key),
                    None => None,
                },
            },
            None => None,
        }
    }
}

/// 遍历方法
pub trait Order {
    /// 前序遍历
    fn preorder(&self);
    /// 中序遍历
    fn inorder(&self);
    /// 后序遍历
    fn postorder(&self);
}

impl<T, U> Order for BinarySearchTree<T, U>
where
    T: Ord + Debug,
    U: Debug,
{
    /// 前序遍历
    fn preorder(&self) {
        println!("key is {:?} and val is {:?}", self.key, self.val);
        if self.left.is_some() {
            self.left.as_ref().unwrap().preorder();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().preorder();
        }
    }

    /// 中序遍历
    fn inorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().inorder();
        }
        println!("key is {:?} and val is {:?}", self.key, self.val);
        if self.right.is_some() {
            self.right.as_ref().unwrap().inorder();
        }
    }

    /// 后序遍历
    fn postorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().postorder();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().postorder();
        }
        println!("key is {:?} and val is {:?}", self.key, self.val)
    }
}

/// 最大最小值
pub trait MaxMin<T, U>
where
    T: Ord,
{
    /// 获取最大值
    ///
    /// # 返回值
    /// 没有最大值为`(None,None)`,否则为`(Some(&T),Some(&U))`
    fn max(&self) -> (Option<&T>, Option<&U>);
    /// 获取最小值
    ///
    /// # 返回值
    /// 没有最小值为`(None,None)`,否则为`(Some(&T),Some(&U))`
    fn min(&self) -> (Option<&T>, Option<&U>);
}

impl<T, U> MaxMin<T, U> for BinarySearchTree<T, U>
where
    T: Ord,
{
    /// 获取最大值
    ///
    /// # 返回值
    /// 没有最大值为`(None,None)`,否则为`(Some(&T),Some(&U))`
    fn max(&self) -> (Option<&T>, Option<&U>) {
        match &self.right {
            Some(node) => node.max(),
            None => match &self.key {
                Some(k) => (Some(k), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    /// 获取最小值
    ///
    /// # 返回值
    /// 没有最小值为`(None,None)`,否则为`(Some(&T),Some(&U))`
    fn min(&self) -> (Option<&T>, Option<&U>) {
        match &self.left {
            Some(node) => node.min(),
            None => match &self.key {
                Some(k) => (Some(k), self.val.as_ref()),
                None => (None, None),
            },
        }
    }
}
