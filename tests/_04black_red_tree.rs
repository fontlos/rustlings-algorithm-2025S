// 完成基本的二叉搜索树接口

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
    color: Color, // 新增颜色标记
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
            color: Color::Red, // 新节点默认为红色
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

    /// 公开的插入接口
    pub fn insert_rb(&mut self, value: T) {
        match self.root.take() {
            Some(root) => {
                let mut new_root = root.insert_rb(value);
                new_root.color = Color::Black; // 根节点始终为黑
                self.root = Some(new_root);
            }
            None => {
                let mut node = TreeNode::new(value);
                node.color = Color::Black; // 根节点强制为黑
                self.root = Some(Box::new(node));
            }
        }
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

    /// 判断节点是否为红色 (空节点视为黑色)
    fn is_red(node: &Option<Box<Self>>) -> bool {
        node.as_ref().map_or(false, |n| n.color == Color::Red)
    }

    /// 红黑树插入入口
    pub fn insert_rb(mut self: Box<Self>, value: T) -> Box<Self> {
        match value.cmp(&self.value) {
            Ordering::Less => {
                self.left = match self.left {
                    Some(left) => Some(left.insert_rb(value)),
                    None => Some(Box::new(Self::new(value))),
                };
            }
            Ordering::Greater => {
                self.right = match self.right {
                    Some(right) => Some(right.insert_rb(value)),
                    None => Some(Box::new(Self::new(value))),
                };
            }
            Ordering::Equal => return self, // 重复值不插入
        }
        self.fixup() // 插入后修复红黑树性质
    }

    /// 修复红黑树性质的三种情况
    fn fixup(mut self: Box<Self>) -> Box<Self> {
        // 右子红且左子黑
        if Self::is_red(&self.right) && !Self::is_red(&self.left) {
            self = self.rotate_left();
        }
        // 左子红且左子的左子红
        if Self::is_red(&self.left) && Self::is_red(&self.left.as_ref().unwrap().left) {
            self = self.rotate_right();
        }
        // 左右子均红
        if Self::is_red(&self.left) && Self::is_red(&self.right) {
            self.flip_colors();
        }
        self
    }

    /// 左旋操作
    fn rotate_left(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.right.take().unwrap();
        self.right = new_root.left.take();
        new_root.color = self.color;
        self.color = Color::Red;
        new_root.left = Some(self);
        new_root
    }

    /// 右旋操作
    fn rotate_right(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.left.take().unwrap();
        self.left = new_root.right.take();
        new_root.color = self.color;
        self.color = Color::Red;
        new_root.right = Some(self);
        new_root
    }

    /// 颜色翻转
    fn flip_colors(&mut self) {
        self.color = match self.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
        self.left.as_mut().unwrap().color = Color::Black;
        self.right.as_mut().unwrap().color = Color::Black;
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
    fn test_rb_insert() {
        let mut tree = BinarySearchTree::new();
        tree.insert_rb(3);
        tree.insert_rb(1);
        tree.insert_rb(5);
        tree.insert_rb(2);

        // 验证根节点为黑
        assert_eq!(tree.root.as_ref().unwrap().color, Color::Black);

        // 验证无连续红节点
        fn check_rb_properties<T: Ord>(node: &Option<Box<TreeNode<T>>>) -> bool {
            node.as_ref().map_or(true, |n| {
                let no_double_red = n.color != Color::Red ||
                    (TreeNode::is_red(&n.left) == false &&
                    TreeNode::is_red(&n.right) == false);
                no_double_red && check_rb_properties(&n.left) && check_rb_properties(&n.right)
            })
        }
        assert!(check_rb_properties(&tree.root));
    }
}
