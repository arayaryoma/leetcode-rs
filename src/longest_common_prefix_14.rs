/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter().fold(
            strs.get(0).unwrap_or(&String::new()).clone(),
            |prefix, s| {
                prefix
                    .chars()
                    .zip(s.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect()
            },
        )
    }
}
// @lc code=end
