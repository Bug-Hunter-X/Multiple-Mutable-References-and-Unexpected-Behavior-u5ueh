# Multiple Mutable References in Rust

This example demonstrates a subtle issue that can arise when working with multiple mutable references in Rust. Although the code executes without a compiler error, it showcases potential pitfalls when modifying data through multiple mutable borrows.  Understanding this behavior is crucial for writing safe and predictable Rust code.