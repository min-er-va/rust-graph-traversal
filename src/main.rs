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
    fn test_find_all(){
        let mut node = TreeNode::new(0);

        let mut node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let mut node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);

        node4.add_left(node5);

        node1.add_left(node3);
        node1.add_right(node4);

        node.add_left(node1);
        node.add_right(node2);

        let nodes = node.find_all(&(|node| node.val == 2 || node.val == 5));
        // Should find node5 and then node2.
        assert!(nodes.len() == 2);
        assert!(nodes[0].val == 5);
        assert!(nodes[1].val == 2);
    }

    #[test]
    fn test_find_all_bfs(){
        let mut node = TreeNode::new(0);

        let mut node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let mut node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);

        node4.add_left(node5);

        node1.add_left(node3);
        node1.add_right(node4);

        node.add_left(node1);
        node.add_right(node2);

        println!("Found nodes with val of 1:");
        let nodes =  node.find_all_bfs(&(|node| node.val == 2 || node.val == 5));
        // Should find node2 and then node5.
        assert!(nodes.len() == 2);
        assert!(nodes[0].val == 2);
        assert!(nodes[1].val == 5);
    }
}