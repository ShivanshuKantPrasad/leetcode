# https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/

class Solution:
    def countMaxOrSubsets(self, nums: List[int]) -> int:
        max = reduce(operator.or_, nums, 0)
        res = 0
        for subset in chain.from_iterable(combinations(nums, r) for r in range(len(nums)+1)):
            res += 1 if reduce(operator.or_, subset, 0) == max else 0 
        return res
        
