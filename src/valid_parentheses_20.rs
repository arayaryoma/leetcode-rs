/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut chars = s.chars();
        chars.all(|c| match c {
            '(' | '{' | '[' => {
                stack.push(c);
                true
            }
            ')' | '}' | ']' => {
                let top = stack.pop();
                match top {
                    Some(top) => {
                        (c == ')' && top == '(')
                            || (c == '}' && top == '{' || (c == ']' && top == '['))
                    }
                    None => false,
                }
            }
            _ => false,
        }) && stack.is_empty()
    }
}
// @lc code=end

/*
https://leetcode.com/problems/valid-parentheses/solutions/500491/rust-0ms/?source=vscode
たしかに、マッチすべき閉じ括弧の方をstackに積んでいけばいいのか
*/
