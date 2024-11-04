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
        match self.left.as_ref() {
            None => {
                self.left = Some(Box::new(child));
            }
            Some(_) => {
                panic!()
            }
        }
    }
    pub fn add_right(&mut self, child: TreeNode<T>) {
        match self.right.as_ref() {
            None => {
                self.right = Some(Box::new(child));
            }
            Some(_) => {
                panic!()
            }
        }
    }

    pub fn find_all(&self, f: & impl Fn(&TreeNode<T>) -> bool) -> Vec<&TreeNode<T>> {
        let mut vec_out = Vec::new();
        if f(self){
            vec_out.push(self);
        }
        if let Some(left) = &self.left {
            vec_out.append(&mut left.find_all(f));
        }
        if let Some(right) = &self.right {
            vec_out.append(&mut right.find_all(f));
        }
        vec_out
    }

    pub fn find_all_bfs(&self, f: & impl Fn(&TreeNode<T>) -> bool) -> Vec<&TreeNode<T>> {
        let mut queue: Vec<&TreeNode<T>> = Vec::new();
        let mut vec_out = Vec::new();

        // Add self to queue
        queue.push(self);

        while let Some(&current) = queue.first() {
            // filter
            if f(current){
                vec_out.push(current);
            }
            // remove the last in the queue
            queue = queue[1..].to_vec();
            // Add children to the queue
            if let Some(left) = &current.left {
                queue.push(left);
            }
            if let Some(right) = &current.right {
                queue.push(right);
            }
        };

        vec_out
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