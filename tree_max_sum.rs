use preloaded::TreeNode;

fn max_sum(tree: Option<&TreeNode>) -> i32 {
    if tree.is_none() {
        return 0;
    }
    if tree.unwrap().left.is_none() {
        return tree.unwrap().value+max_sum(tree.unwrap().right.as_deref());
    }    
    if tree.unwrap().right.is_none() {
        return tree.unwrap().value+max_sum(tree.unwrap().left.as_deref());
    }
    tree.unwrap().value+max_sum(tree.unwrap().left.as_deref()).max(max_sum(tree.unwrap().right.as_deref()))
}