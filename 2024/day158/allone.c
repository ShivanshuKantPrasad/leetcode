// https://leetcode.com/problems/all-oone-data-structure/

#include <malloc.h>
#include <stdint.h>

#define MAX_CALLS 50000
#define MOD 200003
typedef uint16_t nodeptr; // arena "pointer" into `nodes[]`
char zero = 0;

typedef struct {
  nodeptr prev;
  nodeptr next;
  nodeptr other; // keys point to counts and vice versa
  char *key;
} Node;

typedef struct {
  Node nodes[MAX_CALLS + 1];
  Node *others; // keys point to counts and vice versa
  nodeptr head;
  nodeptr tail;
} List;

void del(List *list, nodeptr nodep) {
  Node *node = list->nodes + nodep;
  if (node->prev)
    list->nodes[node->prev].next = node->next;
  else if (node->other)
    list->others[node->other].other = node->next;
  if (list->head == nodep)
    list->head = node->next;
  if (node->next)
    list->nodes[node->next].prev = node->prev;
  if (list->tail == nodep)
    list->tail = node->prev;
  node->prev = node->next = 0;
}

void ins(List *list, nodeptr nodep, nodeptr prev, nodeptr next) {
  Node *node = list->nodes + nodep;
  node->prev = prev;
  node->next = next;
  if (prev)
    list->nodes[prev].next = nodep;
  else {
    if (node->other)
      list->others[node->other].other = nodep;
    list->head = nodep;
  }
  if (next)
    list->nodes[next].prev = nodep;
  else
    list->tail = nodep;
}

typedef struct {
  char *key;
  nodeptr node;
} HashEntry;

#define HASH(hash, table, s, mod)                                              \
  do {                                                                         \
    hash = 0;                                                                  \
    for (char *c = (s); *c; c++) {                                             \
      hash = 31 * hash + *c;                                                   \
    }                                                                          \
    hash %= (mod);                                                             \
    uint32_t step = 0;                                                         \
    while ((table)[hash].node != 0 && strcmp((table)[hash].key, (s)) != 0) {   \
      hash = (hash + ++step) % (mod); /* quadratic probe */                    \
    }                                                                          \
  } while (0)

typedef struct {
  List countList;
  List keyList;
  HashEntry keyMap[MOD];
  nodeptr nKeys;
} AllOne;

AllOne *allOneCreate() {
  AllOne *allOne = (AllOne *)calloc(1, sizeof(AllOne));
  allOne->nKeys = 1;
  allOne->countList.others = &(allOne->keyList);
  allOne->keyList.others = &(allOne->countList);
  return allOne;
}

void allOneInc(AllOne *obj, char *key) {
  uint32_t hash;
  HASH(hash, obj->keyMap, key, MOD);
  if (obj->keyMap[hash].node == 0) {
    obj->keyMap[hash].key = key;
    obj->keyMap[hash].node = obj->nKeys++;
  }
  nodeptr nodep = obj->keyMap[hash].node;
  Node *node = obj->keyList.nodes + nodep;
  node->key = key;
  Node *countNode = obj->countList.nodes + node->other;
  nodeptr next = node->other ? countNode->next : obj->countList.head;
  if (next != node->other + 1)
    ins(&(obj->countList), node->other + 1, node->other, next);
  next = node->other ? countNode->next : obj->countList.head;
  del(&(obj->keyList), nodep);
  if (countNode->other == 0)
    del(&(obj->countList), node->other);
  node->other++;
  ins(&(obj->keyList), nodep, 0, obj->countList.nodes[next].other);
}

void allOneDec(AllOne *obj, char *key) {
  uint32_t hash;
  HASH(hash, obj->keyMap, key, MOD);
  nodeptr nodep = obj->keyMap[hash].node;
  Node *node = obj->keyList.nodes + nodep;
  Node *countNode = obj->countList.nodes + node->other;
  if (countNode->prev != node->other - 1)
    ins(&(obj->countList), node->other - 1, countNode->prev, node->other);
  nodeptr prev = countNode->prev;
  del(&(obj->keyList), nodep);
  if (countNode->other == 0)
    del(&(obj->countList), node->other);
  node->other--;
  if (node->other)
    ins(&(obj->keyList), nodep, 0, obj->countList.nodes[prev].other);
}

char *allOneGetMaxKey(AllOne *obj) {
  nodeptr tailp = obj->countList.tail;
  nodeptr headp = obj->countList.nodes[tailp].other;
  char *key = obj->keyList.nodes[headp].key;
  return key ? key : &zero;
}

char *allOneGetMinKey(AllOne *obj) {
  nodeptr headp = obj->countList.head;
  nodeptr kheadp = obj->countList.nodes[headp].other;
  char *key = obj->keyList.nodes[kheadp].key;
  return key ? key : &zero;
}

void allOneFree(AllOne *obj) { free(obj); }
