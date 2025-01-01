# https://leetcode.com/problems/number-complement/

class Solution(object):
    def findComplement(self, num):
        """
        :type num: int
        :rtype: int
        """
        binary = bin(num)[2:]
        binary = list(binary)
        for i in range(len(binary)):
            binary[i] = "0" if binary[i] == "1" else "1"
        return int("".join(binary), 2)
