struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.sort_unstable();
        let mut t_chars: Vec<char> = t.chars().collect();
        t_chars.sort_unstable();
        s_chars == t_chars
    }
}

fn main() {
    let valid_case = [("anagram", "nagaram", true), ("rat", "car", false)];

    for data in valid_case {
        let result = Solution::is_anagram(data.0.to_string(), data.1.to_string());
        assert_eq!(data.2, result);
    }
}
