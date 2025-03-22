/*
 * @lc app=leetcode id=1672 lang=rust
 *
 * [1672] Richest Customer Wealth
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|customer| customer.iter().sum::<i32>())
            .max()
            .unwrap_or(0)
    }
}
// @lc code=end
