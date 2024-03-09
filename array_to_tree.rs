use preloaded::TreeNode;

fn f(a: &[i32], i: usize) -> Option<TreeNode> {
    if i>=a.len() {None} else {Some(TreeNode::new(a[i],f(a,2*i+1),f(a,2*i+2)))}
}

fn array_to_tree(a: &[i32]) -> Option<TreeNode> {
    f(a,0)
}