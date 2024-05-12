/* https://leetcode.com/problems/reverse-prefix-of-word/
 */

#include <stdio.h>
#include <stdlib.h>

char *reversePrefix(char *word, char ch) {

  int n = 0, m = 0;
  while (word[n] != ch) {
    m++;
    n++;
  }
  while (word[n] != '\0') {
    n++;
  }

  char *result = malloc(300 * sizeof(char));
  printf("%d %d \n", m, n);
  for (int i = 0; i < m; i++) {
    result[i] = word[m - i];
  }
  for (int i = m; i < n; i++) {
    result[i] = word[i];
  }

  return result;
}

int main() {
  printf("%s", reversePrefix("abcdefd", 'd'));
  return 0;
}
