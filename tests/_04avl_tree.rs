// 完成基本的二叉搜索树接口

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
    height: usize, // 新增：节点高度
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if let Some(ref mut root) = self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    /// AVL树插入入口
    pub fn insert_avl(&mut self, value: T) {
        self.root = match self.root.take() {
            Some(root) => Some(root.insert_avl(value)),
            None => Some(Box::new(TreeNode::new(value))),
        };
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        self.root.as_ref().map_or(false, |root| root.search(value))
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // 重复值处理: 这里我们选择不插入重复值
                // 也可以根据需求选择其他处理方式
            }
        }
    }

    // 辅助函数
    // 递归查找节点
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            Ordering::Greater => self
                .right
                .as_ref()
                .map_or(false, |right| right.search(value)),
            Ordering::Equal => true,
        }
    }

    /// AVL树专用插入方法
    /// 因为每次插入的平衡操作可能会消耗掉原根节点, 所以我们需要返回新的根节点
    fn insert_avl(mut self: Box<TreeNode<T>>, value: T) -> Box<TreeNode<T>> {
        // 1. 标准BST插入
        match value.cmp(&self.value) {
            Ordering::Less => {
                self.left = match self.left {
                    Some(left) => Some(left.insert_avl(value)),
                    None => Some(Box::new(TreeNode::new(value))),
                };
            }
            Ordering::Greater => {
                self.right = match self.right {
                    Some(right) => Some(right.insert_avl(value)),
                    None => Some(Box::new(TreeNode::new(value))),
                };
            }
            Ordering::Equal => return self, // 重复值不插入
        };

        // 2. 更新当前节点高度
        self.update_height();

        // 3. 平衡调整（自动处理4种旋转情况）
        self.balance()
    }

    /// 更新节点高度
    fn update_height(&mut self) {
        self.height = 1 + Self::height(&self.left).max(Self::height(&self.right));
    }

    /// 子树高度
    fn height(node: &Option<Box<TreeNode<T>>>) -> usize {
        node.as_ref().map_or(0, |n| n.height)
    }

    /// 平衡调整主逻辑
    fn balance(mut self: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let bf = self.balance_factor();
        // Left-Left 情况
        if bf > 1 && self.left.as_ref().unwrap().balance_factor() >= 0 {
            return self.rotate_right();
        }
        // Left-Right 情况
        if bf > 1 && self.left.as_ref().unwrap().balance_factor() < 0 {
            self.left = Some(self.left.take().unwrap().rotate_left());
            return self.rotate_right();
        }
        // Right-Right 情况
        if bf < -1 && self.right.as_ref().unwrap().balance_factor() <= 0 {
            return self.rotate_left();
        }
        // Right-Left 情况
        if bf < -1 && self.right.as_ref().unwrap().balance_factor() > 0 {
            self.right = Some(self.right.take().unwrap().rotate_right());
            return self.rotate_left();
        }

        // 无需旋转
        self
    }

    /// 右旋操作
    fn rotate_right(mut self: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut new_root = self.left.take().unwrap();
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(self);
        new_root.update_height();
        new_root
    }

    /// 左旋操作
    fn rotate_left(mut self: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut new_root = self.right.take().unwrap();
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(self);
        new_root.update_height();
        new_root
    }

    /// 计算平衡因子
    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) as i32 - Self::height(&self.right) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }

    #[test]
    fn test_avl_insert() {
        let mut avl = BinarySearchTree::new();
        avl.insert_avl(3);
        avl.insert_avl(2);
        avl.insert_avl(1); // 触发右旋
        assert_eq!(avl.root.as_ref().unwrap().value, 2);
        assert_eq!(avl.root.as_ref().unwrap().height, 2);

        avl.insert_avl(4);
        avl.insert_avl(5); // 触发左旋
        assert_eq!(avl.root.as_ref().unwrap().right.as_ref().unwrap().value, 4);
    }
}
