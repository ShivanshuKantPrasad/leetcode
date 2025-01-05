#include <string.h>

char *shiftingLetters(char *s, int **shifts, int shiftsSize,
                      int *shiftsColSize) {
  int len = strlen(s);

  int numShifts[len + 1];
  for (int i = 0; i < len; i++)
    numShifts[i] = 0;

  for (int i = 0; i < shiftsSize; i++) {
    numShifts[shifts[i][0]] += shifts[i][2] == 0 ? -1 : 1;
    numShifts[shifts[i][1] + 1] -= shifts[i][2] == 0 ? -1 : 1;
  }

  int curShift = 0;
  for (int i = 0; i < len; i++) {
    curShift += numShifts[i];
    s[i] = 'a' + ((s[i] - 'a' + curShift) % 26 + 26) % 26;
  }

  return s;
}
