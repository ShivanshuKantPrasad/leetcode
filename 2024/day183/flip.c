// https://leetcode.com/problems/flip-equivalent-binary-trees/description/

#include <stdbool.h>
#include <stddef.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

bool flipEquiv(struct TreeNode *root1, struct TreeNode *root2) {
  if (root1 == NULL && root2 == NULL)
    return true;
  if (root1 == NULL || root2 == NULL)
    return false;

  bool flipped = flipEquiv(root1->left, root2->right) &&
                 flipEquiv(root1->right, root2->left);
  bool nonflipped = flipEquiv(root1->left, root2->left) &&
                    flipEquiv(root1->right, root2->right);
  return (flipped || nonflipped) && root1->val == root2->val;
}
