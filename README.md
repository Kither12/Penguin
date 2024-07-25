# Penguin
Penguin is a dynamically typed programming language inspired by C++, Rust, and Python. Its interpreter is built entirely in Rust and using a tree-walking approach.

My goal is to make it a usable programming language with the runtime to be reasonably fast.
# How to use
To run the interpreter you can use
```
cargo run --release --bin penguin -- test.pn
```
At the present moment, there's a lack of error reporting, and testing isn't very thorough. You can refer to test.pn for an overview of its syntax, though it currently only covers while loops and if-else statements.
