import { TreeNode, isValidBST } from "$/algo_02";

describe("algo-02", () => {
  it("should return true for a valid BST", () => {
    const root = new TreeNode(5);
    root.left = new TreeNode(4);
    root.right = new TreeNode(6);
    expect(isValidBST(root, null, null)).toBeTruthy();
  });

  it("should return false for an invalid BST", () => {
    const root = new TreeNode(5);
    root.left = new TreeNode(6);
    root.right = new TreeNode(6);
    expect(isValidBST(root, null, null)).toBeFalsy();
  });
});
