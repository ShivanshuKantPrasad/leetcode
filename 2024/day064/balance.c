// https://leetcode.com/problems/balance-a-binary-search-tree/editorial/
#include <math.h>
#include <stddef.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

void rightRotate(struct TreeNode *parent, struct TreeNode *node) {
  struct TreeNode *tmp = node->left;
  node->left = tmp->right;
  tmp->right = node;
  parent->right = tmp;
}

void leftRotate(struct TreeNode *parent, struct TreeNode *node) {
  struct TreeNode *tmp = node->right;
  node->right = tmp->left;
  tmp->left = node;
  parent->right = tmp;
}

void makeRotations(struct TreeNode *vineHead, int count) {
  struct TreeNode *current = vineHead;
  for (int i = 0; i < count; ++i) {
    struct TreeNode *tmp = current->right;
    leftRotate(current, tmp);
    current = current->right;
  }
}

int depth(struct TreeNode *root) {
  if (root == NULL)
    return 0;
  int left = depth(root->left);
  int right = depth(root->right);
  return (left > right ? left : right) + 1;
}
struct TreeNode *balanceBST(struct TreeNode *root) {

  if (!root)
    return NULL;

  struct TreeNode *vineHead = &(struct TreeNode){0, NULL, NULL};
  vineHead->right = root;
  struct TreeNode *current = vineHead;
  while (current->right) {
    if (current->right->left) {
      rightRotate(current, current->right);
    } else {
      current = current->right;
    }
  }

  int nodeCount = 0;
  current = vineHead->right;
  while (current) {
    ++nodeCount;
    current = current->right;
  }

  int m = pow(2, floor(log2(nodeCount + 1))) - 1;
  makeRotations(vineHead, nodeCount - m);
  while (m > 1) {
    m /= 2;
    makeRotations(vineHead, m);
  }

  struct TreeNode *balancedRoot = vineHead->right;

  return balancedRoot;
}
