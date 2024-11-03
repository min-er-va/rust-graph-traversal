use std::fmt;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>
}

impl<T> TreeNode<T> {

    pub fn new(val: T) -> TreeNode<T> {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn add_left(&mut self, child: TreeNode<T>) {
        match (self.left.as_ref(), child){
            (None, child) => {
                self.left = Some(Box::new(child));
            }
            (Some(_), _) => {
                panic!()
            }
        }
    }
    pub fn add_right(&mut self, child: TreeNode<T>) {
        match (self.right.as_ref(), child){
            (None, child) => {
                self.right = Some(Box::new(child));
            }
            (Some(_), _) => {
                panic!()
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

impl<T> Iterator for TreeNode<T> {
    type Item = TreeNode<T>;

    // dfs
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}