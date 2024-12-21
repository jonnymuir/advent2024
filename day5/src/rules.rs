use crate::rule::Rule;
use std::fmt;

pub struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    pub fn from_lines(lines: &[String]) -> Self {
        let rules = lines.iter().map(|line| Rule::from_str(line)).collect();
        Rules { rules }
    }

    pub fn is_match(&self, pages: &Vec<u32>) -> bool {
        self.rules.iter().all(|rule| rule.is_match(pages))
    }

    pub fn correct(&self, pages: &Vec<u32>) -> Vec<u32> {
        let corrected = self
            .rules
            .iter()
            .fold(pages.clone(), |corrected_pages, rule| {
                rule.correct(&corrected_pages)
            });

        // Sometimes correcting on rule will knock another rule out of place
        // This can be corrected by running the correction again
        if self.is_match(&corrected) {
            corrected
        } else {
            self.correct(&corrected)
        }
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in &self.rules {
            writeln!(f, "{}", rule)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules_is_match_all_rules_match() {
        let lines = vec!["1|2".to_string(), "3|4".to_string()];
        let rules = Rules::from_lines(&lines);

        let pages = vec![1, 2, 3, 4];
        assert!(rules.is_match(&pages));
    }

    #[test]
    fn test_rules_is_match_some_rules_do_not_match() {
        let lines = vec!["1|2".to_string(), "3|4".to_string()];
        let rules = Rules::from_lines(&lines);

        let pages = vec![1, 2, 4, 3];
        assert!(!rules.is_match(&pages));
    }

    #[test]
    fn test_rules_is_match_empty_pages() {
        let lines = vec!["1|2".to_string(), "3|4".to_string()];
        let rules = Rules::from_lines(&lines);

        let pages = vec![];
        assert!(rules.is_match(&pages));
    }

    #[test]
    fn test_rules_is_match_empty_rules() {
        let lines = vec![];
        let rules = Rules::from_lines(&lines);

        let pages = vec![1, 2, 3, 4];
        assert!(rules.is_match(&pages));
    }

    #[test]
    fn test_rule_correct_when_not_matching() {
        let lines = vec!["1|2".to_string()];
        let rules = Rules::from_lines(&lines);

        let pages = vec![2, 1];
        let corrected_pages = rules.correct(&pages);
        assert_eq!(corrected_pages, vec![1, 2]);
    }

    #[test]
    fn test_rule_correct_when_multiple_not_matching() {
        let lines = vec!["1|2".to_string(), "3|4".to_string()];
        let rules = Rules::from_lines(&lines);

        let pages = vec![2, 1, 4, 3];
        let corrected_pages = rules.correct(&pages);
        assert_eq!(corrected_pages, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_rule_correct_when_multiple_not_matching_require_recursion() {
        let lines = vec!["1|2".to_string(), "3|4".to_string(), "4|1".to_string()];
        let rules = Rules::from_lines(&lines);

        let pages = vec![2, 4, 1, 3];
        let corrected_pages = rules.correct(&pages);
        assert_eq!(corrected_pages, vec![3, 4, 1, 2]);
    }
}
