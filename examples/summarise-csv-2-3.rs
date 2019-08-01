/*!

We've now got a better understand of how to manage memory and other
resources, so let's build the remaining parts of our CSV
summarisation.

# Exercises

We're going to start small, and expand:

1. This program currently crashes when run (`cargo run --bin
   summarise-csv`), due to a few locations that are
   `unimplemented!()`. They each have some hints about what is
   required. Once they're filled out, running this program should
   print the mean of each column from `data/example.csv`.

2. It's unfortunate that we're hard coding the `data/example.csv`
   filename; in `main`, let's make it a command-line argument so that
   this program can be invoked with any CSV file, like `cargo run
   --bin summarise-csv -- data/example.csv`.

3. Now that we can work with any file, let's see test the
   performance. There's an script `generate.py` in `data/` that
   generates some big files, run it in that folder. One of the files
   it generates is the `data/huge.csv` which is 180MB and 1 million
   rows. To run our summarisation, try: `cargo run --release --example
   summarise-csv -- data/huge.csv`.  We've previously been compiling
   in debug mode, for faster compilation, but for performance work we
   should turn optimisations on, which is what `--release` does.

4. Due to the use of files, it's currently not easy to test the major
   component of this program, the `summarise_columns` function. The
   `test_summarise_columns` function needs to be completed (run with
   `cargo test --bin summarise-csv`).

5. (extension) Any error will cause our program to crash, because we
   use `unwrap` everywhere. There's at least two places that are
   likely to fail (the file may be missing, a column element may not
   be a valid number), so can we make this more reliable? `open_file`
   has some guidance, so let's start there and propagate. In `data/`,
   there's a few extra CSV files that contain potential errors that
   you can use for testing.

6. (extension) The `.lines()` iterator has to allocate a new `String`
   for every row, but it gets thrown away after processing that
   row. It is possible to read use this allocation by reading each row
   into the same buffer, using the same `read_line` method that our
   `read_line` function uses
   (https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line).
   This will require switching to a `while` loop with an explicit
   `break`. One can get this program down to using only two string
   buffers.

(Feel free to add more statistics, if you wish!)

*/

#![allow(dead_code)]

// Imports.
use std::fs;
use std::io::{BufRead, BufReader};


fn main() {
    // (Part 2) Command-line arguments can be accessed using
    // std::env::args (https://doc.rust-lang.org/std/env/fn.args.html)
    // which is an iterator over `String`s, of each command line
    // argument. We can support summarising multiple files in a single
    // run by just running over all of the arguments.
    //
    // The first "argument" is the name of the program, and the real
    // arguments start at the second. The `skip` adapter allows us to
    // ignore the leading element(s) of an iterator:
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip

    for file_name in std::env::args().skip(1) {
        println!("summary for {}", file_name);
        summarise_file(&file_name);
    }
}

fn summarise_file(file_name: &str) {
    let mut file = open_file(file_name);

    let headings_line = read_line(&mut file);
    let headings: Vec<&str> = split_headings(&headings_line);

    let summaries = summarise_columns(headings.len(), &mut file);

    // Given two iterators, we can iterate them in lock-step using the
    // `.zip` adapter
    // (https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip).
    // This creates a new iterator lazily, that yields tuples of an
    // element from each of the two iterators.
    //
    // We have to explicitly convert the headings vector to an
    // iterator (using `iter`:
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
    // to have access to the `zip` method.
    for (heading, summary) in headings.iter().zip(summaries) {
        println!("{}: mean = {}", heading, summary.mean())
    }
}


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

// Data types in Rust are declared with either the `struct` or `enum`
// keywords, for product and sum types respectively (`struct` is very
// close to the same keyword in other languages, `enum` is a
// generalisation of C, C and Java use the keyword for).
//
// Here we declare a struct with some contents.
//
// By default types cannot be duplicated, but we need to be able to do
// so, for use in the `vec!` macro below, and so we can "derive" an
// implementation of the `Clone` trait, which is how Rust does
// explicit copies.
#[derive(Clone)]
struct Summary {
    count: u64,
    sum: f64,
}

impl Summary {
    // If there's an obvious/common/simple constructor function for a
    // type, it's a Rust convention to have a factory function called
    // `new` that returns an instance. The `new` name is not
    // special. The lack of a `self` argument here means this is the
    // equivalent of a static method.
    fn new() -> Summary {
        Summary {
            count: 0,
            sum: 0.0,
        }
    }

    // A method can mutate the "receiver" or self or this, meaning the
    // object having the method called upon using `&mut self`, which
    // is a short hand for `self: &mut Self`. The `self` name is a
    // mutable reference to a value of type `Self` (which is `Summary`
    // in this case).
    //
    // This function adds a new value (in the form of a 64-bit
    // floating point number, aka `double`) to this summary.
    fn add(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
    }

