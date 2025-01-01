// https://leetcode.com/problems/distribute-coins-in-binary-tree/

#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

int distributeCoins(struct TreeNode *root) {
  if (root == NULL)
    return 0;
  int moves = distributeCoins(root->left);
  if (root->left) {
    if (root->left->val > 1) {
      int coins = root->left->val - 1;
      root->left->val = 1;
      root->val += coins;
      moves += abs(coins);
      // printf("%d %d %d\n", root->val, root->left->val, coins);
      // printf("%d\n", moves);
    } else if (root->left->val < 1) {
      int coins = 1 - root->left->val;
      root->val -= coins;
      root->left->val = 1;
      moves += abs(coins);
    }
  }
  moves += distributeCoins(root->right);
  if (root->right) {
    if (root->right->val > 1) {
      int coins = root->right->val - 1;
      root->right->val = 1;
      root->val += coins;
      moves += abs(coins);
    } else if (root->right->val < 1) {
      int coins = 1 - root->right->val;
      root->val -= coins;
      root->right->val = 1;
      moves += abs(coins);
    }
  }

  return moves;
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
      new_tree(1, new_tree(0, NULL, NULL), new_tree(0, NULL, NULL));
  print_tree(tree);
  printf("\n");
  printf("%d\n", distributeCoins(tree));

  return 0;
}
