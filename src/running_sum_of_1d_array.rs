/*
 * @lc app=leetcode id=1480 lang=rust
 *
 * [1480] Running Sum of 1d Array
 */

// @lc code=start
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |acc, &x| {
                *acc += x;
                println!("{} ", *acc);
                Some(*acc)
            })
            .collect()
    }
}
// @lc code=end
