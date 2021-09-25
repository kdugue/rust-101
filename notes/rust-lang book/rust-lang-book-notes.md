## The Rust Programming Language

### Introduction

- Cargo: package manager and build tool
- Rustfmt: formatter
- zero-cost abstractions:

### Chapter 1 - Getting Started

- Rust files always have `.rs` extension
- `main` function: always the first code that runs in every executable program; entry point into program
- `!` indicates a macros being called instead of a function
- macros !== functions
- `;` = expression is over and next one is ready to begin
- Cargo
  - handles building code, downloading dependent libraries, and building libraries
- packages are known as _crates_

### Chapter 2 - Programming a Guessing Game

- `cargo new {project_name}`: creates a new cargo project
- `std`: standard library
- for importing packages not in the prelude, use `use`
- variables are immutable by default

```rust
let applesCount = 5; //immutable
let mut bananasCount = 5; //mutable
```

- associated functions are implemented on a type

```rust
let  mut  guess = String::new();

// 'new' is an associated function
// on the String type
```

- `&` indicates a references
  - references give a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times
  - references are immutable by default
- enumerations: types that can have a fixed set of values; each value is called a variant
- crate: collection of Rust source code files
  - binary crate: executable file
  - library crate: contains code intended to be used in other programs
- crates.io: rust devs post their open source projects for others to use
- cargo makes it easy to reuse libraries
- arm: consists of a pattern and the code that should be run if the value given to the beginning of the expressions fits that arm's pattern
- rust allows for shadowing of a previous value of a variable with a new one

```rust
let  mut  guess = String::new();
let  guess: u32 = guess.trim().parse().expect("Please type a number!");
```

### Chapter 3 - Common Programming Concepts

3.1 Variables and Mutability

- variables are immutable (can't change value) by default
- compiler guarantees that when you state a value wont change, it really won't change
- shadowing

```rust
let x = 5;
let x = x + 1;
let x = x * 2;

// shadowing allows to perform a few transformations
// on a value but have the variable be immutable after those
// transformations have been completed
```

3.2 Data Types

- rust is statically typed: it must know the types of all variables at compile time

**Scalar type**: single value

- rust has 4 primary scalar types:
  1.  integers
  2.  floating-point numbers
  3.  Booleans
  4.  characters

**Compound** types: group multiple values into one type

1. **tuples**: groups together a number of values with a variety of types into one compound type
   - fixed length

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

2. **arrays**: elements must be of the same type
   - fixed length
   - useful for when you want your data allocated on the stack rather than the heap

3.3 Functions

- location of function definitions are don't matter
- function parameters must have a type declared
  statements vs. expressions
- statements - perform some action and do not return a value - include ending semicolons

```rust
let x = 5;
```

- expressions - evaluate to a resulting value - do not include ending semicolons (if `;` gets added, then it becomes a statement, which don't return a value)

```rust
fn five() -> i32 {
	5 // 5 is expression, no semi-colon;
}
```

3.4 Comments

3.5 Control Flow

- arms: blocks of code associated with the conditions in `if` expressions
- rust doesn't automatically convert non-Boolean types to a Boolean
- numbers = expressions
- variables must have a single type
- looking through element with `for` loop

```rust
fn main() {
	let a = [10, 20, 30, 40, 50];

	for element in a.iter() {
		println!("the value is: {}", element);
	}
}

```

### Chapter 4 - Understanding Ownership

- ownership enables Rust to make memory safety guarantees without needing a garbage collector

  4.1 What is Ownership?

- memory is managed through a system of ownership with a set of tules that the compiler checks at compile time
- data with an unknown size at compile time or a size that might change must be stored on the heap instead (will not be stored on the stack)
- ownership rules
  - each value has a variable that's called its owner
  - there can only be one owner at a time
  - when the owner goes out of scope, the value will be dropped

```rust
{                    // s is not valid here, it's not yet declared
	let s = "hello"; // s is valid from this point forward

	// do stuff with s
} // this scope is now over, s is no longer valid
```

The `String` type - different from string literals (`let s = "hello"`), which are hardcoded values - `String` is allocated on heap and can store a variable amount of text

```rust
let s = String::from("hello");
// :: is an operator that allows us to namespace

s.push_str(", world!"); // appends a literal to a String

println!("{}", s); // hello, world!
```

- `String` vs string literal
  - literal: since we content contents at compile time, these are faster and more efficient; they are immutable;
  - string literals are string slices
  - `String` is mutable, an amount of memory needs to be allocated onto the heap, which is unknown at compile time
- memory is automatically returned once the variable that owns it goes out of scope
- when a variable goes out of scope, `drop` is called
  - `drop` is automatically called at the closing curly bracket
- `String` is made up of 3 parts (all stored on the stack):
  1.  `ptr`: pointer to the memory that holds the contents of the string
      - memory is stored on the heap that holds the contents
  2.  `len`: length
  3.  `capacity`: capacity

```rust
let s1 = String::from("hello");
let s2 = s1;

// when s1 is assigned to s3, `String` data is copied, meaning
// we copy the pointer, the length, and the capacity that are
// on the stack. We do not copy the data on the heap that the pointer
// refers to

// s1 was "moved" to s2
// With only s2 vaid, when it goes out of scope, it alone
// will free the memory, and we're done
```

- `clone`: allows for a deep copy of the heap data of the `String`

- types that have a known size at compile time are stored entirely on the stack; copies of the actual values are quick to make

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
// x is valid and was not moved into y
```

```rust
fn  main() {
	let  s = String::from("hello"); // s comes into scope

	takes_ownership(s); // s's value moves into the function...
						// ... and so is no longer valid here

	let  x = 5; // x comes into scope

	makes_copy(x);  // x would move into the function,
					// but i32 is Copy, so it's okay to still
					// use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.



fn  takes_ownership(some_string: String) {// some_string comes into scope
	println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn  makes_copy(some_integer: i32) {// some_integer comes into scope
	println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

- returning values can also transfer ownership; assigning a value to another variable moves it
- when a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless the data has been moved to be owned by another variable
- 4.2 References and Borrowing

- `&` = references, allow you to refer to some value without taking ownership of it
- rules of references - at any given time, you can have _either_ one mutable reference _or_ any number of immutable references - references must always be valid
  Reference example

```rust
fn main() {
	let s1 = String::from("hello");
	let len = calculate_length(&s1);

	// &s1 creates a reference that refers to the value of s1 but
	// does not own it. Because it does not own it, the value
	// it points to will not be dropped when the reference
	// goes out of scope

	println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
	s.len()
}
```

- when functions have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership
- borrowing: references as function parameters

```rust
fn main() {
	let s = String::from("hello");
	change(&s);
}

// incorrect, will not compile
fn change(some_string: &String) {
	some_string.push_str(", world");
	// this is illegal and will not work. References are immutable by default
	// We're not allowed to modify something we have a reference to
}

// correct
fn main() {
	let mut s = String::from("hello");
	change(&mut s);
}

fn change(some_string: &mut String) {
	some_string.push_str(", world");
	// by making some_string argument mutable via the &mut keyword
	// this will compile
}
```

- for mutable references, you can have only one mutable reference to a particular piece of data in a particular scope.
  - this helps prevent data races at compile time

Example of too many mutable references in scope -

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
// error[E0499]: cannot borrow `s` as mutable more than once at a time
```

4.3 The Slice Type
Slice

- does not have ownership
- string slice: reference to a part of a `String`

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// 'hello' and 'world' reference a portion of the String
```
