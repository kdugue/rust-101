# Rust In Motion

## Rust Notes

- general purpose language
- places where rust performs well:
  - high performance web services
  - web assembly
  - command-line applications
  - embedded devices
- benefits of rust: speed, safety, fearless concurrency
- rustup: command line tool to manage the installation of Rust versions and optional components
- cargo: package manager and build tool
- binary crate: produces an executable
  - vs library crates: used by other projects

## Unit 1 : Introduction to Rust Syntax

- variable: label and store data into
- declaring a variable
  - all declarations start with `let` keyword

```rust
let x = 5;
```

```rust
fn main() {
	let x = 5;
	let y = 6;
	let z = x + y;
	println("z is {}", z);
}

// main is fn called when 'cargo run' is called
```

- by default, variables can't be changed; it is immutable by default
- use the `mut` keyword to make a variable mutable

```rust
fn main() {
	let mut x = 5;
	x += 1;
	println!("x is now {}", x);
}
```

### Variable Types

- every variable has a type;
  - you can annotate the type of variable: `let mut x: i32 = 5;`
  - most idiomatic Rust doesn't annotate type when Rust can't figure out the type you want
- simple data types
  - Boolean: `bool`
    - `true` or `false`
  - Integer
    - numbers without decimal points
    - signed and unsigned
    - different amounts of space

| signed | unsigned |
| ------ | -------- |
| i8     | u8       |
| i16    | u16      |
| i32    | u32      |
| i64    | u64      |

`` - if a integer type is not declared, it is`i32` by default

- Floating Point
- Character

  - `char`
  - use single quotes

- Compound
  - Tuple
    - group multiple values into one type
    - values don't have to be of the same type

```rust
let tup = (1, 'c', true);
```

- Array
  - fixed size
  - if you add more elements to array, you have to use a `Vec`
- Slices
  - reference to a contiguous subset of data in another data structure

### Functions

Defining function

```rust
fn name(param1: type1, ...) -> return_type {
	...body...
}

name(arg1, ...);

// only use return type is function is returning a type
```

### Control Flow

If / else if / else
while

match

- match expressions are similar to: if/else if/else, switch, and case
- better because of: pattern matching, exhaustiveness checking

### Enums

- defining custom types

```rust
enum HockeyPosition {
	Center,
	Wing,
	Defense,
	Goalie
}
```

### Structs

- when to use enums vs structs
  - enums: choice between a set of values
  - structs: same attributes for each value of a type

```rust
enum HockeyPosition {
	Center,
	Wing,
	Defense,
	Goalie
}

struct HockeyPlayer {
	name: String,
	number: u8,
	position: HockeyPosition,
	goals_ytd: u8
}
```

### Methods

