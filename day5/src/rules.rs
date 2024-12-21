use std::fmt;

struct Rule {
    x: u32,
    y: u32,
}

impl Rule {
    fn from_str(line: &str) -> Self {
        let parts: Vec<&str> = line.split('|').collect();
        let x = parts[0].parse::<u32>().expect("Invalid number");
        let y = parts[1].parse::<u32>().expect("Invalid number");
        Rule { x, y }
    }
}

pub struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    pub fn from_lines(lines: &[String]) -> Self {
        let rules = lines.iter().map(|line| Rule::from_str(line)).collect();
        Rules { rules }
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in &self.rules {
            writeln!(f, "{}|{}", rule.x, rule.y)?;
        }
        Ok(())
    }
}