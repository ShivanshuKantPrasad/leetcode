// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/description/

#include <stdio.h>
int minOperations(int *nums, int numsSize, int k) {
  int xor = 0;
  for (int i = 0; i < numsSize; i++) {
    xor = xor^nums[i];
  }

  xor = xor^k;
  return __builtin_popcount(xor);
}

int main() {
  int nums[] = {2, 1, 3, 4};
  printf("%d", minOperations(nums, 4, 1));
  return 0;
}
