# cuboid

Create a cuboid with specified dimensions in csgrs

## Usage

```
$ cuboid -H
   Compiling panel v0.1.0 (/home/wink/data/3D-Graphics-CAD-CAM/Prusa/models/panel)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.95s
     Running `target/debug/panel -H`
Usage: panel [OPTIONS]

Options:
  -H, --help             Prints help information
  -w, --width <WIDTH>    width of the panel [default: 10.0]
  -l, --length <LENGTH>  length of the panel [default: 100.0]
  -h, --height <HEIGHT>  height of the panel [default: 5.0]
  -V, --version          Print version
```

## Build, run, install

```
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/cuboid`
$ cargo install --path .
  Installing cuboid v0.1.0 (<snipped>)
    Updating crates.io index
     Locking 229 packages to latest Rust 1.88.0 compatible versions
      Adding i_float v1.6.0 (available: v1.15.0)
      Adding i_overlay v1.9.4 (available: v1.10.0)
      Adding i_shape v1.6.0 (available: v1.14.0)
    Finished `release` profile [optimized] target(s) in 0.86s
   Replacing /home/wink/.cargo/bin/cuboid
   Replaced package `cuboid v0.1.0 (<snipped>)` with `cuboid v0.1.0 (<snipped>)` (executable `cuboid`)
```

## Results

The default output file is `cuboid_w-10.00_l-100.00_h-5.00.stl`

This can be visualized as a 3D model using `f3d cuboid_w-10.00_l-100.00_h-5.00.stl`
and converted to an image using `f3d cuboid_w-10.00_l-100.00_h-5.00.stl --output cuboid_w-10.00_l-100.00_h-5.00.png`

#![cuboid](cuboid_w-10.00_l-100.00_h-5.00.png)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
