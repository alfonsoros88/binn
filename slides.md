---
theme: ./dark.json
---

# Building a Rust interface for a C library 🦀

## Part0: Fundamentals

- Foreign Function Interfaces in rust
- Some trivial example

## Part1: Building the bindings crate

- Using `bindgen` and `build.rs`
- Use the `cc` crate to build and link the library

## Part2: Building the rust interface

- Resource management
- Code generation with macros and generics
- Lifetimes and ownership
- Tons of Undefined Behavour

---

# Part0: Fundamentals

Symbols and Calling Conventions

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

- A function's calling convention is part of its type
- The calling convention is specified next to `extern`
- Main calling conventions are `"C"` and `"rust"`
- `"C"` is used by default if `extern` is specified

```rust
extern {
    fn puts(s: *const c_char);
}

extern "C" {
    fn puts(s: *const c_char);
}
```

- Every rust function is implicitly declared with `extern "rust"`

```rust
fn foo() {...}

extern "rust" fn foo() {...}
```

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

### Hands-on example

---

# Creating a Rust interface for a C library

Section Goal:

- The chosen library for today's demo: `binn`
- Organazing Bindings + Rust interface 🦀

## Part1: Create bindings for `binn`

- The `build.rs` hack
- `bindgen` to generate bindings automatically
- `cc` crate for compiling `binn`

## Part2: Safe Rust interface around the bindings

- Memory management
- Code generation
- Lifetime and ownership
- A lot of unsafe code

---

## Few words about binn

`binn` is a very simple C library for serialization.

Uses data containers like lists, maps and objects.

A small example:

```c
  // create a new object
  obj = binn_object();

  // add values to it
  binn_object_set_int32(obj, "id", 123);
  binn_object_set_str(obj, "name", "Samsung Galaxy");
  binn_object_set_double(obj, "price", 299.90);

  // pass the buffer to another function
  // send over the network or save to a file...
  another_function(binn_ptr(obj), binn_size(obj));

  // release the object
  binn_free(obj);
```

---

## Organizing Bindings + Rust Interface 🦀

The bindings are ship in a `*-sys` crate (e.g. `openssl-sys`) and the Rust
high-level interface is provided in separate crate (e.g. `openssl`)

The `*-sys` crates have two main functionalities:

- Link to the native library
- Provide just the bindings and declarations

## Why is this?

- Only one crate can link to a native library
- Handle changes in the C library or in `bindgen`
- The interface changes can be adopted incrementally

---

# Part1: binn-sys crate

#### build.rs

- A script executed before building the crate
- In particular useful for building and linking dependencies

## Enabled in the `Cargo.toml`

```toml
[package]
...
build = "some-file.rs"
```

## How it works:

```rust
fn main() {
    println!("cargo:rerun-if-changed=foo.json");
    println!("cargo:rerun-if-env-changed=BAR");
    println!("cargo:rustc-cfg=KEY={}", "value");
    println!("cargo:rustc-link-lib=foo");
    println!("cargo:rustc-link-search=./lib/dir/");
}
```

---

#### bindgen

- Generates automatically bindings for C headers.
- It comes with a stand-alone binary
- As a library, it can be called from a **build.rs** script.

```rust
fn main() {
    println!("cargo:rustc-link-lib=foo");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");
}
```

---

#### cc

- Builds and links native code
- Automatically deals with cross compilation and environment
- Can handle C, C++ and assembly

```rust
fn main() {
    println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```

---

## Hands-on demo to build binn-sys

---

# Part2: binn crate

Goal:
- Create a safe interface for a `binn_object`
- Enable serialization for basic types
- Deserialize `binn_object`s

---

## Directly to hands-on