    // Similarly, a method can work a borrowed or immutable reference
    // to its receiver, typically for 'const' or read-only things,
    // using `&self`.
    //
    // Similarly, a method can work a borrowed or immutable reference
    // to its receiver, typically for 'const' or read-only things,
    // using `&self`.
    //
    // You may need to add some new fields to the struct. Also, Rust
    // doesn't automatically promote numeric types: it has to be done
    // explicitly using `value as type` (e.g. `self.count as f64`).
    fn mean(&self) -> f64 {
        self.sum / self.count as f64
    }
}

fn summarise_columns(num_columns: usize, file: &mut File) -> Vec<Summary> {
    let mut summaries = vec![Summary::new(); num_columns];

    for raw_row in file.lines() {
        // reading the row may fail, so the lines iterator doesn't
        // actually yield `String` values, but instead yields
        // `Result<String, std::io::Error>`. `Result<ValueType,
        // ErrorType>` is an enum that is either a `ValueType` or an
        // `ErrorType`
        // (https://doc.rust-lang.org/std/result/enum.Result.html).
        //
        // For now, let's just assume that reading a line always
        // works, which we can do with `unwrap`: if the `Result` is an
        // error, `unwrap` will panic, crashing our
        // program. (https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap)
        let row = raw_row.unwrap();

        // We need to read each element in the comma-separated row,
        // and use it to update the appropriate summary value.
        //
        // The way to parse a string to many types is the `parse`
        // method:
        // https://doc.rust-lang.org/std/primitive.str.html#method.parse
        //
        // This is similar to `collect` discussed in `split_headings`,
        // because it returns an arbitrary/generic type, and so may
        // require the "turbofish" operator `::<>`.
        //
        // Additionally, parsing may fail, so it returns `Result`
        // too. A fine approach (for now) would be assuming that the
        // CSV file is valid, and so one potential way to parse would
        // be `element.parse::<f64>().unwrap()`.
        //
        // (The discussion of `zip` in `summarise_file` might be
        // useful.)
        for (element, summary) in row.split(',').zip(&mut summaries) {
            let value = element.parse::<f64>().unwrap();
            summary.add(value)
        }
    }

    summaries
}

#[test]
fn test_summarise_columns() {
    // (Part 4) Writing a test for `summarise_columns` is hard
    // when ut uses `File`, which relies on having an actual real file
    // on disk. It would be much nicer to be able to do it all in
    // memory.
    //
    // The functionality we're on our `BufReader<fs::File>` type all
    // comes from the `std::io::BufRead` trait
    // (https://doc.rust-lang.org/std/io/trait.BufRead.html). Other
    // types support these operations, in particular, in-memory byte
    // arrays, which have type `&[u8]`. The relationship between the
    // "slice" `&[u8]` and `Vec<u8>` is exactly the same as between
    // `&str` and `String` (a `&str` is a `&[u8]` that is valid
    // UTF-8).
    //
    // If we can switch our `summarise_columns` function to work with
    // any `BufRead`, we can test it by feeding in byte arrays.
    //
    // An easy way to do this is to switch the `file` argument to
    // `input: &mut (impl BufRead)`. The `impl` keyword is used as a
    // quantifier here: meaning "any type that implements `BufRead`".

    // A byte array can be created exactly the same way as a string
    // literal, just prefixing it with `b`. This array has 3 rows,
    // each with two elements.
    let two_by_three: &[u8] = b"2019,61\n2020,62\n2021,9999";

    unimplemented!()
}

// Provided functions and types:

fn open_file(name: &str) -> File {
    // (Part 5) `fs::File::open` returns a `std::io::Result<fs::File>`
    // which is an alias for `Result<fs::File, std::io::Error>`
    // (https://doc.rust-lang.org/std/fs/struct.File.html#method.open).
    //
    // This is Rust's main error handling type, and it is an enum, or
    // sum type, which can either be `Ok(...)` containing a real value
    // (a value of type `fs::File` in this case), or `Err(...)`
    // containing an error value (a value of type `std::io::Error`).
    //
    // Rust supports a somewhat monadic approach to error propagation,
    // using the question-mark operator `?`. This can be placed after
    // a value of type `Result`, and, if it is an error, it will
    // return the error from the function it is in, otherwise it just
    // evaluates to the
    // value. (https://doc.rust-lang.org/std/result/index.html#the-question-mark-operator-)
    //
    // Let's replace the `.unwrap()` with `?`. This will require
    // changing the of this function to return a `Result<...,
    // std::io::Error>`, wrapping the current return value. This
    // change can be repeated for each caller.
    let unbuffered = fs::File::open(name).unwrap();

    // The final return value will need to be adjusted too, using the
    // `Ok(...)` constructor to create a `Result` value containing the
    // `BufReader`.
    //
    // This change can be repeated for the `parse`-ing code, although
    // an alternative to propagating the error would be just ignoring
    // invalid values. https://doc.rust-lang.org/std/result/ has some
    // hints about how one can analyse a Result value explicitly, to
    // establish whether it is a error or not.
    BufReader::new(unbuffered)
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
