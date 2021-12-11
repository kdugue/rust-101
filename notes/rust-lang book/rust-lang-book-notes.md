## The Rust Programming Language

### Introduction

- Cargo: package manager and build tool
- Rustfmt: formatter
- zero-cost abstractions: you can write human-friendly high level code and the compiler will give you for free performance at least as good as any optimized low level code you could have written yourself

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
let applesCount = 5; // immutable
let mut bananasCount = 5; // mutable
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
- crate: collection of rust source code files
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

**3.1 Variables and Mutability**

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

**3.2 Data Types**

- rust is statically typed: it must know the types of all variables at compile time

**Scalar type**: single value

- rust has 4 primary scalar types:
  1.  integers
      - `i32` is default integer type
  2.  floating-point numbers
  3.  Booleans
  4.  characters

**Compound** types: group multiple values into one type

1. **tuples**: groups together a number of values with a variety of types into one compound type
   - fixed length

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
// i32 is default integer type
```

2. **arrays**: elements must be of the same type
   - fixed length
   - useful for when you want your data allocated on the stack rather than the heap

**3.3 Functions**

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

**3.4 Comments**

**3.5 Control Flow**

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
- Rust gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

**4.1 What is Ownership?**

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

**4.2 References and Borrowing**

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

**4.3 The Slice Type**
Slice

- does not have ownership
- string slice: reference to a part of a `String`

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// 'hello' and 'world' reference a portion of the String
```

### Chapter 5 - Using Structs to Structure Related Data

- struct: custom data types for grouping together multiple related values

**5.1 Defining and Instantiating Structs**

```rust
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool.
}

let mut user1 = User {
	email: String::from("someone@example.com"),
	username: String::from("someusername123"),
	active: true,
	sign_in_count: 1,
}

user1.email // "someone@example.com"
user1.email = String::from("another@example.com");
user1.email // "another@example.com"
```

- tuple structs: have the added meaning the struct name provides, but don't have names associated with their fields; rather they have the types of the fields

```rust
// tuple struct example
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

- unit-like structs: structs that don't have any fields
- `String` = owned type; `&str` = reference

**5.2 An Example Program Using Structs**

- `#[derive(Debug)]` allows to print out debugging information for struct

**5.3 Method Syntax**

- defined within the context of a struct/enum/trait object
- first parameter is `self` (represents the instance of the struct the method is being called on)

```rust
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!(
		"The area of the rectangle is {} square pixels.",
		rect1.area()
	);
}
```

- methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably
- benefit of using methods over functions: organization
- immutable borrow: when you just need to read from struct and maintain ownership of the original struct, so it can be used again after calling a method
- mutable borrow: when you want to write to struct
- associated functions: functions within `impl` blocks that don't take `self` as a parameter
  - they're still _associated_ with the struct
  - not methods because they don't have an instance of the struct to work with
  - often used for constructors that will return a new instance of the struct

```rust
impl Rectangle {
// associated function
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size
		}
	}
}


let sq = Rectangle::square(3);
```

- a struct can have multiple `impl` blocks
- associated functions let you namespace functionality that is particular to your struct without having an instance available
  - not tied to an instance of a struct
  - do not have a `self` argument
- define methods on structs via `impl`

### Chapter 6 - Enums and Pattern Matching

- enums: allow defining a type by enumerating all possible variants

**6.1 Defining an Enum**

- can use `impl` to define methods on enums

```rust
enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		// method body would be defined here
	}
}

let m = Message::Write(String::from("hello"));
m.call();
```

- `Option`: enum that state whether a value is present or absent

```rust
enum Option<T> {
	None,
	Some(T),
}

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}
```

- everywhere that a value has a type that isn't an `Option<T>`, you can assume that the value isn't null

**6.2 The `match` Control Flow Operator**

- `match`: compare a value against a series of patterns and then execute code based on which pattern matches

```rust
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25
	}
}
```

- matches are _exhaustive_: we must exhaust every last possibility in order for the code to be valid
- `_` pattern will match any value
- `()` is unit value; nothing happens

