use std::fs::File;
use std::io::{BufRead, BufReader};

use autocomplete::Dictionary;

#[test]
fn inserting() {
    let mut dict = Dictionary::<usize>::new();
    assert_eq!(dict.words("A"), vec![]);

    dict.insert(String::from("A"), 1);
    assert_eq!(dict.words("A"), vec![(String::from("A"), 1)]);

    dict.insert(String::from("AA"), 2);
    assert_eq!(
        dict.words("A"),
        vec![(String::from("AA"), 2), (String::from("A"), 1),]
    );

    dict.insert(String::from("ABC"), 3);
    assert_eq!(
        dict.words("A"),
        vec![
            (String::from("ABC"), 3),
            (String::from("AA"), 2),
            (String::from("A"), 1),
        ]
    );

    // Test overwriting weights
    dict.insert(String::from("A"), 10);
    assert_eq!(
        dict.words("A"),
        vec![
            (String::from("A"), 10),
            (String::from("ABC"), 3),
            (String::from("AA"), 2),
        ]
    );
}

#[test]
fn building() {
    let filename = "dictionary.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let words: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let words_count = words.len();
    let dict = Dictionary::<usize>::build_without_weights(words);

    assert_eq!(dict.words("").len(), words_count);
    assert_eq!(dict.words("not_in_dictionary").len(), 0);
    assert_eq!(
        dict.words("test"),
        vec![
            (String::from("test"), 0),
            (String::from("testament"), 0),
            (String::from("tested"), 0),
            (String::from("tester"), 0),
            (String::from("testern"), 0),
            (String::from("testify"), 0),
            (String::from("testimonied"), 0),
            (String::from("testimonies"), 0),
            (String::from("testimony"), 0),
            (String::from("testiness"), 0),
            (String::from("testril"), 0),
            (String::from("testy"), 0),
        ]
    );
}

#[test]
fn non_ascii_strings() {
    let words = vec![String::from("úpěl")];
    let dict = Dictionary::<usize>::build_without_weights(words);

    assert_eq!(dict.words("ú"), vec![(String::from("úpěl"), 0)]);
}

#[test]
fn ordering() {
    // Test that the words are ordered by alphabetical order when weights are equal
    let words = vec![
        (String::from("AB"), 2),
        (String::from("AC"), 1),
        (String::from("AA"), 1),
        (String::from("AD"), 1),
    ];

    let dict = Dictionary::build(words);

    assert_eq!(
        dict.words("A"),
        vec![
            (String::from("AB"), 2),
            (String::from("AA"), 1),
            (String::from("AC"), 1),
            (String::from("AD"), 1),
        ]
    );
}