- defining behavior on custom types
- methods are defined within a block with the `impl` (implementation) keyword
- first parameter of method is always `self`, which refers to the type we are defining the method on
- `&self`: methods that don't change anything about `self`'s values (i.e., when reading fields from hockey's position)

### Associated Functions

- defined within an `impl` block
- don't take `self` as a parameter
- commonly used to create instances

```rust
impl HockeyPlayer {
	fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
		HockeyPlayer {
			name: name,
			number: number,
			position: position,
			goals_ytd: 0
		}
	}
}

```

## Unit 2: Ownership and Borrowing

- ownership and borrowing: how rust deals with memory management

### Module 2: What is ownership?

- ownership:
  - rust's strategy for managing data in memory and preventing common problems
  - each piece of data has one owning variable
  - owner is responsible for cleaning up that data
  - cleanup happens when the owner goes out of scope
  - the owner decides on mutability
- memory management model: somewhere in between manual memory management in C and garbage collection in Ruby
  - rust is the best of both worlds: ownership gives control over memory allocation and the associated performance, but by cleaning up data automatically when the owner goes out of scope we can't mess up the memory access and we wont be using memory longer than we strictly need to
  - `String` is a type that has data that needs to be cleaned up when it goes out of scope
  - by default, with non-primitive types, Rust moves ownership

```rust
let s = String::from("hello");


// ownership from s moves to t
// s is no longer able to be used after moving
let t = s;
```

- cloning: deep copy of the allocated memory; there will two copies of the data that each have an owner, each owner is responsible for cleaning their own data
- you can't move a value while it's borrowed
- benefits of ownership
  - control over performance
  - memory safety helps to avoid errors
- `clone()` is not idiomatic rust

### Module 3: Ownership Exercise Solutions

### Module 4: Borrowing

- borrowing: a way to allow some code to use a value with moving ownership; lend out a value instead of transferring ownership
- borrowing is indicated by `&` symbol
- borrowing is done for performance reasons: if a function doesn't need ownership of a value that has allocated memory, instead of cloning the value and giving that to the function, we can give the function a reference to the original value
  - borrowing helps to reduce allocations and improve performance
- borrowing is also a signal of intent

```rust
pub fn push_str(&mut slef, string: &str)

/*
this string argument means that it wants nothing to do
with allocation or deallocation of that data
*/
```

- you can't return a reference from a function that points to something that was created within that function and will go out of scope and will get cleaned up at the end of the function
- unlike other languages, references in rust are always valid

### Module 5: Slices

- slice: data type that always borrows data owned by some other data structure (i.e. `String`)
  - consists of a pointer and a length
    - pointer: reference to the start of data that the slice contains
    - length: number of elements after the start the slice contains
  - can be created string `String`, array, and vectors
  - always valid references
  - have their indices checked at runtime
  - string allows to borrow a chunk of data
- slice example:

```rust
let s = String::from("goodbye");
let s_slice = &s[2..4];
// starting index is 2
// ending index is 4
```

- vectors: similar to array, but can grow and shrink in size (i.e., they have the `.push()` method)
- character boundaries are NOT checked at compile time
- better to use slice or string slice as parameters of functions/methods (rather than borrowing from a vector or array)
  - allows for functions/methods to be used in more context
  - functions can accept borrowed strings or string literals
- string literals create string slices
- How are we able to call functions that a string slice that is a reference to an owned string?
  - Rust has an implementation in its standard library called deref (de-reference) trait (on string) --> this allows Rust to be able to convert a reference to a string into a string slice containing the whole string
  - "deref coercion": when calling function/method, compiler will automatically dereference the arguments (if need be) to convert them to match the function parameter type

```rust
fn main() {
	let s = String::from("book");
	let plural = pluralize(&s);

	println!(
		"I have one {}, you have two {}",
		s,
		plural,
	);
}

fn pluralize(singular: &str) -> String {
	singular.to_owned()  + "s";
}

// this is why we are able to call the pluralize function
// even though pluralize takes in a string slice as its parameter type
// and we have a reference to an owned string (plural variable is reference)
// to an owned string
```

- more about deref traits: https://doc.rust-lang.org/std/ops/trait.Deref.html

### Module 6: Borrowing and Mutability

- to make a mutable reference: `&mut x`
- mutable references are commonly used as first parameters of methods, so that the methods can modify the current instance
- borrowing rules
  - you may have EITHER (for a value):
    - many immutable references
    - one mutable reference

### Module 7: Borrowing Code Patterns

- borrowing happens at the same time = in the same (lexical) scope
- adding new scope tells rust where borrows end
- non-lexical lifetimes (NLL)
  - borrows that aren't used through the end of the scope
  - borrows only used for part of an expression
  - borrows that aren't used in an arm(?)
- non-lexical lifetime caveats
  - Entry API will still be preferred
  - won't change the need to split structs
  - code that actually violates the borrowing rules will be still be invalid
  - existing code might not be updated

### Module 8: Ownership of More Than Just Memory

- sockets: system resource that's a connection to a network endpoint for sending and receiving data
  - example of socket: TCP

## Unit 3: Error Handling

### Module 2: Panicking when something goes wrong

- panic concept:
  - it stops the program in invalid states
  - like an emergency stop button;
- `panic!` macro
  - causes program to stop, with optional message
  - `panic!("Something's not right here!")`
- when to panic
  - continuing would be incorrect
    - important to prevent security vulnerabilities!
  - no way for calling code to recover
  - when problem must be fixed in code
- other panicking macros
  - `unreachable!` - impossible to get to this spot
  - `unimplemented!` - code isn't written yet
  - assert family (`assert!`, `assert_eq!`, `asset_ne!`) - panic if a condition isn't true
  -

### Module 3: Handling Results and Options

- `Result`
  - has two variants: success and failure
- `Option`
  - has concept of two variants: (1) having something, or (2) having nothing

### Module 4: Writing a function that returns `Result` and using `?`

### Module 5: Advantages of Rusts's error handling strategy

- advantages
  - possibility of errors is clear
  - compiler enforces handling
  - find and fix bugs sooner, fewer exploits from hackers/malicious actors
  -
- disadvantages
  - development flow will be a lot more slow
  - multiple error types

### Module 6: Custom error types

- `Box<Error>` or `Box<dyn Error>` (same thing)
  - Trait object, which consists of a
    - Pointer (Box)
    - Trait (`std::error::Error`)
  - any type the implements the `Error` trait
  - `Error` requires `Debug` and `Display` for printing
  - downsides of using `Box<Error>`
    - can't inspect the error type in code
    - can't decide to handle different errors differently
- use `Box<Error>` trait when you need to know that an error happened + print
- using a custom error type
  - should be used when needing to distinguish errors in code

### Module 7: Error handling crates

### Module 8: Useful methods on `Result` and `Option`

## Unit 4: Lifetimes

- lifetime: length of time a reference to some value may exist
- rust compiler ensures all references have valid lifetimes

### Module 2: An Exploration of Concrete Lifetimes

- value's lifetime (in rust):
  - a value's lifetime is the time a value is at one memory address
  - starts: created or moved into a location in memory
  - end: moved or dropped from that location
- reference's lifetime similar to value's lifetime, with the exception of one additional constraint: it must be contained within the referenced value's lifetime
  - this ensures that every reference will always point to a valid value

### Module 3: Visualizing Lifetimes to Understand Borrow Checker Errors

- examples of invalid references:
  - returning a reference to a value created within an inner scope
  - returning a reference to a value created within a function
  - referencing a moved value
  - self-referential struct
  - storing references in a `HashMap`
- when a reference's lifetime is longer than the value it is referring to, that is invalid and will cause an compile error
- ownership is moved when a value is returned from a function
- moving a value copies values on the stack

### Module 4: An Exploration of Generic Lifetimes

- generic lifetime:
  - life time of a reference in code where we can't know all of the possible concrete lifetimes of the values being referenced at compile time
  - can exist in functions, methods, structs, enums, traits

### Module 5: A Short Introduction to Generics

- generic type parameters
  - abstraction that allow us to write code once that can used many times with different types

```rust
// implement generic for approving different types (articles, users, comments)

pub struct Approval<T> {
	item: T,
	approved: bool,
}

impl<T> Approval<T> {
	pub fn new(item: T) -> Approval<T> {
		Approval {
			item,
			approved: false,
		}
	}
}
```

### Module 6: Lifetime Parameters are a Kind of Generic

- purpose of generic lifetime parameters is to tell the compilers how the lifetimes of references are related; used to express relationships
- generic lifetime parameter denoted by `'a`

```rust
fn simulate_game<'a>(home: &'a str, away: &'a str) -> &'a str {
	if rand::random() {
		home
	} else {
		away
	}
}
```

- why lifetime parameters are necessary
  - express your intent in the signature
  - keeps compiler analysis local
  - compiler can't tell what lifetimes should be in complex cases
- generic types vs generic lifetimes
  - both declared in angle brackets (ex: `<'a, T>`)
  - both used in structs, enums, functions, methods, traits, etc
  - differences
    - generic type parameters are **generic over types** (they allow you to write code once that works with many different types), while generic lifetime parameters are **generic over scopes** (allows you to write code once that works with many different references that are valid for different lengths of time)
    - naming conventions
      - generic type parameters: `UpperCamelCase`
      - generic lifetime parameters: `snake_case`
    - when using generic type parameters, they generate code for each usage, while with generic lifetime parameters, they are used during analysis only

### Module 7: Lifetime Parameters are Descriptive, not Prescriptive

- lifetime parameters are descriptive
  - describe intended relationships between references
  - concrete lifetimes that fill in the parameters are determined by the code calling the definition containing lifetime parameters
    - lifetime parameters don't have any control over, or knowledge of, what those concrete lifetimes will be; the only that we can say is that when there are concrete lifetimes, they will have this relationship
- lifetime parameters are not prescriptive
  - don't order code to behave a certain way
  - don't change concrete lifetimes

### Module 8: Lifetime Elision

- lifetime elision: set of rules programmed into the compiler that were added so that you don't have to add generic lifetime parameters on every reference
  - if rules aren't enough to figure out lifetime parameters, compiler will error
- you can annotate lifetimes if you want even in cases where lifetime elision would let you leave them out (note: this is NOT idiomatic rust)
- 3 rules of lifetime elision
  1.  in parameters, each reference gets its own lifetime
  2.  if there is one lifetime in the parameters, returned reference gets that lifetime
  3.  (only applies to methods) if there is a `&self` or `&mut` `self` parameter, returned reference gets that lifetime