```rust
let some_u8_value = 0u8;
match some_u8_value {
	1 => println!("one"),
	3 => println!("three"),
	5 => println!("five"),
	7 => println!("seven"),
	_ => (),
}
```

**6.3 Concise Control Flow with `if let`**

- `if let`: `match` that runs code when the value matches one pattern and then ignores all other values

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
	println!("three");
}
```

### Chapter 7 - Managing Growing Projects with Packages, Crates, and Modules

- package can contain multiple binary crates and optionally one library crate
- rust's module system:
  - packages: build, test, and share crates
  - crates: modules that produces a library or executable
  - modules and `use`:control organization, scope, and privacy of paths
  - paths: a way of naming an item

**7.1 Packages and Crates**

- crate: binary or library
- package: one or more crates that provides a set of functionality
  - contains a `Cargo.toml` (file that describes how to build those crates)
  - can contain at most one library crate
  - can contain as many binary crates as you'd like
- package that only contains `src/main.rs`: only contains binary crate
- package that contains both `src/main.rs` and `src/lib.rs` has 2 crates: (1) library crate, and (2) binary crate
- crate's functionality is namespaced in its own scope; compiler does not get confused about which trait/struct belongs to which crate

**7.2 Defining Modules to Control Scope and Privacy**

- modules
  - organize code within a crate into groups for readability and reuse
  - control _privacy_ of items (`public` vs `private`)
  - group related definitions together and name why they're related
- module A contained inside module B also means:
  - module A is the child of module B
  - module B is the parent of module A

```rust
// module example
mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {}
		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}
		fn serve_order() {}
		fn take_payment() {}
	}
}

```

**7.3 Paths for Referring to an Item in the Module tree**

- path has 2 forms:
  1.  absolute path: using crate root via crate name, or a literal `crate`
  2.  relative path: starts from current module; uses `self`, `super`, or an identifier in the current module
- modules define Rust's privacy boundary
- functions, methods, structs, enums, modules, and constants are private by default
  - module system hides inner implementation details by default
- `pub` (public): exposes inner parts of child modules' code to outer ancestor modules
- making a module public doesn't make its contents public; `pub` on module only lets code in its ancestor modules refer to it
- by default, enum variants are public; once enum is set to `pub`, all of its variants are public (you don't have to annotate each variant)

```rust
pub enum Appetizer {
	Soup,
	Salad
}

// Soup is public
// Salad is public

```

- by default, structs and its fields are private, you have to

```rust
pub struct Breakfast {
	pub toast: String,
	seasonal_fruit: String
}

// toast is public
// seasonal_fruit is private
```

**7.4 Bringing Paths into Scope with the `use` Keyword**

- `use` : import other crates and use their functionality
- ex-exporting: bringing an item into scope but also making the item available for others to bring into their scope
- standard library (`std`) ships with Rust language
- `*` glob operator:
  - `use std::collections::*;` : brings all public items defined in `std::collections` into the current scope

**7.5 Separating Modules into Different Files**

### Chapter 8 - Common Collections

- _collections_: standard library for data structures
- data in _collections_ is stored on the heap, data can grow or shrink (as opposed to having the data stored on a stack)
- common collections:
  - vectors: store a variable number of values in ordered fashion
  - strings: collection of characters
  - hash map: key-value pairs

**8.1 Storing Lists of Values with Vectors**
vectors: - known as `Vec<T>`; `<T>` indicates that any type can be stored - can only store values of the same type - indexed, starting with 0

```rust
// create vector of i32 type
let v: Vec<i32> = Vec::new();

// initialize vector
let mut v = vec![1, 2, 3];

// interacting with vector
v.push(4);
v.push(5);

// accessing values: index approach
let thirdElement =: &i32 = &v[2];

// accessing values: get method
let thirdElement = v.get(2);

// iterating over immutable vector
let nums = vec![100, 200, 300];
for i in &v {
	println!("num is {}", i);
}

