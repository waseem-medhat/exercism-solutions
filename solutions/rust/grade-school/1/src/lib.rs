use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    roster: BTreeMap<u32, BTreeSet<&'static str>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'static str) {
        if self.contains(student) {
            return;
        }
        let entry = self.roster.entry(grade).or_default();
        entry.insert(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.roster.get(&grade) {
            None => vec![],
            Some(set) => set.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn contains(&self, student: &'static str) -> bool {
        self.roster.iter().any(|(_, set)| set.contains(student))
    }
}
