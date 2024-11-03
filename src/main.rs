use graph_traversal::TreeNode;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_and_child(){
        let mut node = TreeNode::new(4);
        let child = TreeNode::new(3);
        node.add_left(child);

        assert!(node.left.unwrap().val == 3)
    }

    #[test]
    fn node_and_children(){
        let mut node = TreeNode::new(0);

        node.add_left(TreeNode::new(1));
    }
}