// iterating over mutable vector
let mut v = vec![50, 100, 150];
for i in &mut v {
	*i += 50;
	// *(dereference operator) because we are chaning the value that
	// the mutable reference refers to
}
```

**8.2 Storing UTF-8 Encoded Text with Strings**

- `String` vs `str`
  - `String`:
    - available in the standard library
    - growable, mutable, owned type
  - `str` (string slice)
    - available in the core language
    - string literals
- both `String` and `str` are UTF-8 encoded
- `to_string()` and `String::from()` are equivalent
- strings do not support indexing

```rust
// creating empty str
let myStr = String::new();

// create string from string literal
let data = "hello";
let converted_data = data.to_string();

let data = "hello";
let converted_data = String::from(data);

// appending to string

// iterating over string
for c in "hello".chars() {
	println!("{}", c);
}
// will print: h e l l o
```

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
// format is another way to format a string
// format! macro returns a 'String' with its contents
```

**8.3 Storing Keys with Associated Values in Hash Maps**

- represented as `HashMap<K, V>`
- store data on heap (similar to vectors)
- all keys must be on the same type; all values must be of the same type
- `insert` moves values into the hash map; values are now owned by the hash map
- inserting references to values in a hash map won't move the values
- values that references point to must be valid for at least as long as the hash map is valid

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50)

// retrieval
let team_name = String::from("Blue");
let score = scores.get(&team_name);

// iterating over map
for (key, value) in &scores {
	println!("{}: {}", key, value);
}

// above loop will print
// Yellow: 50
// Blue: 10

// conditionally inserting
scores.entry(String::from("Red")).or_insert(25) // inserts

// will not insert since 'Blue' already exists in hashmap
scores.entry(String::from("Blue")).or_insert(20)
```

### Chapter 9 - Error Handling

- Rust requires acknowledgement of handling of a possible error before your code will compile
- 2 categories of errors:
  1.  recoverable
      - ex: file not found error
  2.  unrecoverable
      - symptoms of bugs
      - ex: trying to access a location beyond the end of an array
- Rust doesn't have exceptions
  - recoverable errors: `Result<T, E>`
  - unrecoverable errors: `panic!`

**9.1 Unrecoverable Errors with panic!**

- backtrace: list of functions that have been called to get to point of error

**9.2 Recoverable Errors with Result**

- `Result` enum has two variants: `Ok` and `Err`

```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

- propagating error: return error to the calling code
- don't need to use `return` if it's the last expression in the function
- error values that have the `?` operator called on them go through the `from` function, which is used to convert errors from one type into another
- when the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
	let mut s = String::new();

	File::open("hello.txt")?.read_to_string(&mut s)?;

	Ok(s);
}
```

**9.3 To panic! or Not To panic!**

- better to return `Result` when defining a function that might fail
- `panic!`: signals that program is in a state it can't handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values
-

### Chapter 10 - Generic Types, Traits, and Lifetimes

- generics: abstract stand-in for concrete types or other properties

**10.1 Generic Data Types**

- `T` = type

```rust
fn largest<T>(list: &[T]) -> {
	let mut largest = list[0];

	for &item in list {
		if item > largest {
			largest = item;
		}
	}

	largest
}

fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let char_list = vec!['y', 'm', 'a', 'q']

	largest(&number_list);
	largest(&char_list);
}

```

- using generics costs the same as using concrete types
  - monomorphization: converting generic code into specific code by filling in the concrete types that are used when compiled

**10.2 Traits: Defining Shared Behavior**

- trait: tells Rust compiler about functionality a particular type has and can share with other types; can define shared behavior in an abstract way
  - allow us to define a set of methods that are shared across different types

```rust
pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub struct Tweet {
	pub username; String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}
```

- traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior

**10.3 Validating References with Lifetimes**

- lifetime: scope for which things are valid
- every reference in Rust has a lifetime
- most lifetimes are inferred; only need to be annotated when the lifetimes of references could be related in a few different ways
- dangling references: referencing data other than the data it's intended to reference
- borrow checker: compares scopes to determine whether all borrows are valid
  - runs at compile time, and checks to make sure that all borrowed values or references are valid
- lifetime of a variable: refers to how long the variable lives for
- lifetime annotations
  - describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes
  - start with `'`
  - annotations go in the function signature, not in the function body
- lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. once they're connect, rust has enough information to allow memory-safe operations and disallow operations that would create dangling points or otherwise violate memory safety

```rust
&i32			// a reference
&'a i32			// a reference with an explicit lifetime
&'a mut i32		// a mutable reference with an explicit lifetime
```

```rust
fn main() {
	let string1 = String::from("longest string is long");

	{
		let string2 = String::from("xyz");
		let result = longest(string1.as_str(), string2.as_str());
		println!("The longest string is {}", result);
	}
}

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}

}
```

- lifetime ellision rules
  - patterns programmed into Rust's analysis of references
  - set of particular cases that the compiler will consider, and if your code fits these cases, you don't need to write the lifetimes explicitly
  - input lifetimes: lifetimes on function or method parameters
  - output lifetimes: lifetimes on return values
- compiler has 3 rules for figuring out what lifetimes references have when there aren't explicit annotations
  1.  each parameter that is a reference gets its own lifetime parameter
  2.  there is exactly one input lifetime parameter
  3.  if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the life time of `sef` is assigned to all output lifetime parameters
- lifetime of all string literals is `'static`
- `'static` lifetime: reference can live as long as the duration of the program
  - all string literals have a static lifetime because they are stored in program's binary

### Chapter 11 - Writing Automated Tests

- test in Rust: function annotated with `test` attribute
- attribute: metadata about pieces of Rust code
- test macros
  - `assert!`: when you want to ensure that some condition in a test evaluates to `true`

**11.1 How to Write Tests**

**11.2 Controlling How Tests Are Run**

- `cargo test` command: compiles code in test mode and runs the resulting test binary
  - default behavior: run all tests in parallel (using threads)
- `cargo test -- --test=threads=1`: set the number of tests threads to 1; tests won't interfere with each other if they share state
- `cargo test {name_of_test}`: run a single test

```rust
// to ignore a test

#[test]
#[ignore]
fn expensive_test() {
	// code that takes an hour to run
}
```

`cargo test -- --ignored`: will run the ignored tests

**11.3 Test Organization**

- 2 main categories of tests:
  1.  unit tests
      - small and more focused, testing one module in isolation at a time
      - test private interfaces
  2.  integration tests:
      - entirely external to your library
      - uses only public interface and potentially exercising multiple modules per test
- `#[cfg(test)]`: runs test only when when `cargo test` is invoked; tests do not run when `cargo build` is invoked
  - cfg stands for "configuration"
- able to test private functions
- child modules can use the items in their ancestor modules
- `user super::*;`: brings all of the module's parent's items into scope
- only library crates expose functions that other crates can use; binary create sare meant to be run on their own
- if a project is a binary crate that only contains a _src/main.rs_ file and doesn't have a _src/lib.rs_ file, we can't create integration tests in the _tests_ directory and bring functions defined in the _src/main.rs_ file into scope with a `use` statement

### Chapter 12 - An I/O Project: Building a C

**12.1 Accepting Command Line Arguments**

- iterators produce a series of values,
  - `collect`: turns iterator into a collection that contains all elements
    - needs to have annotation because Rust isn't able to infer collection type

**12.2 Reading a File**

**12.3 Refactoring to Improve Modularity and Error Handling**

- primitive obsession: anti pattern where using primitive values when a complex type would be more appropriate
- `unwrap_or_else`: allows to define some custom, non-`panic!` error handling.
- `()`: unit type; returns nothing
  - when used in a function, `()` indicates that the function it is in is only being used for side effects only
  - doesn't return a value we need
- `Box<dyn Error>`: (trait object) return a type that implements the `Error` trait, but doesn't specify the particular type the return value will be
  - dyn is "dynamic"
- _src/main.rs_: binary crate
- _src/lib.rs_: library crate

**12.4 Developing the Library's Functionality with Test Driven Development**

- data reference by a slice needs to be valid for the reference to be valid

**12.5 Working with Environment Variables**

- `to_lowercase` creates new data rather than referencing existing data
- `env` standard library module: use when working with environment variables
- use `is_err` over `unwrap`, `expect` when you don't care about the value

