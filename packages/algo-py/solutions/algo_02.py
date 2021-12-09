class TreeNode:
    def __init__(self, val):
        self.left = None
        self.right = None
        self.val = val


def is_valid_BST(node, min, max):
    if node == None:
        return True
    if (min is not None and node.val <= min) or (max is not None and max <= node.val):
        return False
    return is_valid_BST(node.left, min, node.val) and is_valid_BST(node.right, node.val, max)
