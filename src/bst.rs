use crate::r#box::MyBox;

/// 二分探索木。
/// 
/// # Examples
/// ```rust
/// use rikubox::bst::BinarySearchTree;
/// let mut tree = BinarySearchTree::new();
/// tree.insert(10);
/// ```
pub struct BinarySearchTree<T> {
    root: Option<MyBox<Node<T>>>,
}

struct Node<T> {
    value: T,
    left: Option<MyBox<Node<T>>>,
    right: Option<MyBox<Node<T>>>,
}

impl<T> BinarySearchTree<T> {
    /// 新しい空の二分探索木を作成します。
    ///
    /// # Examples
    /// ```
    /// use rikubox::bst::BinarySearchTree;
    /// let tree = BinarySearchTree::<usize>::new();
    /// ```
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// 値を挿入します。
    /// 
    /// # Examples
    /// ```rust
    /// use rikubox::bst::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10);
    /// ```
    pub fn insert(&mut self, value: T) {
        let mut current_node = &mut self.root;

        while let Some(node) = current_node {
            if value == node.value {
                return;
            }

            if value < node.value {
                current_node = &mut node.left;
            } else {
                current_node = &mut node.right;
            }
        }

        *current_node = Some(MyBox::new(Node::new(value)));
    }

    /// 値を検索します。`value`がツリーに含まれる場合は `true` を返します。
    /// 
    /// # Examples
    /// ```rust
    /// use rikubox::bst::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10);
    /// assert!(tree.contains(&10));
    /// assert!(!tree.contains(&11));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == &node.value {
                return true;
            }

            if value < &node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }

        false
    }
}

impl<T> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        let mut stack: Vec<MyBox<Node<T>>> = Vec::new();

        if let Some(root_node) = self.root.take() {
            stack.push(root_node);
        }

        while let Some(mut node) = stack.pop() {
            if let Some(left) = node.left.take() {
                stack.push(left);
            }
            if let Some(right) = node.right.take() {
                stack.push(right);
            }
        }
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree_is_empty() {
        let t = BinarySearchTree::<i32>::new();
        assert!(!t.contains(&0));
        assert!(!t.contains(&-1));
        assert!(!t.contains(&123));
    }

    #[test]
    fn insert_single_value() {
        let mut t = BinarySearchTree::new();
        t.insert(10);
        assert!(t.contains(&10));
        assert!(!t.contains(&9));
        assert!(!t.contains(&11));
    }

    #[test]
    fn insert_multiple_values_and_contains() {
        let mut t = BinarySearchTree::new();
        for &v in &[5, 3, 7, 2, 4, 6, 8] {
            t.insert(v);
        }

        for &v in &[5, 3, 7, 2, 4, 6, 8] {
            assert!(t.contains(&v));
        }
        for &v in &[0, 1, 9, 10, -1] {
            assert!(!t.contains(&v));
        }
    }

    #[test]
    fn duplicate_inserts_are_ignored() {
        let mut t = BinarySearchTree::new();
        t.insert(5);
        t.insert(5);
        t.insert(5);

        assert!(t.contains(&5));
        assert!(!t.contains(&4));
        assert!(!t.contains(&6));
    }

    /// 片側に偏る（最悪ケース）でも正しく動くか
    #[test]
    fn works_with_descending_inserts_degenerate_tree() {
        let mut t = BinarySearchTree::new();
        for v in (0..100).rev() {
            t.insert(v);
        }
        for v in 0..100 {
            assert!(t.contains(&v));
        }
        assert!(!t.contains(&100));
    }

    #[test]
    fn works_with_strings() {
        let mut t = BinarySearchTree::new();
        t.insert("cat".to_string());
        t.insert("dog".to_string());
        t.insert("ant".to_string());

        assert!(t.contains(&"cat".to_string()));
        assert!(t.contains(&"dog".to_string()));
        assert!(t.contains(&"ant".to_string()));
        assert!(!t.contains(&"fox".to_string()));
    }

    #[cfg(not(miri))]
    #[test]
    fn many_inserts() {
        let mut t = BinarySearchTree::new();
        for v in 0..10_000 {
            t.insert(v);
        }
        // 大量に drop が走る
        drop(t);
    }
}
