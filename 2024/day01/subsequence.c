// https://leetcode.com/problems/longest-ideal-subsequence/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int min(int a, int b) { return a < b ? a : b; }

int max(int a, int b) { return a > b ? a : b; }
int longestIdealString(char *s, int k) {
  int length = strlen(s);
  int dp[26];
  memset(dp, 0, sizeof dp);

  dp[s[0] - 'a'] = 1;
  int res = 1;

  for (int i = 1; i < length; i++) {
    int c = s[i] - 'a';

    for (int j = 0; j < 26; j++) {

      if (abs(c - j) <= k) {
        dp[c] = max(dp[c], dp[j]);
      }
    }

    dp[c]++;
    if (dp[c] > res)
      res = dp[c];
  }

  return res;
}

int main(int argc, char *argv[]) {
  printf("%d\n", longestIdealString("acfgbd", 2));
  printf("%d\n", longestIdealString("abcd", 2));

  return EXIT_SUCCESS;
}