**12.6 Writing Error Messages to Standard Error Instead of Standard Output**

- two types of output:
  - _standard output_: general info
  - _standard error_: error info
- `println!`: only capable of printing to standard output
- `eprintln!`: for printing errors

### Chapter 13 - Functional Language Features: Iterators and Closures

**13.1 Closures: Anonymous Functions that Can Capture Their Environment**

- closure
  - anonymous function you can save in a variable or pass as arguments to other functions
  - can capture values from the scope in which they're defined
  - explicit type annotations on parameters and functions not required, because closures are usually short and relevant only within a narrow context rather tan in any arbitrary scenario
  - unlike function, can capture their environment and can access variables from the scope in which they're defined
- when a closure captures a value from it's environment, it uses memory to store the values for use in the closure body
-
- closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership , borrowing mutably, and borrowing immutably
  - associated traits:
    - `FnOnce`: taking ownership
    - `FnMut`: borrowing mutably
    - `Fn`: borrowing immutably
  - use `move` keyword to force ownership of values it uses in the environment
- `|`: vertical pipes

```rust
// closure example
let expensive_closure = |num| {
	println!("calculating slowly");
	thread::sleep(Duration::from_secs(2));
	num
};
```

- reason for using closure: defined the code to call at one point, store that code, and call it at a later point

**13.2 Processing a Series of Items with iterators**

- iterator
  - responsible for the logic of iterating over each item and determining when the sequence has finished
  - lazy: they have no effect until you call methods that consume the iterator to use it up

```rust
let v1 = vec![10, 20, 30];
for val in v1_iter {
	println!("val is {} ", val);
}
// will print
// val is 10
// val is 20
// val is 30
```

- all iterators implement the `Iterator` trait (in standard library)
- if using the `next` method from `Iterator`, the data structure you're calling it on must be mutable
  - `next` method changes the internal state that the iterator uses to keep track of where it is in the sequence
- methods that call `next` are called _consuming adaptors_, because calling them uses up the iterator
- _iterator adaptors_: allow you to change iterators into different kinds of iterators
- `collect`: consumes the iterator and collects the resulting values into a collection data type

```rust
let v1: Vec<i32> =vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

**13.3 Improving Our I/O Project**

**13.4 Comparing Performance: Loops vs Iterators**

- iterators get compiled down to roughly the same code as if you'd written the lower-level code yourself
- Iterators are one of Rust’s _zero-cost abstractions_, by which we mean using the abstraction imposes no additional runtime overhead.
- _Unrolling_: optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.

### Chapter 14 - More About Cargo and Crates.io

**14.1 Customizing Builds with Release Profiles**

- two main profiles:
  - `dev` for development
  - `release` for release builds

**14.2 Publishing a Crate to Crates.io**

- crate registry at crates.io distributes source code for packages
- documentation comments use 3 slashes `///`
- `cargo doc --open`: build the HTML for crate's documentation and documentation for crate's dependencies
- tests are also performed on documentation examples
- re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead

**14.3 Cargo Workspaces**

- workspaces: set of packages that share the same _Cargo.lock_ and output directory

**14.4 Installing Binaries from Crates.io with cargo install**

- `cargo install`: install and use binary crates locally
- only packages with binary targets can be installed
  - binary target: runnable program that is created if the crates has a _src/main.rs_ file or another file specified as binary

**14.5 Extending Cargo with Custom Commands**

### Chapter 15 - Smart Pointers

- pointer: variable that contains an address in memory
- Rust's version of a pointer is a reference
- smart pointers: pointer + additional metadata and capabilities
  - examples: `String`, `Vec<T>`
  - usually implemented with structs
  - implement `Drefer` and `Drop` traits
    - `Deref`: allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers
    - `Drop`: allows you to customize the code that is run when an instance of the smart pointer goes out of scope

**15.1 Using `Box<T>` to Point to Data on the Heap**

- Boxes allow for storage on the heap rather than the stack

```rust
fn main() {
	let b = Box::new(5);
	println!("b = {}", b);

	// value of 5 is allocated to the heap
}
```

