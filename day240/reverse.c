#include <stddef.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

void dfs(struct TreeNode *left, struct TreeNode *right, int level) {
  if (left == NULL || right == NULL)
    return;
  if (level % 2 == 0) {
    int temp = left->val;
    left->val = right->val;
    right->val = temp;
  }
  dfs(left->left, right->right, level + 1);
  dfs(left->right, right->left, level + 1);
}

struct TreeNode *reverseOddLevels(struct TreeNode *root) {
  dfs(root->left, root->right, 0);
  return root;
}
