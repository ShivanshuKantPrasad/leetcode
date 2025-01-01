// https://leetcode.com/problems/palindrome-partitioning/description/

#include <bitset>
#include <cstdlib>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
  int n;
  bitset<16> dp[16] = {0};
  // Compute dp[l][r] for every substring s[l..r]
  inline void compute_DP(string &s) {
    for (int l = 0; l < n - 1; l++) {
      dp[l][l] = 1;
      dp[l][l + 1] = s[l] == s[l + 1];
    }
    dp[n - 1][n - 1] = 1;
    for (int l = n - 1; l >= 0; l--) {
      for (int len = 3; len <= n - l; len++) {
        int r = l + len - 1;
        dp[l][r] = dp[l + 1][r - 1] && s[l] == s[r];
      }
    }
  }

  // backtracking to find the valid partitions for s[start:]
  vector<vector<string>> ans;
  inline void dfs(string &s, int start, vector<string> &valids) {
    if (start >= n)
      ans.push_back(valids);
    for (int end = start; end < n; end++) {
      if (dp[start][end]) {
        valids.push_back(s.substr(start, end - start + 1));
        dfs(s, end + 1, valids);
        valids.pop_back(); // backtracking
      }
    }
  }

  vector<vector<string>> partition(string &s) {
    n = s.size();
    if (n == 1)
      return {{s}};
    fill(dp, dp + 16, 0);
    compute_DP(s);
    vector<string> valids;
    dfs(s, 0, valids);
    return ans;
  }
};

int main() {
  auto sol = new Solution();
  string string = "aab";
  auto result = sol->partition(string);
  for (auto partition : result) {
    cout << "[ ";
    for (auto string : partition) {
      cout << string << ", ";
    }
    cout << "]\n";
  }
  return EXIT_SUCCESS;
}
