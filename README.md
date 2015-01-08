# cairo-rs
This library provides [Cairo](http://cairographics.org) bindings for the [Rust](http://rust-lang.org/) programming language.

The library attempts to provide exactly the same API as the C library, however some changes were necessary due to the presence of C unions and the like which do not have convenient Rust counterparts.

Currently in an experimental state, cairo-rs is most likely not anywhere near bug-free.

## Dependencies
You'll need to have the (http://cairographics.org/download/)[cairo development libraries] to link with.

On Windows, you'll need to rename `libcairo.dll.a` to `libcairo.a` and place it in in `C:\Rust\bin\rustlib\i686-pc-mingw32\lib` or `C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib`.

## Usage
To install using the Git repository, add the following to your Cargo.toml:

```ini
[dependencies.cairo-sys]
git = "https://github.com/cyderize/cairo-rs.git"
```

And add the line

```rust
extern crate "cairo-sys" as cairo_sys;
```

to your crate root.

## Example
An example can be run with
```
cargo run --example main
```

## Licence
### The MIT License (MIT)

Copyright (c) 2014 Cyderize

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
