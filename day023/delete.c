// https://leetcode.com/problems/delete-leaves-with-a-given-value/

#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

struct TreeNode *removeLeafNodes(struct TreeNode *root, int target) {
  if (root) {
    root->left = removeLeafNodes(root->left, target);
    root->right = removeLeafNodes(root->right, target);
    if (!root->left && !root->right && root->val == target) {
      // free(root);
      root = NULL;
    }
  }
  return root;
}

void print_tree(struct TreeNode *root) {

  printf("Tree { val: %d, left: ", root->val);

  if (root->left)
    print_tree(root->left);
  else
    printf("None");

  printf(", right: ");

  if (root->left)
    print_tree(root->left);
  else
    printf("None");

  printf("}");
}

struct TreeNode *new_tree(int val, struct TreeNode *left,
                          struct TreeNode *right) {
  struct TreeNode *node = malloc(sizeof(struct TreeNode));
  *node = (struct TreeNode){val, left, right};
  return node;
}

int main(int argc, char *argv[]) {
  struct TreeNode *tree =
      new_tree(1, new_tree(2, new_tree(2, NULL, NULL), NULL),
               new_tree(3, new_tree(2, NULL, NULL), new_tree(4, NULL, NULL)));
  print_tree(tree);
  printf("\n");
  print_tree(removeLeafNodes(tree, 2));

  return 0;
}
