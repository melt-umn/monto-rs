use aho_corasick::{AcAutomaton, Automaton};

pub fn find_todos(src: &str) -> Vec<(usize, usize)> {
    AcAutomaton::new(vec!["todo", "TODO"])
        .find(src)
        .map(|m| (m.start, m.end))
        .collect()
}
