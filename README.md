# Penguin
Penguin is a dynamically typed programming language inspired by C++, Rust, and Python. Its interpreter is built entirely in Rust and uses a tree-walking approach.

My goal is to make it a usable programming language with a runtime that is reasonably fast.
# How to use
To run the interpreter you can use
```
cargo run --release -- examples/prime.pn
```
# Benchmarking
The table below illustrates the time it takes to run two simple programs between Penguin and Python. The benchmarks are measured using [Hyperfine](https://github.com/sharkdp/hyperfine). The code used for benchmarking is in the example folder.
| Program | Penguin  | Python |
| ------------- | ------------- | ------------- |
|  prime |  520.6 | 412.7 |
| sum  | 53.6  | 103.9  |
# Overview
```
// declaration
gimme a = 0;
gimme b = true;

//assignment
a = false;

// if-elif-else
if a == true{
}
elif b{}
else {}

// while loop
while true{
  if a{
    break;
  }
  a = true;
  b = false;
  continue;
};

//function
gimme is_even = (a) => {
  return a % 2 == 0;
};
println(is_even(100));

//pass by ref function
gimme swap = (a, b) => {
  gimme temp = a;
  a = b;
  b = temp;
}
swap(&a, &b);
println(a);
println(b);
```
