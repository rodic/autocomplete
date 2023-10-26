# Autocomplete
[<img alt="github" src="https://img.shields.io/badge/github-rodic/autocomplete-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rodic/autocomplete)
[<img alt="crates.io" src="https://img.shields.io/crates/v/autocomplete.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/autocomplete)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-autocomplete-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/autocomplete)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/rodic/autocomplete/rust.yml?branch=master&style=for-the-badge" height="20">](https://github.com/rodic/autocomplete/actions?query=branch%3Amaster)

## Description

The `Dictionary` struct stores a collection of words, each with its corresponding weight. In this structure, each word is represented as a path within a tree, where each node along the path corresponds to a character in the word. Terminal nodes hold both the word itself and its weight. 

These terminal nodes store the complete word to expedite lookups, eliminating the need for backtracking and reconstructing the word from individual characters. Weights are used to arrange entries within the resulting vector, wherein words with greater weight will be positioned at the beginning.

```rust
pub struct Dictionary<T> {
    entries: BTreeMap<char, Dictionary<T>>,
    terminal: Option<Terminal<T>>,
}

struct Terminal<T> {
    weight: T,
    word: String,
}
```

For example, words
```
A, 1
AA, 5
ABC, 3
```

Would be represented as

```rust
Dictionary {
    terminal: None,
    entries: {
        'A': Dictionary {
            terminal: Some({
                weight: 1,
                word: "A",
            })
            entries: {
                'A': Dictionary {
                    terminal: Some({
                        weight: 5,
                        word: "AA"
                    }),
                    entries: {},
                },
                'B': Dictionary {
                    terminal: None,
                    entries: {
                        'C': Dictionary {
                            termainal: Some({
                                weight: 3,
                                word: "ABC",
                            })
                            entries: {},
                        },
                    },
                },
            },
        },
    },
}
```

## Usage

Both `build` and `build_without_weights` methods provide a convenient way to initialize and construct a `Dictionary` object from a list of words, either with or without weights. These methods abstract away the details of manually inserting each word into the dictionary, making it easier to create and work with `Dictionary` objects in your code.

```rust
// Example using build method
let words_with_weights = vec![
    ("A".to_string(), 1),
    ("AA".to_string(), 5),
    ("ABC".to_string(), 3),
];

let dictionary = Dictionary::<usize>::build(words_with_weights);

// Example using build_without_weights method
let words = vec![
    "A".to_string(),
    "AA".to_string(),
    "ABC".to_string(),
];

let dictionary_unweighted = Dictionary::<usize>::build_without_weights(words);
```
To query the `Dictionary` by prefix, use `words` methods.
```rust
// Example using words
dictionary.words("A")
// [
//     ("AA", 5), 
//     ("ABC", 3), 
//     ("A", 1), 
// ]
```

## Installation

The package is available at [crates.io](https://crates.io/crates/autocomplete)

## License

This project is licensed under the MIT license. Please see the [LICENSE](LICENSE) file for more details.
