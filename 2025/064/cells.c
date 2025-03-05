// https://leetcode.com/problems/count-total-number-of-colored-cells/

long long coloredCells(int n) {
  long long m = (long long)n;
  return 2 * m * m - (2 * m - 1);
}
