//! # RikuBox & BST
//! 
//! 学習用に作成した、自作スマートポインタと二分探索木ライブラリです。
//! 
//! ## 特徴
//! - `MyBox`: `NonNull` と `Layout` を使用した自作 Box
//! - `BinarySearchTree`: スタックオーバーフローしない安全な実装
//! 
//! ## Example
//! ```rust
//! use rikubox::bst::BinarySearchTree;
//! let mut tree = BinarySearchTree::new();
//! tree.insert(10);
//! ```

pub mod r#box;
pub mod bst;
