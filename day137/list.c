
#include <malloc.h>
#include <stddef.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int length(struct ListNode *head) {
  if (head == NULL)
    return 0;
  return 1 + length(head->next);
}
struct ListNode **splitListToParts(struct ListNode *head, int k,
                                   int *returnSize) {
  int n = length(head);
  *returnSize = k;
  int avg = n / k;
  int extra = n % k;

  struct ListNode **result =
      (struct ListNode **)calloc(k, sizeof(struct ListNode *));
  struct ListNode **current = result;
  for (int i = 0; i < k; i++) {
    *current = head;
    current++;

    struct ListNode *prev = head;
    for (int i = 0; i < avg; i++) {
      if (head == NULL)
        break;
      prev = head;
      head = head->next;
    }
    if (head == NULL)
      continue;

    if (i < extra) {
      prev = head;
      head = head->next;
    }
    prev->next = NULL;
  }
  return result;
}