- at compile type, Rust needs to know how much a space a type takes up
- pointer's size doesn't change based on the amount of data it's pointing to
- indirection: instead of storing a value directly, we'll change the data structure to store the value indirectly by storing a pointer to the value instead
- Boxes solely provide indirection and heap allocation

**15.2 Treating Smart Pointers Like Regular References with the Deref Trait**

- `*` dereference operator:
- reference: "arrow to a value stored somewhere else"

```rust
fn main() {
	let x = 5;
	let y = &x; // with Box, can also written as let y = Box::nex(x);

	assert_eq!(5, x);
	assert_eq!(5, *y);

	// we have to use * because comparing a number (5) to a reference to a number isn't
	// allowed because they're different type. Dereference operator (*) is used to
	// follow the reference to the value it's pointing to
}
```

- `Deref` trait has to be used to enable dereferencing
- in order to implement a trait, trait's required methods must be declared
- deref coercion: converts a type into a reference to another type
  - EX: it can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`
  - happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition
- mutable references have the ability to be coerced to immutable references
- immutable references can never coerce to mutable references

**15.3 Running Code on Cleanup with the Drop Trait **

- `Drop` trait: let's you customize what happens when a value is about to go out of scope
  - mostly used when implementing a smart pointer
- variables are dropped in the reverse order of their creations
- destructor: function that cleans up an instance (ex: `Drop` trait)
- constructor: function that creates an instance
- to force a value to be cleaned up early, you can use `std::mem::drop` function

**15.4 `Rc<T>`, the Reference Counted Smart Pointer**

- `Rc<T>`: reference counting
  - keeps track of the number of reference to a value to determine whether or not the value is still in use; if there are zero references to a value, the value can be cleaned up without any references becoming invalid
  - only for single-threaded scenarios
  - allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist

**15.5 `RefCell<T>` and the Interior Mutability Pattern**

- interior mutability: mutating the value inside an immutable value
- `RefCell<T>`: represents single ownership over the data it holds
  - borrowing running are enforced at runtime
  - only for use in single-threaded scenarios (gives a compile time error if used in a multi-threaded scenario)

**15.6 Reference Cycles Can Leak Memory**

- memory leak: memory that is never cleaned up

### Chapter 16 - Fearless Concurrency

- concurrent programming: different parts of a program execute independently
- parallel programing: different parts of a program execute at the same time

**16.1 Using Threads to Run Code Simultaneously**

- create new thread via `thread::spawn`

```rust
use std::thread;
use std::time::Duration;

