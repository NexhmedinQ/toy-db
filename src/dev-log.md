## Initial Thoughts
The purpose of this project is to gain a deeper understanding about how databases work internally. My initial focus
before starting is to create a concurrent relational database. I am interested in the algorithmic side of
things so the B-tree implementation will be my own rather than from a Rust library. My initial
goal is to implement an on-disk only database which I'll hope to extend by adding an in-memory caching layer
in between my frontend and on-disk data structure. The implementation details will be heavily influenced
by ```Database Internals - Alex Petrov``` and 
```SQLite Database System Implementation - Sibsankar Haldar```.

## REPL
Eventually we will need to create a simple client that connects to the database server and sends queries but for
now we will get started with a simple REPL implementation to make testing easy