/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


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
        match self.root {
            Some(ref mut node) => {
                // If the root exists, delegate the insertion to the TreeNode's insert method
                node.insert(value);
            }
            None => {
                // If the root is None, create a new TreeNode and set it as the root
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match &self.root {
            Some(node) => node.search(value),
            None => false, // If the tree is empty, return false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // If the value is less than the current node's value, insert into the left subtree
                match self.left {
                    Some(ref mut left_node) => left_node.insert(value),
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            Ordering::Greater => {
                // If the value is greater than the current node's value, insert into the right subtree
                match self.right {
                    Some(ref mut right_node) => right_node.insert(value),
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            Ordering::Equal => {
                // If the value is equal to the current node's value, do nothing (no duplicates)
            }
        }
    }

    // Search for a value in the tree
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // If the value is less than the current node's value, search the left subtree
                match &self.left {
                    Some(left_node) => left_node.search(value),
                    None => false,
                }
            }
            Ordering::Greater => {
                // If the value is greater than the current node's value, search the right subtree
                match &self.right {
                    Some(right_node) => right_node.search(value),
                    None => false,
                }
            }
            Ordering::Equal => {
                // If the value is equal to the current node's value, return true
                true
            }
        }
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


