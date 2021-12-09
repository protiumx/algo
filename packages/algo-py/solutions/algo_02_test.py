import unittest
from solutions import TreeNode, is_valid_BST


class TestAlgo_01(unittest.TestCase):

    def test_valid(self):
        root = TreeNode(5)
        root.left = TreeNode(4)
        root.right = TreeNode(6)
        self.assertTrue(is_valid_BST(root, None, None))

    def test_invalid(self):
        root = TreeNode(5)
        root.left = TreeNode(6)
        root.right = TreeNode(4)
        self.assertFalse(is_valid_BST(root, None, None))


if __name__ == '__main__':
    unittest.main()
