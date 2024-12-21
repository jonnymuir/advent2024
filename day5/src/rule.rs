use std::fmt;

pub struct Rule {
    x: u32,
    y: u32,
}

impl Rule {
    pub fn from_str(line: &str) -> Self {
        let parts: Vec<&str> = line.split('|').collect();
        let x = parts[0].parse::<u32>().expect("Invalid number");
        let y = parts[1].parse::<u32>().expect("Invalid number");
        Rule { x, y }
    }

    pub fn is_match(&self, pages: &Vec<u32>) -> bool {
        // Get the first position of the first occurrence of x, or if it doesn't exist, return true
        match pages.iter().position(|&page| page == self.x) {
            Some(xpos) => match pages.iter().position(|&page| page == self.y) {
                Some(ypos) => xpos < ypos,
                None => true,
            },
            None => true,
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}|{}", self.x, self.y)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_is_match_x_before_y() {
        let rule = Rule { x: 1, y: 2 };
        let pages = vec![1, 2];
        assert!(rule.is_match(&pages));
    }

    #[test]
    fn test_rule_is_match_x_after_y() {
        let rule = Rule { x: 1, y: 2 };
        let pages = vec![2, 1];
        assert!(!rule.is_match(&pages));
    }

    #[test]
    fn test_rule_is_match_x_and_y_not_in_order() {
        let rule = Rule { x: 1, y: 2 };
        let pages = vec![3, 1, 2];
        assert!(rule.is_match(&pages));
    }

    #[test]
    fn test_rule_is_match_x_and_y_not_present() {
        let rule = Rule { x: 1, y: 2 };
        let pages = vec![3, 4, 5];
        assert!(rule.is_match(&pages));
    }
}
