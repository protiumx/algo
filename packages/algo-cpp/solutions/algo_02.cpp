#include <gtest/gtest.h>
#include <stdio.h>

struct TreeNode
{
  int val;
  TreeNode *left;
  TreeNode *right;

  TreeNode(int val) : val(val), left(NULL), right(NULL) {}
};

bool is_valid_BST(TreeNode *node, int *max, int *min)
{
  if (max != NULL && *max <= node->val)
  {
    return false;
  }
  if (min != NULL && node->val <= *min)
  {
    return false;
  }
  bool left_is_valid = true;
  bool right_is_valid = true;
  if (node->left != NULL)
  {
    left_is_valid = is_valid_BST(node->left, &node->val, min);
  }
  if (node->right != NULL)
  {
    right_is_valid = is_valid_BST(node->right, max, &node->val);
  }

  return left_is_valid && right_is_valid;
}

TEST(IsValidBST, TestValid)
{
  TreeNode root(5);
  TreeNode leftChild(4);
  TreeNode rightChild(6);

  root.left = &leftChild;
  root.right = &rightChild;

  EXPECT_TRUE(is_valid_BST(&root, NULL, NULL));
}

TEST(IsValidBST, TestInvalid)
{
  TreeNode root(5);
  TreeNode leftChild(4);
  TreeNode rightChild(6);

  root.right = &leftChild;
  root.left = &rightChild;

  EXPECT_FALSE(is_valid_BST(&root, NULL, NULL));
}
