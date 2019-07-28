/*!
We want to get a sense of how Rust manages resources.

This exercise contains `open_file` and `read_line`, as well as a "noisy" File type, that prints a
message when it is being closed to help demonstrate the clean-up phase of resource management.

There's a variety of set-up code, that can and should be skipped to start with. The lengthy comments
are explanations if you wish to understand, and for future reference.

# Exercises

There's three things to do here, in `main` below:

1. open the `data/examples.csv` file, and get a sense of when it gets closed. Feel free to define
   your own functions that take a `file: File` argument (and return them), and scatter `println!`s!
   The file value will be closed when the variable holding it goes out of scope (like the end of a
   function).

2. just open the file (no need to play with its closing time), read the first line of the file (the
   headings), and print it out

3. (extension) read the second line (the first row of data), and print it out too

# Reminder

The way to print is `println!("<format string>", values, to, format);`, and {}s specify the location
of each value. For example:

```
println!("hello {}, {}", "world", 123);
// hello world, 123
```

*/

// We won't be using all the code all the time, so the warning about code that we never call isn't
// helpful. This attribute (#) is attached to this whole file (!), and "allows" the `dead_code` lint
// in the compiler, so that it doesn't emit any warnings. (Feel free to comment out or remove this
// line, to see what the compiler can flag!)
#![allow(dead_code)]

// Imports, to bring types/functionality from the `std` crate (the standard library) into scope in
// this file/module).

// The fs (filesystem) module contains `File` for IO to a file
use std::fs;
// The io (input/output) module contains general IO functionally, like: `BufReader` allows doing
// batch reads using an in-memory buffer, `BufRead` is a trait that abstracts across all the types
// that behave like this. (`self` refers to the module itself, and is the same as a separate `use
// std::io;`, allowing us to use `io::...` in the rest of the code.)
use std::io::{self, BufRead, BufReader};

// Rust binaries start at the `main` function.
fn main() {
    let file_name = "data/example.csv";
    // let file = ...
}


// Provided functions and types:

// Open the file `name`. The `&str` is a "string slice" or "borrowed string", which is a borrowed
// reference to a sequence of UTF-8 bytes somewhere (anywhere) in memory. It's the type of a string
// literal (a pointer into read-only memory in the program code itself), and can be created cheaply
// from a dynamically allocated string like String.
fn open_file(name: &str) -> File {
    File { inner: BufReader::new(fs::File::open(name).unwrap()) }
}

// Read a line from the file. Reading requires mutation (e.g. the buffer may be refilled with new
// data), so we have to mark the file variable as "mut"-able.
fn read_line(mut file: File) -> String {
    let mut string = String::new();
    file.read_line(&mut string).unwrap();
    string
}


// A custom type just to have the "noisy" drop (aka destructor or finalizer) that is used for
// closing and cleaning-up resources. For this type, we just print a message to indicate the point
// at which the file is being closed. The drop of the inner type is run automatically, which
// includes closing the underlying operating-system file resource.
struct File {
    inner: BufReader<fs::File>
}
// impl = "implementation" is how one adds methods to types. It can be used for "inherent" methods
// like here, which are functions associated with a single particular type ('File' in this case).
impl File {
    // Reproducing the line-reading API from BufRead exactly, so that this type behaves like
    // BufReader for the purposes of this exercise.
    fn read_line(&mut self, string: &mut String) -> io::Result<usize> {
        self.inner.read_line(string)
    }
}

// `impl` it is also used to implement a trait (similar to interfaces, type classes or protocols in
// other languages) for a type, which are the main route for polymorphism, where a type satisfies
// the rules for a particular set of behaviours (a trait packages up those behaviours and rules and
// gives them a name). `Drop` is one example of a trait in std (a very special one), and we can use
// it to add clean-up that runs just as values of type `File` are being cleaned-up.
impl Drop for File {
    fn drop(&mut self) {
        println!("*** closing file now ***")
    }
}
