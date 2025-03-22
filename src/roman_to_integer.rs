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
                'V' => {
                    if let Some(&'I') | Some(&'V') | None = iter.peek() {
                        result += 5
                    } else {
                        result -= 5
                    }
                }
                'X' => {
                    if let Some(&'I') | Some(&'V') | Some(&'X') | None = iter.peek() {
                        result += 10
                    } else {
                        result -= 10
                    }
                }
                'L' => {
                    if let Some(&'I') | Some(&'V') | Some(&'X') | Some(&'L') | None = iter.peek() {
                        result += 50
                    } else {
                        result -= 50
                    }
                }
                'C' => {
                    if let Some(&'I') | Some(&'V') | Some(&'X') | Some(&'L') | Some(&'C') | None =
                        iter.peek()
                    {
                        result += 100
                    } else {
                        result -= 100
                    }
                }
                'D' => {
                    if let Some(&'I') | Some(&'V') | Some(&'X') | Some(&'L') | Some(&'C')
                    | Some(&'D') | None = iter.peek()
                    {
                        result += 500
                    } else {
                        result -= 500
                    }
                }
                'M' => {
                    if let Some(&'I') | Some(&'V') | Some(&'X') | Some(&'L') | Some(&'C')
                    | Some(&'D') | Some(&'M') | None = iter.peek()
                    {
                        result += 1000
                    } else {
                        result -= 1000
                    }
                }
                _ => (),
            }
        }
        result
    }
}
// @lc code=end
