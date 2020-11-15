# Rustic Ray

Ray Tracer based on a book The Ray Tracer Challenge by Jamis Buck.

I'm learning Rust using The Ray Tracer Challenge book. This repository is the end result of working through the book. There is another repository [ray-tracer-challenge-rust](https://github.com/y2k4life/ray-tracer-challenge-rust) progressing through each chapter. There is a branch in this repository that was my first attempt. I did not and still don't have a complete grasp on ownership, moving, borrowing, copying, and cloning. Then there is the lifetime fo `Thing`s. I made changes to my first attempt by eliminating `clone`ing as a solution.

In my second attempt I approached the project using the concepts of Rust to reason what I was doing. I started to speak Rust. Who owns this `Thing`, do I want a function to own this `Thing` or do I want the function to borrow the `thing` and not own it.

I made the primitives `Ray`, `Point`, `Vector`, `Color`, and `Matrix` have the `Copy` trait. Because the shapes `Sphere`, `Plane`, etc. implement a trait `Shape` and the `Shape` is Boxed into a `Vec` of the `World` giving them the `Copy` trait was not feasible. At first I gave them the `Clone` trait. I was then abusing cloning as a solution to problems. I was also abusing boxing everything which in a way was a copy/clone.

I started to think of ownership and borrowing. The `World` owns `Shapes` in a `Box` and lets other pieces `Intersect`, `Computations` borrow the `Shapes`. Instead of cloning the shape over and over again. I was cloning to prevent moving and transferring ownership.

## performance

There is a scene in `main` used to compare the cloning branch to the ownership and borrowing. I'm sure there is more to this than just cloning. I wonder the overhead on `Box`ing But what do I know I'm just starting to learn Rust. What I do know is the changes increased performance by 50%. I use the `time` function with Linux have yet to use benchmark in Rust.
