struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

void bstToGstHelper(struct TreeNode *root, int *nodeSum) {
  if (!root)
    return;
  bstToGstHelper(root->right, nodeSum);
  *nodeSum += root->val;
  root->val = *nodeSum;
  bstToGstHelper(root->left, nodeSum);
}

struct TreeNode *bstToGst(struct TreeNode *root) {
  int nodeSum = 0;
  bstToGstHelper(root, &nodeSum);
  return root;
}
