#include <stdio.h>
#include <stdlib.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

int *nodesBetweenCriticalPoints(struct ListNode *head, int *returnSize) {
  int first = -100000;
  int last = -100000;
  int index = 1;
  int min = 100000;

  struct ListNode *prev = head;
  struct ListNode *cur = head->next;
  while (cur->next != NULL) {
    if ((prev->val < cur->val && cur->val > cur->next->val) ||
        (prev->val > cur->val && cur->val < cur->next->val)) {
      if (first == -100000)
        first = index;
      int dist = index - last;
      if (dist < min)
        min = dist;
      last = index;
    }
    prev = cur;
    cur = cur->next;
    index += 1;
  }

  int *result = malloc(2 * sizeof(int));
  if (first != last) {
    result[0] = min;
    result[1] = last - first;
  } else {
    result[0] = -1;
    result[1] = -1;
  }
  *returnSize = 2;
  return result;
}
