package solutions

type TreeNode struct {
	val   int
	left  *TreeNode
	right *TreeNode
}

func NewTreeNode(val int) *TreeNode {
	return &TreeNode{val: val, left: nil, right: nil}
}

func IsValidBST(root *TreeNode, min *int, max *int) bool {
	if root == nil {
		return true
	}
	if (min != nil && root.val <= *min) ||
		(max != nil && root.val >= *max) {
		return false
	}

	return IsValidBST(root.left, min, &root.val) && IsValidBST(root.right, &root.val, max)
}
