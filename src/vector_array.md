```rs
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4];
     println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]);
```
![](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/vec-capacity-4.svg)
```rs 
 v.push(5);
    //prints 8
    println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]);
}
```
![](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/vec-capacity-8.svg)
```rs
let v: Vec<i32> = vec![1, 2, 3, 4];
let s = &v[1..3];
```
![](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/slice-of-vec.svg)

Since slices borrow from the underlying data structure, all the usual borrowing rules apply. For example, this code is rejected by the compiler:
```rs
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    let s = &v[..];
    v.push(5);
    println!("First element in slice: {:}", s[0]);
}
```
Why? Because when the slice is created, it points to the first element of the vector's backing buffer and as a new element is pushed onto the vector, it allocates a new buffer and the old buffer is deallocated.
