// https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/
#include <stdlib.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

int gcd(int a, int b) {
  if (b == 0)
    return a;
  return gcd(b, a % b);
}

struct ListNode *insertGreatestCommonDivisors(struct ListNode *head) {
  if (!head)
    return head;
  struct ListNode *current = head;
  struct ListNode *next = head->next;

  while (next) {
    int a = gcd(current->val, next->val);
    struct ListNode *newNode = malloc(sizeof(struct ListNode));
    newNode->val = a;
    newNode->next = next;
    current->next = newNode;

    current = next;
    next = next->next;
  }

  return head;
}
