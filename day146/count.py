# https://leetcode.com/problems/uncommon-words-from-two-sentences/

class Solution(object):
    def uncommonFromSentences(self, A, B):
        count = collections.Counter(A.split())
        count += collections.Counter(B.split())

        return [word for word in count if count[word] == 1]
