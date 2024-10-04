/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord + Debug + Clone, // 添加 Clone 和 Debug 约束
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + Debug + Clone, // 添加 Clone 和 Debug 约束
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Debug + Clone,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a node into the subtree rooted at this node
    // Returns true if insertion is successful, false if value already exists
    fn insert(&mut self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left_child) = self.left {
                    left_child.insert(value)
                } else {
                    self.left = Some(Box::new(TreeNode::new(value.clone())));
                    true
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right_child) = self.right {
                    right_child.insert(value)
                } else {
                    self.right = Some(Box::new(TreeNode::new(value.clone())));
                    true
                }
            }
            Ordering::Equal => {
                // Duplicate value; do not insert
                false
            }
        }
    }

    // Search for a value in the subtree rooted at this node
    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref left_child) = self.left {
                    left_child.search(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref right_child) = self.right {
                    right_child.search(value)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Debug + Clone,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut root_node) => {
                let inserted = root_node.insert(&value);
                if !inserted {
                    println!(
                        "Value {:?} already exists in the BST. Skipping insertion.",
                        value.clone()
                    );
                }
            }
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match self.root {
            Some(ref root_node) => root_node.search(&value),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 搜索一个尚未插入的值，应返回 false
        assert_eq!(bst.search(1), false);

        // 插入多个值
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 搜索已插入的值，应返回 true
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 搜索未插入的值，应返回 false
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复的值
        bst.insert(1);
        bst.insert(1);

        // 搜索值应返回 true
        assert_eq!(bst.search(1), true);

        // 检查树结构，确保没有重复的节点
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }

    #[test]
    fn test_empty_tree() {
        let bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.search(10), false);
    }

    #[test]
    fn test_single_element_tree() {
        let mut bst = BinarySearchTree::new();
        bst.insert(42);
        assert_eq!(bst.search(42), true);
        assert_eq!(bst.search(43), false);
    }

    #[test]
    fn test_complex_tree() {
        let mut bst = BinarySearchTree::new();
        let values = vec![15, 10, 20, 8, 12, 17, 25, 6, 11, 16, 27];
        for &val in &values {
            bst.insert(val);
        }

        // 搜索所有插入的值，应返回 true
        for &val in &values {
            assert_eq!(bst.search(val), true);
        }

        // 搜索未插入的值，应返回 false
        let non_existing = vec![5, 9, 13, 19, 22, 30];
        for &val in &non_existing {
            assert_eq!(bst.search(val), false);
        }
    }
}
