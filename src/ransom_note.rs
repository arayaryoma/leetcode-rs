/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_charts: HashMap<char, u16> = HashMap::new();
        magazine.chars().for_each(|c| {
            *magazine_charts.entry(c).or_insert(0) += 1;
        });

        ransom_note.chars().all(|c| {
            if let Some(count) = magazine_charts.get_mut(&c) {
                if *count == 1 {
                    magazine_charts.remove(&c);
                } else {
                    *count -= 1;
                }
                true
            } else {
                false
            }
        })
    }
}
// @lc code=end
