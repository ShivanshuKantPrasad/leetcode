// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/
#include <stdbool.h>
#include <stdlib.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *modifiedList(int *nums, int numsSize, struct ListNode *head) {
  struct ListNode result = (struct ListNode){.val = 0, .next = head};
  struct ListNode *prev = &result;
  struct ListNode *cur = head;

  bool present[100001] = {false};
  for (int i = 0; i < numsSize; i++) {
    present[nums[i]] = true;
  }
  while (cur) {
    if (!present[cur->val]) {
      prev->next = cur;
      prev = prev->next;
      cur = cur->next;
    } else {
      struct ListNode *temp = cur;
      cur = cur->next;
      free(temp);
    }
  }
  prev->next = cur;
  return result.next;
}
