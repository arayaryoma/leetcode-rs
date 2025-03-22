/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut iter = s.chars().peekable();
        let mut result = 0;

        while let Some(c) = iter.next() {
            match c {
                'I' => {
                    if let Some(&'I') | None = iter.peek() {
                        result += 1
                    } else {
                        result -= 1
                    }
                }
                'V' => result += 5,
                'X' => {
                    if let Some(&'L') | Some(&'C') = iter.peek() {
                        result -= 10
                    } else {
                        result += 10
                    }
                }
                'L' => result += 50,
                'C' => {
                    if let Some(&'D') | Some(&'M') = iter.peek() {
                        result -= 100
                    } else {
                        result += 100
                    }
                }
                'D' => result += 500,
                'M' => result += 1000,
                _ => (),
            }
        }
        result
    }
}
// @lc code=end
