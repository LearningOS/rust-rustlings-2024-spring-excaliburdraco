/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;

// use super::algorithm10::NodeNotInGraph;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}
/*TreeNode<T> 表示二叉搜索树中的一个节点，T是节点的值类型，需要实现Ord接口，可以进行比较操作。
left和right是节点的左右子节点，分别指向两个TreeNode。value是节点的值。
 */

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}
/*定义一个二叉搜索树 BinarySearchTree<T>，T是节点的值类型，需要实现Ord接口，可以进行比较操作。
root是BinarySearchTree的根节点，指向一个TreeNode。
root与left,right不同，left、right也是Option<Box<TreeNode<T>>>,因为根节点可能为空。
root为Some(Box<TreeNode<T>>)表示根节点不为空，为None表示根节点为空。

 */

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
        //TODO
        match self.root{
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // true
        match self.root {
            Some(ref node) => node.search(value),
            None => false,
        }
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
            Ordering::Less => match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            },
            Ordering::Equal => {} // 重复值不插入
        }
    }

    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                Some(ref node) => node.search(value),
                None => false,
            },
            Ordering::Greater => match self.right {
                Some(ref node) => node.search(value),
                None => false,
            },
            Ordering::Equal => true,
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
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
