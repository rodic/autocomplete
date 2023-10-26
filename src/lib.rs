//! A Rust Auto-complete feature using Trie data structure.
//!
//! # Examples
//!
//! ```
//! use autocomplete::Dictionary;
//!
//! let mut dict = Dictionary::<usize>::new();
//!
//! dict.insert(String::from("A"), 1);
//! dict.insert(String::from("AA"), 5);
//! dict.insert(String::from("ABC"), 3);
//!
//! assert_eq!(
//!   dict.words("A"),
//!   vec![(String::from("AA"), 5),(String::from("ABC"), 3),(String::from("A"), 1)]
//! );
//!
//! assert_eq!(dict.words("B"), vec![]);
//! ```
use std::collections::BTreeMap;

pub struct Dictionary<T> {
    entries: BTreeMap<char, Dictionary<T>>,
    terminal: Option<Terminal<T>>,
}

struct Terminal<T> {
    weight: T,
    word: String,
}

impl<T> Dictionary<T>
where
    T: Ord + Default + Copy,
{
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            terminal: None,
        }
    }

    pub fn build(words: Vec<(String, T)>) -> Self {
        words
            .into_iter()
            .fold(Self::new(), |mut dict, (word, weight)| {
                dict.insert(word, weight);
                dict
            })
    }

    pub fn build_without_weights(words: Vec<String>) -> Self {
        let weighted_words = words.iter().map(|w| (w.clone(), T::default())).collect();
        Self::build(weighted_words)
    }

    pub fn insert(&mut self, word: String, weight: T) {
        let dict = word.chars().fold(self, |dict, c| {
            dict.entries.entry(c).or_insert_with(Self::new)
        });
        dict.terminal = Some(Terminal { weight, word });
    }

    fn to_words(&self, result: &mut Vec<(String, T)>) {
        if let Some(Terminal { word, weight }) = &self.terminal {
            result.push((word.clone(), *weight));
        }
        self.entries
            .iter()
            .for_each(|(_, dict)| dict.to_words(result));
    }

    pub fn words(&self, prefix: &str) -> Vec<(String, T)> {
        prefix
            .chars()
            .try_fold(self, |dict, c| dict.entries.get(&c))
            .map_or_else(
                || Vec::new(),
                |dict| {
                    let mut result = Vec::new();
                    dict.to_words(&mut result);
                    result.sort_by(|(_, w1), (_, w2)| w2.cmp(w1));
                    result
                },
            )
    }
}
