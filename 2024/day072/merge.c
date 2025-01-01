#include <stddef.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *mergeNodes(struct ListNode *head) {
  if (head == NULL) {
    return NULL;
  }
  head = head->next;
  struct ListNode *cur = head;
  while (cur != NULL) {
    int sum = 0;
    struct ListNode *prev = cur;
    while (cur->val != 0) {
      sum += cur->val;
      cur = cur->next;
    }
    prev->val = sum;
    prev->next = cur->next;
    cur = cur->next;
  }
  return head;
}
