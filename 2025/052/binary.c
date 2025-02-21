#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

typedef struct {
  struct TreeNode *tree;
} FindElements;

typedef struct TreeNode TreeNode;

void findElementsInitialize(TreeNode *root) {
  if (root->left) {
    root->left->val = 2 * root->val + 1;
    findElementsInitialize(root->left);
  }
  if (root->right) {
    root->right->val = 2 * root->val + 2;
    findElementsInitialize(root->right);
  }
}

FindElements *findElementsCreate(struct TreeNode *root) {
  FindElements *find = malloc(sizeof(FindElements));
  root->val = 0;
  findElementsInitialize(root);
  find->tree = root;
  return find;
}

bool findTreeElements(TreeNode *root, int target) {
  return (root != NULL) &&
         (root->val == target || findTreeElements(root->left, target) ||
          findTreeElements(root->right, target));
}

bool findElementsFind(FindElements *obj, int target) {
  return obj != NULL && findTreeElements(obj->tree, target);
}

void findTreeFree(TreeNode *root) {
  if (root != NULL) {
    findTreeFree(root->right);
    findTreeFree(root->left);
  }
  free(root);
}

void findElementsFree(FindElements *obj) {
  findTreeFree(obj->tree);
  free(obj);
}

/**
 * Your FindElements struct will be instantiated and called as such:
 * FindElements* obj = findElementsCreate(root);
 * bool param_1 = findElementsFind(obj, target);

 * findElementsFree(obj);
*/
