#include "stdbool.h"

struct ListNode {
  int val;
  struct ListNode *next;
};

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

bool isSubPathHelper(struct ListNode *head, struct TreeNode *root) {
  if (!head)
    return true;
  if (!root)
    return false;

  return (root->val == head->val) &&
         (isSubPathHelper(head->next, root->right) ||
          isSubPathHelper(head->next, root->left));
}

bool isSubPath(struct ListNode *head, struct TreeNode *root) {
  return root && (isSubPathHelper(head, root) || isSubPath(head, root->left) ||
                  isSubPath(head, root->right));
}
