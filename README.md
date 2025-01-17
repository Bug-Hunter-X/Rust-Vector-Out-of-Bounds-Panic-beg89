# Rust Vector Out of Bounds Panic

This repository demonstrates a common error in Rust: panicking when trying to access an element of a vector using an index that is out of bounds.  The `bug.rs` file shows the erroneous code, and `bugSolution.rs` provides a corrected version.

The error typically happens when you assume the vector has elements when it may be empty.  Always check the vector's length before accessing elements to prevent this panic.
