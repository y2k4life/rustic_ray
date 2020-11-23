# Rustic Ray

Ray Tracer based on a book The Ray Tracer Challenge by Jamis Buck.

I'm learning Rust using The Ray Tracer Challenge book. This repository is the end result of working through the book. There is another repository [ray-tracer-challenge-rust](https://github.com/y2k4life/ray-tracer-challenge-rust) progressing through each chapter. There is a branch in this repository that was my first attempt. I did not and still don't have a complete grasp on ownership, moving, borrowing, copying, and cloning. Then there is the lifetime of `Thing`s. I made changes to my first attempt by eliminating `clone`ing as a solution to my problems.

In my second attempt I approached the project using the concepts of Rust to reason what I am doing. I started to speak Rust. Who owns this `Thing`, do I want a function to own this `Thing` or do I want the function to borrow the `thing` and not own it.

I made the primitives `Ray`, `Point`, `Vector`, `Color`, and `Matrix` have the `Copy` trait. Because the shapes `Sphere`, `Plane`, etc. implement a trait `Shape` and the `Shape` is Boxed into a `Vec` of the `World` giving them the `Copy` trait was not feasible. At first I gave them the `Clone` trait. I was then abusing cloning as a solution to problems. I was also abusing boxing everything which in a way was a copy/clone.

I started to think of ownership and borrowing. The `World` owns `Shapes` in a `Box` and lets other pieces `Intersect`, `Computations` borrow the `Shapes`. Instead of cloning the shape over and over again. I was cloning to prevent moving and transferring ownership.

## performance

I'm sure there is more to this than just cloning. I wonder the overhead on `Box`ing But what do I know I'm just starting to learn Rust. What I do know is the changes from cloning to borrowed references increased performance by almost 50%.

## Deviations from the book

This implementation was done with the end results in mind and working backwards. There are are no tuples, at the end of Chapter 1 there is `Point` and `Vector`. Instead of tuples this implementation use `struct`s that represent a `Point` and a `Vector`. Chapter 2 introduces `Color` and is implemented as a tuple, in this project there is a `struct` to represent `Color` and no tuples.  Chapter 3 works through a 2x2 and a 3x3 matrix to get to a 4x4 matrix. There is only one `struct` for storing a matrix, `Matrix`. The `Matrix` is an array of arrays creating a 4x4 matrix. There are 2x2 matrices that use the 4x4 `Matrix` filling in only the first two rows and columns. There are no `struct`s for 3x3 or 2x2 matrices or matrices of any other shape than 4x4. This was done for simplicity and performance. This implementation also uses Rust array instead of `Vec` and this was done for performance. I'm assumed working with a fixed size array should be quicker than working with a `Vec` that has extra overhead to manage the ability to change it's size.

## Profiling

Learning Rust using this book was very helpful and the next step after was profiling the Rust application to learn more and make things faster. Ray Tracing is time consuming and finding ways to produce a scene in seconds rather than minutes was valuable. I used [Valgrind](https://www.valgrind.org/) profiling tools and [kcachegrind](http://kcachegrind.sourceforge.net/html/Home.html) to analyze Callgrind output. This was valuable finding expensive code that was called multiple times and in particular inverting a matrix and building a matrix for transformations. Another tool I used to determine if code written one way would compile differently than code written a different way was [Compiler Explorer](https://rust.godbolt.org/) (vector vs. array). Set it up with two sources and compare the compiled code.

## Tooling

Used the following tools:

* [VSCode](https://code.visualstudio.com/)
* [Rust Anayzer](https://rust-analyzer.github.io/) | [VSCode extention](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) | Debugger
* Windows | Ubuntu Desktop (Profiling) | WSL2
* [Valgrind](https://www.valgrind.org/)
* [kcachegrind](http://kcachegrind.sourceforge.net/html/Home.html)
* [Compiler Explorer](https://rust.godbolt.org/)

## Chapter 14 - Groups

This was a tough chapter, circular references are frowned upon in Rust. A child shape of a group should not have a reference to the group if the group has a references to the children of the group. `<pun>` I went in circles working on this one `</pun>`. My solution is to have a container (ShapeContainer) as the root of all the shapes. The container is represented as a tree of all the shapes. Give a shape in a group a parent id of the group. When the parent shape/group is needed traverse the shape container looking for the shape with an id that matches the parent id. Instead of working up from the shape to find the parent using a borrowed reference of the parent, work from the top of the tree down using a parent shape id of the shape. Intercepting a ray worked from the group down that was easy. The Normal At uses the parent of a given shape. Therefore the ShapeContainer is passed into normal_at.

```
             SC
            / | \
           S  S  G
               / | \
              S  S  S
```

## Feedback

I'm open to feedback. Submit an issue or PR. I'm learning how to write idiomatic Rust. I'm still learning where to use borrowed references `&` when to use `Copy` or `Clone` or any of the other borrow checking and life times. Learning how to write code without fixing broken code with `.clone()` everywhere. Learning when I should use `for` loop or when to use an `Iterator`. What are all these `Box`es let alone a `Box`ed `dyn`. Code will work but does that mean it is written the Rust way. My biggest challenge with Rust is not knowing when to use what and where. I'm more reactive to the borrow checker the proactive.
