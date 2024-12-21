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
        let lines = vec![
            "1|2".to_string(),
            "3|4".to_string(),
        ];
        let rules = Rules::from_lines(&lines);

        let pages = vec![1, 2, 3, 4];
        assert!(rules.is_match(&pages));
    }

    #[test]
    fn test_rules_is_match_some_rules_do_not_match() {
        let lines = vec![
            "1|2".to_string(),
            "3|4".to_string(),
        ];
        let rules = Rules::from_lines(&lines);

        let pages = vec![1, 2, 4, 3];
        assert!(!rules.is_match(&pages));
    }
   
    #[test]
    fn test_rules_is_match_empty_pages() {
        let lines = vec![
            "1|2".to_string(),
            "3|4".to_string(),
        ];
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
}