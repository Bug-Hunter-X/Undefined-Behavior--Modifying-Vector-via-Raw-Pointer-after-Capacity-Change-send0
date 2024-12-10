# Rust Undefined Behavior Example: Raw Pointer Vector Modification

This repository demonstrates a common source of undefined behavior in Rust: attempting to modify a vector through a raw pointer after its capacity has been changed.  This can lead to unpredictable results, including crashes or silent data corruption.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file provides a safer alternative.

## How to Reproduce

1. Clone this repository.
2. Navigate to the directory.
3. Run `rustc bug.rs` and then `./bug`.
4. Observe the potentially unexpected output.  The program *may* panic, or it may silently corrupt memory.

## Solution

The `bugSolution.rs` file illustrates a better approach, leveraging Rust's ownership and borrowing system to avoid undefined behavior.