fn main() {
	thread::spawn(|| {
		for i in 1..10 {
			println!("hi number {} from the spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});

	for i in 1..5 {
		println!("hi number {} from the main thread!", i);
		thread::sleep(Duration::from_millis(1));
	}
}
```

- blocking a thread: thread is prevented from performing work or exiting

**16.2 Using Message Passing to Transfer Data Between Threads**

- message passing: threads/actors communicate by sending each other messages containing data
- channel consists of a transmitter and receiver
  - transmitter: called with data you want to send
  - receiver: checks for arriving messages
- `mpsc`: multiple producer, single consumer

**16.3 Shared-State Concurrency**

- shared memory concurrency
  - multiple threads can access the same memory location at the same time
- mutex
  - mutual exclusion
  - allows only one thread to access some data at a given time
  -

**16.4 Extensible Concurrency with the Sync and Send Traits**

### Chapter 17 - Object Oriented Programming Features of Rust

**17.1 Characteristics of Object-Oriented Languages**

- object oriented programming (OOP)
  - made up of objects
  - object packages both data and the procedures (methods) that operate on that data
- `impl` blocks provide methods on structs and enums
- encapsulation: implementation details of an object aren't accessible to code using that object; only way to interact with that object is through the public interface
- Rust's OOP-like features:
  - encapsulation: option to use `pub` or not for different parts of code enables encapsulation of implementation details
- inheritance: object inherits from another object's definition, gaining the parent object's data and behavior without have to define them again
- no inheritance in Rust; no way to define a struct that inherits the parent struct's fields and method implementations
- instead of inheritance, Rust uses trait objects to enable polymorphism

**17.2 Using Trait Objects That Allow for Values of Different Types**
Trait objects - points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime - can't contain data - allows for abstraction across common behavior

```rust
// trait example

pub trait Draw {
	fn draw(&self);
}
```

**17.3 Implementing an Object-Oriented Design Pattern**

- unpopulated fields in structs are not allowed

### Chapter 18 - Patterns and Matching

**18.1 All the Places Patterns Can Be Used**

- `match` expressions have to be exhaustive: all possibilities for the value in the `match` expression must be accounted for
- downside of using `if let`: compiler doesn't check for exhaustiveness (unlike for `match`)

```rust
// for loop example
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
	println!("{} is at index {}", value, index);
}

```

**18.2 Refutability: Whether a Pattern Might Fail to Match**

- patterns come in 2 forms:
  - irrefutable: patterns that match any possible value
    - example: `let x = 5;`
  - refutable: patterns that can fail to match for some possible values
    - example: `if let Some(x) = a_value`
- match arms must use refutable patterns, except for the last arm, which should match any remaining values with an irrefutable pattern

**18.3 Pattern Syntax**

- match multiple patterns with `|`

```rust
let x = 1;

match x {
	1 | 2 => println!("one or two"),
	3 => println!("three"),
	_ => println!("anything"),
}

// prints one or two
```

- `..=`: inclusive range of values

```rust
let x = 5;

match x {
	1..=5 => println!("one through five"),
	_ => println!("something else"),
}

// prints one through five
```

- ignore values with `_`

```rust
fn foo(_: i32, y: i32) {
	println!("The code only uses the y parameter: {}", y);
}

fn main() {
	foo(3,4);
}
```

- `_` vs `_foo` (name starting with an underscore

```rust
let s = String::from("Hello!");

//(a)
if let Some(_s) = s {
	println!("found a string");
}

// (b)
if let Some(_) = s {
	println!("found a string")
}

println!("{:?}", s);

// using block (a) we'll get an error because the s value
// will still be moved into _s, which prevents us from using s again

// using block (b) will not cause any errors because we never bind s
// to anything; it isn't moved
```

- `..` pattern: ignores any parts of a value that we haven't explicitly matched in the rest of the pattern

```rust
fn main() {
	let numbers = (2, 4, 8, 16, 32);

	match numbers {
		(first, ..., last) => {
			println!("Some numbers: {} {}", first, last);
		}
	}
}
```

- `@`: test a value and save it in a variable within one pattern

```rust
enum Message {
	Hello { id: i32 }
}

let msg = Message::Hello {id: 5};

match msg {
	Message::Hello {
		id: id_variable @ 3..=7,
	} => println!("Found an id in range: {}", id_variable),
	Message::Hello {id: 10..=12} => {
		println!("Found an id in another range")
	}
	Message::Hello {id} => println!("Found some other id: {}", id),
}

// prints Found an id in range: 5
// able to save value of variable in id_variable
```

### Chapter 19 - Advanced Features

**19.1 Unsafe Rust**

- unsafe rust: version of rust that doesn't enforce memory safety guarantees
- 5 things you can do in unsafe rust
  1.  dereference a raw pointer
  2.  call an unsafe function or method
  3.  access or modify a mutable static variable
  4.  implement an unsafe trait
  5.  access fields of `union` S

**19.2 Advanced Traits**

**19.3 Advanced Types**

- `!` (never type): denotes function that will never return

```rust
fn bar() => ! {
	// code here
}
```

- diverging functions: functions that return never
- dynamically sized types: values whose size is only known at runtime
- slice data structure stores the starting position and the length of the slice

**19.4 Advanced Functions and Closures**

- able to pass regular functions to functions
- `fn`: function pointer

```rust
fn add_one(x: i32) -> i32 {
	x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
	f(arg) + f(arg)
}

fn main() {
	let answer = do_twice(add_one, 5);
	println!("The answer is: {}", answer);
}
```

- closures are represented by traits; you can't return closures directly

**19.5 Macros**

- metaprogramming: a way of writing code that writes other code
