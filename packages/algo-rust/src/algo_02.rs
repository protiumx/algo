struct TreeNode {
  val: i32,
  left: Option<Box<TreeNode>>,
  right: Option<Box<TreeNode>>,
}

impl TreeNode {
  fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

fn is_valid_bst(node: Option<Box<TreeNode>>, min: Option<i32>, max: Option<i32>) -> bool {
  match node {
    None => true,
    Some(n) => {
      if min != None && n.val <= min.unwrap() || max != None && n.val >= max.unwrap() {
        return false;
      }
      is_valid_bst(n.left, min, Some(n.val)) && is_valid_bst(n.right, Some(n.val), max)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_valid() {
    let mut root = Box::new(TreeNode::new(5));
    root.left = Some(Box::new(TreeNode::new(4)));
    root.right = Some(Box::new(TreeNode::new(6)));

    assert_eq!(is_valid_bst(Some(root), None, None), true);
  }

  fn test_invalid() {
    let mut root = Box::new(TreeNode::new(5));
    root.left = Some(Box::new(TreeNode::new(6)));
    root.right = Some(Box::new(TreeNode::new(6)));

    assert_eq!(is_valid_bst(Some(root), None, None), false);
  }
}
