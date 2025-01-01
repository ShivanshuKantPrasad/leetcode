class Solution:
    def generate_partitions(self, candidates, n, current_partition, index, answer) -> List[List[int]]:
        # Base cases
        if n == 0:
            answer.append(current_partition)

        # print(n)
        if n < 0:
            return

        result = []
        for i in range(index, len(candidates)):
            if i > index and candidates[i] == candidates[i - 1]:
                continue
                
            self.generate_partitions(
                candidates,
                n - candidates[i],
                current_partition + [candidates[i]],
                i + 1,
                answer
            )

    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        answer = []
        candidates.sort()
        self.generate_partitions(candidates, target, [], 0, answer)
        return answer
