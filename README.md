# Rust Exercises

=================================================================

This repository provides a hands-on collection of Rust exercises and practical coding challenges designed to help developers master the language's core concepts—including ownership, borrowing, lifetime annotations, and smart pointers. Built with a test-driven approach, each module allows you to implement solutions, run automated unit tests, and iteratively refine your understanding of Rust's unique memory safety guarantees and type system. Whether you are transitioning from another language or looking to solidify your systems programming skills, these practice problems offer a clear, step-by-step path from fundamental syntax to advanced idiomatic patterns.

=================================================================

# Box<T> Exercises

1. **Singly Linked List Node**
* **Goal:** Create an enum named `Node` representing a singly linked list of integers with two variants: `Elem(i32, Box<Node>)` and `Nil`. Write an associated function `length(&self) -> usize` that recursively calculates the size of the list.
* **Concept practiced:** Resolving recursive types with unknown size at compile time using `Box`.

2. **Node-Based Stack (Linked Stack)**
* **Goal:** Implement a `Stack<T>` struct using a recursive node wrapped in `Option<Box<Node<T>>>`. Implement the methods `push(&mut self, elem: T)` and `pop(&mut self) -> Option<T>`.
* **Concept practiced:** Moving and replacing pointers on the heap using `Option::take`.

3. **Node-Based Queue (Linked Queue)**
* **Goal:** Create a queue struct (`Queue<T>`) that stores elements linked via `Box`. Implement insertion at the back and removal from the front.
* **Concept practiced:** Managing ownership and heap pointers when modifying the ends of a linear data structure.

4. **Binary Search Tree (BST)**
* **Goal:** Implement a `TreeNode<T: Ord>` struct containing a value `T` and two child nodes: `left: Option<Box<TreeNode<T>>>` and `right: Option<Box<TreeNode<T>>>`. Create an `insert(&mut self, value: T)` method to add new nodes in the correct position.
* **Concept practiced:** Navigating and mutating nested pointers on the heap.

5. **Arithmetic Expression Tree**
* **Goal:** Create an `Expr` enum with variants `Literal(i64)`, `Add(Box<Expr>, Box<Expr>)`, and `Multiply(Box<Expr>, Box<Expr>)`. Implement an `eval(&self) -> i64` method to recursively evaluate the tree's result.
* **Concept practiced:** Syntax trees and recursive evaluation of heap-allocated data.

6. **Heterogeneous Node Collection (Trait Objects)**
* **Goal:** Create a `DataNode` trait with a `to_string_repr(&self) -> String` method. Implement the trait for `i32` and `String`. Then, create a heterogeneous `List` that stores elements of type `Box<dyn DataNode>`.
* **Concept practiced:** Dynamic polymorphism in data structures using `Box<dyn Trait>`.

7. **Directed Acyclic Graph (DAG) via Pointer Vector**
* **Goal:** Implement a `GraphNode<T>` struct containing a value `T` and a collection of neighbors: `neighbors: Vec<Box<GraphNode<T>>>`. Write a Depth-First Search (`DFS`) function to check whether a path exists between two nodes.
* **Concept practiced:** Complex non-linear structures with multiple single-ownership pointers.

8. **Simplified B-Tree (N-ary Tree)**
* **Goal:** Create a generic tree node `NaryNode<T>` that holds a vector of children: `children: Vec<Box<NaryNode<T>>>`. Implement a Breadth-First Search (`BFS`) traversal method using an auxiliary queue.
* **Concept practiced:** Combining dynamic collections (`Vec`) with individual heap allocations (`Box`).

9. **Dynamic Sparse Matrix using Boxed Slices**
* **Goal:** Create a `SparseMatrix<T>` struct that accepts a `Vec<T>` of arbitrary size, shrinks its capacity to fit, and converts it into a `Box<[T]>` (a contiguous heap-allocated slice without stack capacity overhead). Implement matrix transposition without reallocating new vectors on the stack.
* **Concept practiced:** Optimizing stack memory footprint using `Box<[T]>` instead of `Vec<T>`.

10. **Hash Table with External Chaining (Separate Chaining)**
* **Goal:** Create a fixed-size `HashTable<K, V>` struct with an array of buckets. Each bucket must contain a custom linked list using `Box<Node<K, V>>`. Implement `insert`, `get`, and `remove` operations while handling collisions via manual pointer manipulation.
* **Concept practiced:** Building advanced composite data structures by combining arrays and heap pointers.



