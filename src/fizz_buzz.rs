/*
 * @lc app=leetcode id=412 lang=rust
 *
 * [412] Fizz Buzz
 */

// @lc code=start
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|x| match x {
                x if x % 15 == 0 => "FizzBuzz".to_string(),
                x if x % 3 == 0 => "Fizz".to_string(),
                x if x % 5 == 0 => "Buzz".to_string(),
                _ => x.to_string(),
            })
            .collect()
    }
}
// @lc code=end
