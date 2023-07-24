# Rust Implementation of Lox (WIP)

This implements the bytecode compiler and VM for the Lox programming language from [craftinginterpreters.com](craftinginterpreters.com) in Rust.

Differences from the book:
1. The chunks use [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding) to compress and store line number information for bytes.
2. The instruction set implements `OP_CONSTANT_LONG` to store more than 256 constants using 3 big-endian bytes &mdash; allowing a maximum of 16777216 constants.
3. Although I have implemented the on-demand lexer, the final interpreter uses the lexer from my [tree-walking](https://www.github.com/nimaipatel/rlox-treewalking) implementation since the `Iterator` trait makes `lazy_scanner` unergonomic to use.
4. The parser is also directly imported from my tree-walking implementation.