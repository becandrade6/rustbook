_Ownership_ is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.

## The Stack and the Heap

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as _last in, first out (LIFO)_. Think of a stack of plates: When you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called _pushing onto the stack_, and removing data is called _popping off the stack_. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a _pointer_, which is the address of that location. This process is called _allocating on the heap_ and is sometimes abbreviated as just _allocating_ (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can usually do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often. But knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

## Ownership Rules

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be droped.

## The String Type

Let's use the String type to illustrate a data that is stored on the heap.

It is able to store an amount of text that is unknown to us at compile time.

```Rust
let s = String::from("hello");

s.push_str(",world!");  // push_str() appends a literal to a String

println!("{s}");  // this will print `hello, world!`
```

### Memory and Allocation

To allocate an amount of memory on the heap, unknown at compile time:
- the memory must be requested from the memory allocator at runtime.
- we need a way of returning this memory to the allocator when we're done with our `String`

When we call `String::from`, its implementation requests the memory it needs. Universal in most programming languages.

In languages with a garbage collector, the GC keeps track of and cleans up memory that is not being used anymore, and we don't need to think about it. In most languages without a GC, it is our responsability to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. If we forget, we'll waste memory. If we do it too early, we'll have a invalid variable. If we do it twice, that's a bug too. We need to pair exactly one `allocate` with exactly one `free`.

Rust takes a different path: The memory is automatically returned once the variable that owns it goes out of scope. Here's a version of our scope example using a `String` instead of a string literal:

```Rust
{
	let s = String::from("hello"); // s is valid from this point forward
	
	// do stuff with s
}   // the scope is now over, and s is no longer valid
```

There is a natural point at which we can return the memory our String needs to the allocator: when `s` goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it's where the author of String can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

This pattern has a profound impact on the way Rust code is written. It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we've allocated on the heap.

## Variables and Data Interacting with Move

*Double free error* is one of the memory safety bugs. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnearabilities.

```Rust
// the String has 3 'fields'
// ptr (points to the heap where the string literal is)
// len (how much memory, in bytes, the contents of the String are currently using)
// capacity (the total amount of memory, in bytes, the String has received from the allocator)
let s1 = String::from("hello");

// when we do this, we create another stack, copying the 3 fields of s1 to s2
// but now, we would have two variables with the field 'ptr' pointing to the same heap space
// so, when both variables go out of scope, we would have the double free error, we would try to free memory twice by freeing the ptr
let s2 = s1;

// but, to prevent this, Rust considers s1 as no longer valid after new assignment. So, Rust doesnt need to free anything when s1 goes out of scope

// if we try the line below, it wont work
// println!("{s1}, world!");
```

We call the act of Rust to consider s1 as no longer valid, a ***move***. We would say s1 was moved into s2.

Rust will never automatically create "deep" copies of your data, Therefore, any *automatic* copying can be assumed to be inexpensive in terms of runtime performance.

## Scope and Assignment

The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the drop function as well. When you assign a completely new value to an existing variable, Rust will call `drop` and free the original value's memory immediately.

```Rust
// creates new data in heap and binds to s in stack
let mut s = String::from("hello");

// the original string immediately goes out of scope, Rust will run the drop function on it
// and its memory will be freed right away.
// now, a new data is created in heap and binds to s in stack
s = String::from("ahoy");

// this prints 'ahoy, world'
println!("{s}, world");
```

## Variables and Data Interacting with Clone

If we *do* want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`. 

```Rust
let s1 = String::from("hello");

// clones the first data on heap to another heap space and binds to s2 on stack
let s2 = s1.clone();

println!("s1 = {s1}), s2 = {s2}");
```

## Stack-Only Data: Copy

This code using integers works and is valid:

```Rust
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```

Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there's no reason we would want to prevent `x` from being valid after we create the variable `y`.

There is no difference between deep and shallow copying here, so calling clone wouldn't do anything different from the usual shallow copying.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are. If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add `Copy` annotation to that type, we'll get a compile-time error.

So, what types implement the `Copy` trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`. Here are some of the types that implement `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

## Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. 

```Rust
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here
    
    let x = 5;                      // x comes into scope
    
    makes_copy(x);                  // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
    
} 
	// Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { 
	// some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { 
	// some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a compile-time error. These static checks protect us from mistakes. 

## Return values and Scope

Returning values can also transfer ownership.

```Rust
fn main() {
	// gives_ownership moves its return value into s1
    let s1 = gives_ownership();
    
    // s2 comes into scope
    let s2 = String::from("hello");
	
	// s2 is moved into takes_and_gives_back, which also moves
	// its return value into s3
    let s3 = takes_and_gives_back(s2);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       
	// gives_ownership will move its return value into the function
    // that calls it
    
    // some_string comes into scope
    let some_string = String::from("yours");
    
    // some_string is returned and moves out to the calling function
    some_string
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    
    // a_string is returned and moves out to the calling function
    a_string
}
```

While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

Luckily for us, Rust has a feature for using a value without transferring ownership: ***references***.
