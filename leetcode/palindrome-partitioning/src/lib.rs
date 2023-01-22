pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = vec![];
        let mut current_path = vec![];
        Self::backtrack(&s, &mut result, &mut current_path);
        result
    }

    fn backtrack(s: &str, result: &mut Vec<Vec<String>>, current_path: &mut Vec<String>) {
        let current_path_len = current_path.iter().fold(0, |acc, s| acc + s.len());
        // if reached end of string, push path ro result
        if current_path_len == s.len() {
            result.push(current_path.clone());
            return;
        }

        for segment_end_index in current_path_len..s.len() {
            let segment = &s[current_path_len..=segment_end_index];
            if !Self::is_palindrome(segment) {
                continue;
            }
            current_path.push(segment.to_string());
            Self::backtrack(&s, result, current_path);
            current_path.pop();
        }
    }
    fn is_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = Solution::partition("aab".to_string());
        assert_eq!(result, vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
    }

    #[test]
    fn example2() {
        let result = Solution::partition("a".to_string());
        assert_eq!(result, vec![vec!["a"]]);
    }

    #[test]
    fn example3() {
        let result = Solution::partition("aabb".to_string());
        assert_eq!(
            result,
            vec![
                vec!["a", "a", "b", "b"],
                vec!["a", "a", "bb"],
                vec!["aa", "b", "b"],
                vec!["aa", "bb"],
            ]
        );
    }
}
