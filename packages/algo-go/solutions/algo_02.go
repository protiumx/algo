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
	if min != nil && root.val <= *min {
		return false
	}
	if max != nil && root.val >= *max {
		return false
	}
	leftIsValid := true
	rightIsValid := true

	if root.left != nil {
		leftIsValid = IsValidBST(root.left, min, &root.val)
	}
	if root.right != nil {
		rightIsValid = IsValidBST(root.right, &root.val, max)
	}
	return leftIsValid && rightIsValid
}
