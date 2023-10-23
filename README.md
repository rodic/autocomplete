# Autocomplete

## Description

The Dictionary struct stores a collection of words, each with its corresponding weight. In this structure, each word is represented as a path within a tree, where each node along the path corresponds to a character in the word. Terminal nodes hold both the word itself and its weight. These terminal nodes store the complete word to expedite lookups, eliminating the need for backtracking and reconstructing the word from individual characters.

```rust
pub struct Dictionary<T> {
    entries: HashMap<char, Dictionary<T>>,
    weight: Option<T>,
    word: Option<String>
}
```

For example, words
```
A, 1
AA, 2
ABC, 3
```

Would be represented as

```rust
Dictionary {
    weight: None,
    word: None,
    entries: {
        'A': Dictionary {
            weight: Some(1),
            word: Some("A"),
            entries: {
                'A': Dictionary {
                    weight: Some(2),
                    word: Some("AA")
                    entries: {},
                },
                'B': Dictionary {
                    weight: None,
                    word: None,
                    entries: {
                        'C': Dictionary {
                            weight: Some(3),
                            word: Some("ABC"),
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
    ("AA".to_string(), 2),
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
//     ("ABC", 3), 
//     ("AA", 2), 
//     ("A", 1), 
// ]
```

## License

The MIT License (MIT)

Copyright © 2023 <copyright holders>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
