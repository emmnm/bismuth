use std::collections::HashMap;

struct LSystem {
    alphabet: Vec<char>,
    constants: Vec<char>,
    start: String,
    rules: HashMap<char, String>,
}

impl LSystem {
    /// Some guarantees of a LSystem:
    /// 1. Every char in start is in alphabet
    /// 2. Given a rule X -> Y, X and Y are in alphabet
    /// 3. Each char in alphabet has a rule that maps it to something else
    ///   -> by default a char maps to itself, but this can be overwriten with
    ///   the `push` method
    /// See https://en.wikipedia.org/wiki/L-system for more details

    pub fn new(start: String) -> LSystem {
        // push chars in start to alphabet
        // and make them map to themselves in a rule
        let mut alphabet: Vec<char> = Vec::new();
        let mut rules: HashMap<char, String> = HashMap::new();
        for c in start.chars() {
            if !alphabet.contains(&c) {
                alphabet.push(c);
                rules.insert(c, c.to_string());
            }
        }

        LSystem {
            alphabet: alphabet,
            constants: Vec::new(),
            start: start,
            rules: rules,
        }
    }

    pub fn to_string(&self) -> String {
        self.start.clone()
    }

    /// Iterate to the next LSystem string, following all rules
    pub fn next(&mut self) -> LSystem {
        let mut next = String::from("");
        for c in self.start.chars() {
            next.push_str(self.get(c));
        }

        LSystem {
            alphabet: self.alphabet.clone(),
            constants: self.constants.clone(),
            start: next,
            rules: self.rules.clone(),
        }
    }

    /// Add a character to the alphabet with the default 'identity rule'
    pub fn add(&mut self, c: char) {
        if !self.alphabet.contains(&c) {
            self.alphabet.push(c);
            self.rules.insert(c, c.to_string());
        }
    }

    /// Helper function for push
    fn overwrite_rule(&mut self, key: &char, value: &String) {
        *self.rules.get_mut(key).unwrap() = value.to_owned();
    }

    /// Add a constant to the LSystem
    /// Constants always map to themselves and cannot be changed
    pub fn add_constant(&mut self, c: char) {
        self.add(c);
        self.constants.push(c);
    }

    /// Add a new rule to the LSystem
    /// If the values in the rule do not exist, they are added
    pub fn push(&mut self, rule: HashMap<char, String>) {
        for (key, value) in &rule {
            // Make sure key is not a constant
            if !self.constants.contains(&key) {
                for c in value.chars() {
                    self.add(c);
                }
                if !self.rules.contains_key(key) {
                    self.add(*key);
                }
                self.overwrite_rule(key, value);
            }
        }
    }

    /// Returns the rule for which key maps to
    pub fn get(&self, key: char) -> &String {
        &self.rules[&key]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use LSystem;

    #[test]
    fn test_new_should_have_correct_alphabet() {
        let lsys: LSystem = LSystem::new("AB".to_string());
        assert!(lsys.alphabet.contains(&'A'));
    }

    #[test]
    fn test_new_should_have_correct_start() {
        let lsys: LSystem = LSystem::new("AB".to_string());
        assert_eq!(lsys.start, "AB");
    }

    #[test]
    fn test_new_should_not_have_duplicate_alphabet_chars() {
        let lsys: LSystem = LSystem::new("AA".to_string());
        assert_eq!(lsys.alphabet.len(), 1);
    }

    #[test]
    fn test_new_should_add_default_rules() {
        let lsys: LSystem = LSystem::new("A".to_string());
        assert_eq!(*lsys.get('A'), "A");
    }

    #[test]
    fn test_add_should_add_to_alphabet() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        lsys.add('B');
        assert!(lsys.alphabet.contains(&'B'));
    }

    #[test]
    fn test_add_should_add_default_rule() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        lsys.add('B');
        assert_eq!(*lsys.get('B'), "B");
    }

    #[test]
    fn test_add_constant_should_add_to_constants() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        lsys.add_constant('B');
        assert!(lsys.constants.contains(&'B'));
    }

    #[test]
    fn test_add_constant_should_add_to_alphabet() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        lsys.add_constant('B');
        assert!(lsys.alphabet.contains(&'B'));
    }

    #[test]
    fn test_add_constant_should_add_default_rule() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        lsys.add_constant('B');
        assert_eq!(*lsys.get('B'), "B");
    }

    #[test]
    fn test_push_should_overwrite_existing_rule() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "B".to_string());
        lsys.push(rule);
        assert_eq!(*lsys.get('A'), "B");
    }

    #[test]
    fn test_push_should_add_to_alphabet() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "B".to_string());
        lsys.push(rule);
        assert!(lsys.alphabet.contains(&'B'));
    }

    #[test]
    fn test_push_should_add_default_rules_for_new_chars() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "B".to_string());
        lsys.push(rule);
        assert_eq!(*lsys.get('B'), "B");
    }

    #[test]
    fn test_push_should_not_overwrite_constants() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        lsys.add_constant('A');
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "B".to_string());
        lsys.push(rule);
        assert_eq!(*lsys.get('A'), "A");
    }

    #[test]
    fn test_push_should_work_with_multiple_rules() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "AB".to_string());
        rule.insert('B', "A".to_string());
        lsys.push(rule);
        assert_eq!(*lsys.get('B'), "A");
        assert_eq!(*lsys.get('A'), "AB");
    }

    #[test]
    fn test_next_with_simple_rules_should_have_correct_start() {
        let mut lsys: LSystem = LSystem::new("A".to_string());
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "B".to_string());
        lsys.push(rule);
        let next: LSystem = lsys.next();
        assert_eq!(next.start, "B");
    }

    #[test]
    fn test_next_with_complex_rules_should_have_correct_start() {
        let mut lsys: LSystem = LSystem::new("AB".to_string());
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "AB".to_string());
        rule.insert('B', "A".to_string());
        lsys.push(rule);
        let next: LSystem = lsys.next();
        assert_eq!(next.start, "ABA");
    }

    #[test]
    fn test_next_should_work_in_chains() {
        let mut lsys: LSystem = LSystem::new("AB".to_string());
        let mut rule: HashMap<char, String> = HashMap::new();
        rule.insert('A', "AB".to_string());
        rule.insert('B', "A".to_string());
        lsys.push(rule);
        let next: LSystem = lsys.next().next();
        assert_eq!(next.start, "ABAAB");
    }

    #[test]
    fn test_to_string_should_return_start_string() {
        let lsys: LSystem = LSystem::new("AB".to_string());
        assert_eq!(lsys.to_string(), "AB");
    }
}
