// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

#include <stdio.h>
#include <stdlib.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *removeNthFromEnd(struct ListNode *head, int n) {

  struct ListNode *res = &(struct ListNode){0, head};
  struct ListNode *cur = res;

  for (int i = 0; i < n; i++) {
    head = head->next;
  }

  while (head) {
    head = head->next;
    cur = cur->next;
  }

  struct ListNode *temp = cur->next;
  cur->next = cur->next->next;
  free(temp);

  return res->next;
}

struct ListNode *makeList(int val, struct ListNode *next) {
  struct ListNode *res = malloc(sizeof(struct ListNode));
  res->val = val;
  res->next = next;
  return res;
}

void printList(struct ListNode *head, char *name) {
  printf("List %s:\n", name);
  int index = 0;
  while (head) {
    printf("List[%d] = %d\n", index, head->val);
    head = head->next;
    index++;
  }
}

int main(void) {
  struct ListNode *list =
      makeList(1, makeList(2, makeList(3, makeList(4, makeList(5, NULL)))));
  printList(list, "original");

  struct ListNode *result = removeNthFromEnd(list, 2);
  printList(result, "result");
  printf("\n");

  list = makeList(1, NULL);
  printList(list, "original");

  result = removeNthFromEnd(list, 1);
  printList(result, "result");
  printf("\n");

  list = makeList(1, makeList(2, NULL));
  printList(list, "original");

  result = removeNthFromEnd(list, 1);
  printList(result, "result");
  printf("\n");

  return 0;
}
