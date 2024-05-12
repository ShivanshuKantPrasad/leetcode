// https://leetcode.com/problems/number-of-wonderful-substrings/description/

#include <stdio.h>

long long wonderfulSubstrings(char *word) {
  int cnt[1025] = {0};
  cnt[0] = 1;

  char *letter = word;
  int i = 0;
  int mask = 0;
  long long res = 0;
  while (*letter != '\0') {
    mask ^= (1 << (*letter - 'a'));
    res += cnt[mask];
    for (int i = 0; i < 10; i++) {
      res += cnt[mask ^ (1 << i)];
    }
    cnt[mask]++;
    letter++;
    i++;
  }

  return res;
}

int main() {
  printf("%lld\n", wonderfulSubstrings("aba"));
  printf("%lld\n", wonderfulSubstrings("aabb"));
  printf("%lld\n", wonderfulSubstrings("he"));
}
