// https://leetcode.com/problems/n-ary-tree-postorder-traversal/?envType=daily-question&envId=2024-08-26

#include <stdio.h>
#include <stdlib.h>

struct Node {
  int val;
  int numChildren;
  struct Node **children;
};

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

int size(struct Node *root) {
  if (root == NULL)
    return 0;
  int res = 1;
  for (int i = 0; i < root->numChildren; i++) {
    res += size(root->children[i]);
  }
  return res;
}
int post(struct Node *root, int index, int *result) {
  if (root == NULL)
    return index;
  for (int i = 0; i < root->numChildren; i++) {
    index = post(root->children[i], index, result);
  }
  result[index] = root->val;
  return index + 1;
}

int *postorder(struct Node *root, int *returnSize) {
  *returnSize = size(root);
  int *result = (int *)malloc(sizeof(int) * (*returnSize));
  post(root, 0, result);
  return result;
}
