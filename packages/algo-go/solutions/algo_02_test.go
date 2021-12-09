package solutions

import "testing"

func TestIsValid(t *testing.T) {
	root := TreeNode{
		val:   5,
		left:  NewTreeNode(4),
		right: NewTreeNode(6),
	}
	if !IsValidBST(&root, nil, nil) {
		t.Error("Failed")
	}
}

func TestIsInvalid(t *testing.T) {
	root := TreeNode{
		val:   5,
		left:  NewTreeNode(7),
		right: NewTreeNode(6),
	}
	if IsValidBST(&root, nil, nil) {
		t.Error("Failed")
	}
}
