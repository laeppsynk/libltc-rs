# libltc-rs

A rust wrapper for libltc.

## Dependencies

Download and compile libltc's fork from [druskus20/libltc](https://github.com/druskus20/libltc).

If the library is not found, make sure to set the right `LD_LIBRARY_PATH`:

```bash
export LD_LIBRARY_PATH=/usr/local/lib 
```

## LICENSE

I believe the terms of the original license (LGPG) allow for this project to be
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
