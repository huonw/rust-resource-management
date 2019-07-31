/*!
We want to understand a bit more about borrowing, by making a function that prints out each heading
independently.

There's the same provided functions for opening and reading the file as the previous exercise, but
without the "noisy" file type, and instead just a type alias for convenience.

# Exercises

The `main` function has an implementation of printing out the headers, you just need to fill out the
functions it calls:

1. implement `split_headings_example_csv`, hard coded to our particular example file "year,data".

2. implement `split_headings_loop`, which works with any headings line. Fill out the test
   `test_split_headings_loop` for it too. Make sure you use it in `main` too.

3. (extension) `split_headings_loop` is returning direct substrings copied out of `headings_line`,
   it would be nice if we didn't have to do any copies or create any new strings. Can
   `split_headings_loop` be changed somehow to return each heading as `&str`, and point directly
   into the data owned by `headings_line` in `main`?

# Reminder

A `String` can be sliced (creating a `&str` view to its data) using `&my_string[start..end]`. The
syntax with `..` (`start..end`) is a range representing the interval `[start, end)`, that is, it
includes `start` and excludes `end`.

A `&str` can be converted back into a `String` (making a `String` value that's independent of
whereever the `&str` points to) using `.to_string()`.

The `vec![a, b, ...]` syntax can be used to create a vector with elements `a`, `b`, ...

*/

#![allow(dead_code)]

// Imports.
use std::fs;
use std::io::{BufRead, BufReader};



fn split_headings_example_csv(headings_line: String) -> Vec<String> {
    // To start with, let's work with just the heading we know is in `example.csv`: "year,data". The
    // first heading starts at byte 0 and end just before byte 4, and the second starts at byte 5
    // and ends just before byte 9.

    vec![
        headings_line[0..4].to_string(),
        headings_line[5..9].to_string(),
    ]
}

fn split_headings_loop(headings_line: String) -> Vec<String> {
    // let's start with an empty vector and build it up to include whatever headings are in
    // `headings_line`.
    //
    // Documentation: https://doc.rust-lang.org/std/vec/struct.Vec.html
    let mut result = vec![];

    // the `split` function returns an iterator over `&str` slices into the original one. We can
    // iterate using a `for` loop, which looks like `for <pattern> in <iterable> {
    // ... }`. Documentation: https://doc.rust-lang.org/std/string/struct.String.html#method.split
    //
    // Other iterable things include Vec and ranges (the same `start..end` syntax used for slicing
    // strings).
    for heading in headings_line.split(',') {
        result.push(heading.to_string())
    }

    result
}

#[test]
fn test_split_headings_loop() {
    // The Rust compiler and cargo have built in support for unit tests, which are just normal
    // no-argument no-return value functions, that can be compiled and executed with `cargo test`
    // (like `cargo run`, it also supports `cargo test --bin borrowing` to run the tests just for
    // this file).

    // A test fails if it panics, like hitting `unimplemented!()` or failing an assertion. There's a
    // few assertion macros in the standard library, such as:
    //
    // - `assert!` works with any boolean condition, e.g. `assert!(foo(x) && bar(y))`.
    //   Documentation: https://doc.rust-lang.org/std/macro.assert.html
    //
    // - `assert_eq!` compares its two arguments for equality (using the `==` operator, which does
    //   value comparisons), e.g. `assert_eq!(baz(x), qux(y))`. Documentation:
    //   https://doc.rust-lang.org/std/macro.assert_eq.html

    assert_eq!(split_headings_loop("x".to_string()), vec!["x"]);
    assert_eq!(split_headings_loop(",".to_string()), vec!["", ""]);
    assert_eq!(split_headings_loop("x,y,z".to_string()), vec!["x", "y", "z"]);
}


fn main() {
    let file_name = "data/example.csv";
    let mut file = open_file(file_name);
    let headings_line = read_line(&mut file);

    let headings: Vec<String> = split_headings_loop(headings_line);

    // `headings` is a vector, and we can explicitly convert it to an iterator using the `.iter()`
    // method. This allows us to then use adapters like `.enumerate()`, which is a new lazy iterator
    // that yields tuples of the headings element and their index (starting at 0). We can
    // pattern-match directly on the tuple in the loop head.  Documentation:
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
    for (index, heading) in headings.iter().enumerate() {
        println!("heading #{} is {}", index, heading)
    }
}

// Provided functions and types:

fn open_file(name: &str) -> File {
    BufReader::new(fs::File::open(name).unwrap())
}

fn read_line(file: &mut File) -> String {
    let mut string = String::new();
    file.read_line(&mut string).unwrap();
    // remove the new line from the end
    string.trim_end_matches('\n').to_string()
}

// A type alias so that we don't have to write this type everywhere,
// writing the left-hand side in a type is exactly the same as writing
// the right-hand side, just shorter.
type File = BufReader<fs::File>;
