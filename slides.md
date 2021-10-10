---
theme: ./dark.json
---

# Building a Rust interface for binn

## Part0: Fundamentals

- Foreign Function Interfaces in rust
- Some trivial example

## Part1: Building the bindings crate

- Using `bindgen` and `build.rs`
- Use the `cc` crate to build and link `binn`

## Part2: Building the rust interface for `binn`

- Use scoping to manage resource allocations
- Macros for generate repetitive code
- Undefined Behavour all over the place

---

## Symbols

- Use `extern` to annotate symbols residing within a foreign interface

```rust
extern {
    static FOREIGN_SYMBOL: bool;

    fn foreign_function_foo(x: i32);
}
```

- Use `#[no_mangle]` for symbols exported by your rust library

```rust
#[no_mangle]
pub static RUST_SYMBOL: bool = true;

#[no_mangle]
pub extern fn rust_function_foo(x: i32) {...}
```

---

## Calling Conventions

The **calling convention** defines:

- How the stack frame for the function is prepared
- How arguments are passed
- Where to write the function result
- Where to jump back when the function returns
- How to cleanup stuff like register after the function completes

---

## Calling Conventions

- The calling convention is specified next to `extern`
- Main calling conventions are `"C"` and `"rust"`
- The `"C"` is used by default if `extern` is specified

```rust
extern {
    fn puts(s: *const c_char);
}

extern "C" {
    fn puts(s: *const c_char);
}
```

- A function's calling convention is part of its type
- Every rust function is implicitly declared with `extern "rust"`

---

## Types across FFI

#### Basic types

Rust's standard lib comes with C types in `std::os::raw`

```rust
type c_int = i32
type c_char = i8/u8
type c_long = i32/i64
```

#### Arrays

Arrays are handled with pointers and sizes. `Vec`'s API defines:

```rust
pub fn into_raw_parts(self) -> (*mut T, usize, usize)

pub unsafe fn from_raw_parts(
    ptr: *mut T,
    length: usize,
    capacity: usize
) -> Vec<T, Global>
```

---

## Types across FFI

#### Strings

- `std::ffi::CStr` and `std::ffi::CString` are similar to `str` and `String`.
- Strings have to be converted to a sequence of bytes terminated with 0.

```rust
extern "C" { fn puts(s: *const c_char); }

let to_print = CString::new("Hello!").expect("CString::new failed");

unsafe {
    puts(to_print.as_ptr());
}
```
---

## Types across FFI

#### Enums without data

```rust
#[repr(C)]
enum Foo { Bar, Baz }
```

is represented with a single integer. It is possible to specify the
discriminator value and type:

```rust
#[repr(C, u8)]
enum Foo {
    Bar = 1,
    Baz = 2
}
```

---

## Types across FFI

#### Enums with Data

```rust
#[repr(C)]
enum Foo {
    Bar(i32),
    Baz { a: bool, b: f64 }
}
```

is represented with a discriminant integer and a union of data.

```rust
#[repr(C)]
enum FooTag { Bar, Baz }
#[repr(C)]
struct FooBar(i32);
#[repr(C)]
struct FooBaz{ a: bool, b: f64 }
#[repr(C)]
union FooData {
  bar: FooBar,
  baz: FooBaz,
}
#[repr(C)]
struct Foo {
    tag: FooTag,
    data: FooData
}
```

---

## Types across FFI

#### Function pointers

Nothing really special just be careful with panics and unwinds

```rust
extern "C" {
    fn callback(f: extern "C" fn() -> c_int) -> c_int;
}

extern "C" fn foo() -> c_int {
    42
}

assert_eq!(unsafe { callback(foo) }, 42);
```

---

## bindgen and build.rs

#### build.rs

- A script executed before building the crate
- In particular, it is useful for building the and linking dependencies

#### bindgen

- Generates automatically bindings for C headers.
- It comes with a stand-alone binary
- As a library, it can be called from a **build.rs** script.

---

# Part1: Creating binn-sys

Goal:
    - Create `binn-sys` that provides just the bindings for `binn`
    - Use `bindgen` to generate the bindings from a `build.rs`
    - Use `cc` to build and link `binn`

