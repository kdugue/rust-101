## Rust

- type system enforces ownership and borrowing:

  1. all resources have a clear owner
  2. others can borrow from the owner
  3. owner cannot free or mutate the resource while it is borrowed

- when passing data between threads, Rust will to make sure the data is thread-safe to use
