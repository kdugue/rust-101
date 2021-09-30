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
- Rust gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

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

### Chapter 5 - Using Structs to Structure Related Data

- struct: custom data types for grouping together multiple related values

  5.1 Defining and Instantiating Structs

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

  5.2 An Example Program Using Structs

- `#[derive(Debug)]` allows to print out debugging information for struct

  5.3 Method Syntax

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

  6.1 Defining an Enum

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

  6.2 The `match` Control Flow Operator

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

6.3 Concise Control Flow with `if let`

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
  - contains a `Cargo.toml` file that describes how to build those crates
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
