// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/

lexicographicallySmallestArray = (
    nums,
    limit,
) => {
    const sorted = [...nums].sort((a, b) => a - b);
    
    let prev = ~1e9;
    let group = -1;
    const groups = [];
    const mapping = {};

    for (let i = 0; i < nums.length; ++i) {
        const x = sorted[i];
        if (prev + limit < x) {
            ++group;
            groups[group] = i;
        }
        mapping[x] = group;
        prev = x;
    }
    
    return nums.map(x => sorted[groups[mapping[x]]++]);
}
