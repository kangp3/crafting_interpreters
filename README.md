# Crafting Interpreters

Peter's making it some nonzero portion of the way through [Crafting Interpreters](https://craftinginterpreters.com/)

# Current Error

```
   Compiling crafting_interpreters v0.1.0 (/home/kangp3/projects/crafting_interpreters)
error[E0506]: cannot assign to `curr` because it is borrowed
    --> src/main.rs:40:21
     |
40   |                     curr = curr.next.as_ref().unwrap().borrow();  // &DLLNode<T>
     |                     ^^^^   ---- `curr` is borrowed here
     |                     |
     |                     `curr` is assigned to here but it was already borrowed
     |                     borrow later used here
     |
     = note: borrow occurs due to deref coercion to `DLLNode<T>`
note: deref defined here
    --> /home/kangp3/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1436:5
     |
1436 |     type Target = T;
     |     ^^^^^^^^^^^

error[E0597]: `curr` does not live long enough
  --> src/main.rs:40:28
   |
38 |                 let curr = head.borrow();
   |                     ---- binding `curr` declared here
39 |                 for _ in 0..index {
40 |                     curr = curr.next.as_ref().unwrap().borrow();  // &DLLNode<T>
   |                            ^^^^ borrowed value does not live long enough
...
44 |             },
   |             -
   |             |
   |             `curr` dropped here while still borrowed
   |             borrow might be used here, when `curr` is dropped and runs the destructor for type `Ref<'_, DLLNode<T>>`

Some errors have detailed explanations: E0506, E0597.
For more information about an error, try `rustc --explain E0506`.
error: could not compile `crafting_interpreters` (bin "crafting_interpreters") due to 2 previous errors
```
