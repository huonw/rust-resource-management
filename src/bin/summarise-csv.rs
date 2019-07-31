/*!

# Exercises

# Reminder

*/

#![allow(dead_code)]

// Imports.
use std::fs;
use std::io::{BufRead, BufReader};



fn split_headings(headings_line: &str) -> Vec<&str> {
    // There's a nicer way to construct the vector of heading names
    // than the manual loop.
    //
    // In addition to adapters like `enumerate` that lazily create new
    // iterators, iterators also have methods that evaluate
    // them. `collect` is a common one, that collects all the elements
    // into a data structure, like a vector.
    //
    // The method is generic, meaning it can collect to many sorts of
    // data structures. The compiler can't automatically work out what
    // type of data structure to collect to, and so it has to be
    // specified. One way to do this is with the turbofish operator
    // `::<...>`. In this case, the element type can be inferred from
    // the iterator, so we can elide it with `_`.
    //
    // Documentation: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
    headings_line.split(',').collect::<Vec<_>>()
}

#[test]
fn test_split_headings() {
    assert_eq!(split_headings("x"), vec!["x"]);
    assert_eq!(split_headings(","), vec!["", ""]);
    assert_eq!(split_headings("x,y,z"), vec!["x", "y", "z"]);
}

struct Summary {
    count: u64,
}

impl Summary {
    fn new() -> Summary {
        Summary {
            count: 0
        }
    }

    fn add(&mut self, value: f64) {
        unimplemented!()
    }

    fn mean(&self) -> f64 {
        unimplemented!()
    }
}

fn summarise_columns(num_columns: usize, files: &mut File) -> Vec<Summary> {
    unimplemented!()
}

fn main() {
    let file_name = "data/example.csv";
    let mut file = open_file(file_name);
    let headings_line = read_line(&mut file);

    let headings: Vec<&str> = split_headings(&headings_line);
    let summaries = summarise_columns(headings.len(), &mut file);

    for (heading, summary) in headings.iter().zip(summaries) {
        println!("{}: mean = {}", heading, unimplemented!())
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
