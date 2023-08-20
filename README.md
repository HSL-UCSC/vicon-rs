Low-level hardware abstraction layer for
reading data from a Vicon motion capture system.

## Using this Crate

The documentation for this crate can be
generated by running `cargo doc`; the 
generated docs will be written to `release/`.

## Compiling on a Linux Host

Before building this crate, or a Crate 
_depending_ on this crate, you need to:

1. Install Rust.
2. Install Clang.

Once these tools are installed, you can
build this crate by running:

1. `cargo build --release`

To _run_ this crate (or crates depending on it),
you will need to ensure all the `.so` files
in [`vendor/libvicon`](vendor/libvicon/) are
available on your `LD_LIBRARY_PATH`.

## License and Contributions

> ***Special Thanks*** to the Worcester Polytechnic Instute
> [Novel Engineering of Swarm Technology](https://nestlab.net/) (**NEST**) lab
> for providing a reference usage of the Vicon APIs, and 
> providing us with access to their Vicon systems for testing this crate.

Copyright 2023 Alicorn Systems, Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License. Refer
to [the license file](LICENSE.txt) for more information.

The files within `vendor/` retain their original copyrights
and licenses, and are included in this repository to simplify
the build process for downstream users.