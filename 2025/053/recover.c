// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/description/
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

typedef struct TreeNode TreeNode;

char peek(char *str) { return *str; }

char next(char **str) {
  if (peek(*str) == '\0') {
    return '\0';
  }

  char next = **str;
  (*str)++;
  return next;
}

int number(char **str) {
  int num = 0;
  while (isdigit(peek(*str))) {
    num = num * 10 + (next(str) - '0');
  }
  return num;
}

int depth(char **str) {
  int depth = 0;
  while (peek(*str + depth) == '-') {
    depth++;
  }
  return depth;
}

TreeNode *new(int number) {
  TreeNode *res = malloc(sizeof(TreeNode));
  res->val = number;
  res->left = NULL;
  res->right = NULL;
  return res;
}

bool insert(TreeNode *tree, int depth, int number) {
  if (number == 6)
    printf("trying %d %d\n", depth, number);
  if (depth == 0) {
    if (tree->val != 0)
      return false;
    tree->val = number;
    return true;
  } else {
    if (tree->left == NULL) {
      tree->left = new (0);
    }
    if (insert(tree->left, depth - 1, number))
      return true;

    if (tree->right == NULL) {
      tree->right = new (0);
    }
    return insert(tree->right, depth - 1, number);
  }
}

TreeNode *helper(char **traversal, int d) {
  int cur_depth = depth(traversal);
  if (cur_depth != d)
    return NULL;
  *traversal += cur_depth;

  int val = number(traversal);

  TreeNode *node = new (val);
  node->left = helper(traversal, d + 1);
  node->right = helper(traversal, d + 1);
  return node;
}

struct TreeNode *recoverFromPreorder(char *traversal) {
  return helper(&traversal, 0);
}
