export class TreeNode {
  constructor(
    public val: number,
    public left: TreeNode | null = null,
    public right: TreeNode | null = null
  ) {}
}

export function isValidBST(
  node: TreeNode | null,
  min: number | null,
  max: number | null
): boolean {
  if (node == null) {
    return true;
  }
  if ((min != null && node.val <= min) || (max != null && node.val >= max)) {
    return false;
  }

  return (
    isValidBST(node.left, min, node.val) &&
    isValidBST(node.right, node.val, max)
  );
}
