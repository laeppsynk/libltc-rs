# libltc-rs

A thin rust wrapper for [libltc](https://github.com/x42/libltc) by x42.

I tried my best to keep the API as close to the original as possible while
encapsulating the borrow / move semantics of the original library.

## Dependencies

The current supported version of libltc is [1.3.2](https://github.com/x42/libltc/releases/tag/v1.3.2).

Support for higher than 8bit encoding is being considered, if it happens, it
will be in my fork: [druskus20/libltc](https://github.com/druskus20/libltc).

If the library is not found, make sure to set the right `LD_LIBRARY_PATH`:

```bash
export LD_LIBRARY_PATH=/usr/local/lib 
```

## Running the examples

```bash
cargo run --example simple
cargo run --example encode -- test.txt  # creates test.raw
cargo run --example decode -- test.txt  # reads test.raw
```

When opening these raw files with an audio software like Audacity, it will most
likely not work as intended. A script like
[raw_to_wav.py](./examples/raw_to_wav.py) can be used to convert the raw file
to a wav file.

## Tips on debugging memory leaks

Make sure to be clear on where raw pointers get deallocated. Either by the
library (ltc_encoder_free, which also frees it's internal buffer), or by the
borrow checker (i.e. LTCFrame::drop).

Make sure that all the references reflect the ownership semantics of the
underlying library. The C codebase does not use `const *` so everything is
technically a `*mut`. Refer to the original library's documentation and code to
figure out the actual ownership.

```bash
valgrind ./target/debug/examples/simple
```

## LICENSE

I believe the terms of the original license (LGPL) allow for this project to be
licensed under the MIT license, since it is a wrapper around the original
library. The original library is required to be installed separately, and
is dynamically linked by [build.rs](./build.rs).

From the [original project](https://x42.github.io/libltc/index.html):

> **Can I use libltc in a proprietary/closed-source project?**
>
> Yes, with some care. In a nutshell: Create a dynamic library (.dll, ,dylib,
> .so) of libltc and link your program against it. Your project remains
> independent. You only need to be able to convey means to re-create this
> library (source-code, build-scripts) to anyone who asks. The easiest way to
> do this is to simply not modify libltc and refer to the upstream source (keep
> a copy just in case). If you copy the library code directly into your project
> and statically link your application against it, your project will have to be
> licensed in terms of the LGPL or a compatible license. See the license text
> for details and consult with a person with expertise in licensing.

The examples are loosely based on the original and thus excluded from the cargo
package and licensed under LGPL.
