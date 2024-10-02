# RUST 

This repo is for learning rust 

Integer datatypes:- 

unsigned :- u8,16,32,64
signed :- i8,16,32,64


Mutuable vs immutable variables 

Variables in rust are immutable by default cant be changed

so "mut" is used to make them mutable


```rust
fn main(){
  println!("Hello World!");
}
```


```rust
 println!("The number stored in num is {}", num);

// this is for format printing like %d in C
 ```

String literal of two types &str and String

## Memory Managment

1. Memory Allocation
we use pointer to point to a memory address in C and initialize memory after the memory location is done we free(the address) so now there is a garbage value and the pointer is pointing to it
An example

```C
#include <stdio.h>
#include <stdlib.h>

int main(){
    int *ptr = (int *)malloc(sizeof(int));
    if(ptr == NULL){
        printf("Memory allocation failed.\n");
        return 1;
    }

    printf("Value stored in allocated memory: %d\n", *ptr);
    // Assign the memory
    *ptr = 10;

    printf("Value stored in allocated memory: %d\n", *ptr);
    // freed the pointer
    free(ptr);
    // the memory is freed but the pointer is still in use and can be accessed
    printf("Trying to access freed memory: %d\n", *ptr);

    return 0;
    
}
```

## Stack vs Heap

Stack :- Works with fixed size data, size of mem allocated is known to compiler, it allocates and deallocates automatically and its fast

Heap :- Works with dynamic size data, memory allocated dynamically at runtime, does allocation and deallocation automatically but slow 


## Ownership 

1. Each value in rust has a variable called its owner
2. there can be only one owner at a time
3. when owner goes out of scope the value is dropped.


```Rust
let x: u8 = 2;
let y: u8 = x;

// Memory is copied from x to y


let s1: String = String::from("Helloo");
let s2: String = s1;

// Memory is not copied due to heap allocation of String datatype 
// s1 is the owner of hello
// when we do s2 = s1 there is transfer of ownership
// and the new owner is s2
```


Another example of ownership transfer

```Rust
fn main(){
  let x: String = String::from("Hello");
  process_str(x);   // As this function is called the ownership transfers to item in the parameter and the next print statement giver error
  println!("The value of x in main is {}", x);
}

fn process_str(item: String){
  println!("The value in the function process_str is {}", item); // hello - new owner is item
}
```


We can pass the refrence of a variable which is called borrowing

```Rust 
fn main(){
  let str1: String = String::from("Hello");
  let len: usize = calculate_len(&str1);

  // The & is the refrence of str1 which is borrowing it for calculate_len function call
  println!("The size of {} is {}", str1, len)
}

fn calculate_len(item: String)-> usize{
  return item.len();
}
```


when we pass the refrence & we can only read no write to the variable as long as we pass mut variable

```Rust

let mut s1: String = String::from("Hello ");

append_str(&mut s1);

fn append_str(s1:&mut String){
  s1.push_str("World");
}

```

this will let us borrow the memory and then also modify it