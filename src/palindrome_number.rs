/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(n: i32) -> bool {
        let mut m = n;
        if n < 0 || (n % 10 == 0 && n != 0) {
            return false;
        }

        let mut reversed = 0;
        while m > 0 {
            reversed = reversed * 10 + m % 10;
            println!("reversed: {}", reversed);
            println!("m: {}", m);
            m /= 10;
        }
        n == reversed
    }
}
// @lc code=end
