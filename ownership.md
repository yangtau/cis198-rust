# Ownership

### move

没有实现 Copy trait 的类型，在赋值或者传参的时候都会 move。所有权被 move 之后，原来的变量不能再次被使用。

```rust
let a: i32 = 4;
let b = a; // copy, a is not moved

let v1 = vec![1, 3, 4];
let v2 = v1; // ownership of the Vec moves to v2
println("{}", v1[1]); // error: value borrowed here after move
```

如果只有 *move* 这一种操作，可以保证一个变量要么拥有对一个对象的 ownership 要么就是不可用的，而一个对象在同一时刻只会被一个变量所拥有。那么当一个拥有对某个对象的 ownership 的变量 goes out of scoop 的时候，这个对象的内存就可以被释放。

### borrow

只有 *move*，可以简化内存的管理，但是会给编程带来很多麻烦。为了简化操作，Rust 增加了 *borrow* 的操作。一个变量的数据可以通过引用的方式被借用。借用时，ownership 不会改变，当引用 goes out of scoop 时，借用结束。

由于引用是对数据的借用，所以其能力是有局限的，它不能转移所有权。这个道理很 intuitive：我借用的东西，我不拥有其所有权，所以更不能把所有权转移给别人。

```rust
let v1 = vec![1, 2, 3];
let v2 = &v1;
let v3: Vec<i32> = *v2; 
// error: cannot move out of `*v2` which is behind a shared reference
```

一个对象可以被有多个 immutable 引用，或者只有一个 mutable 引用。

```rust
let mut v1 = vec![1, 2, 3];
let v2 = &v1;
let v3 = &v1;
println!("{:?}{:?}", v2, v3);

let v4 = &mut v1; // mutable borrow occurs here
let v5 = &v1; // error: cannot borrow `v1` as immutable because it is also borrowed as mutable
v4.push(5);
println!("{:?}", v5);
```

同时，除了对引用的限制外，变量本身也受到了限制。在一个变量被引用的时候，它的 ownership 不能转移。

```rust
let v1 = vec![1, 2, 3];
let v2 = &v1;
let v3 = v1; // error: cannot move out of `v1` because it is borrowed
println!("{:?}{:?}", v2, v3);
```

## dereference

**auto-dereference:**

- When making method calls on a reference.
- When passing a reference as a function argument.

```rust
/// `length` only needs `vector` temporarily, so it is borrowed.
fn length(vec_ref: &&Vec<i32>) -> usize {
    // vec_ref is auto-dereferenced when you call methods on it.
    vec_ref.len()
}
fn main() {
    let vector = vec![];
    length(&&&&&&&&&&&&vector);
}
```

**explicit dereference:**

- When writing into them.
- And other times that usage may be ambiguous.

```rust
let mut a = 5;
let ref_a = &mut a;
*ref_a = 4;
println!("{}", *ref_a + 4);
// ==> 8
```



