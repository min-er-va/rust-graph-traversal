use std::collections::VecDeque;

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
        assert!(self.left.is_none());
        self.left = Some(Box::new(child));
    }
    pub fn add_right(&mut self, child: TreeNode<T>) {
        assert!(self.right.is_none());
        self.right = Some(Box::new(child));
    }

    pub fn find_all(&self, f: impl Fn(&TreeNode<T>) -> bool) -> Vec<&TreeNode<T>> {
        let mut stack: Vec<&TreeNode<T>> = Vec::new();
        let mut vec_out = Vec::new();

        stack.push(self);
        while let Some(current) = stack.pop() {
            // filter
            if f(current){
                vec_out.push(current);
            }
            // Add children to the queue
            if let Some(right) = &current.right {
                stack.push(right.as_ref());
            }
            if let Some(left) = &current.left {
                stack.push(left.as_ref());
            }
        };
        vec_out
    }

    pub fn find_all_bfs(&self, f: impl Fn(&TreeNode<T>) -> bool) -> Vec<&TreeNode<T>> {
        let mut q: VecDeque<&TreeNode<T>> = VecDeque::new();
        let mut vec_out = Vec::new();

        // Add self to queue
        q.push_back(self);

        while let Some(current) = q.pop_front() {
            // filter
            if f(current){
                vec_out.push(current);
            }
            // Add children to the queue
            if let Some(left) = &current.left {
                q.push_back(left);
            }
            if let Some(right) = &current.right {
                q.push_back(right);
            }
        };
        vec_out
    }
}

impl<T> Iterator for TreeNode<T> {
    type Item = TreeNode<T>;

    // dfs
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}