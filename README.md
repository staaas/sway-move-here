# `sway-move-here`

`sway-move-here` is a helper tool to move all the workspaces to the
current output in [Sway][0] window manager.

In a nutshell it does the following:

1. Retrieve all the outputs (displays) and workspaces.
2. Move all the workspaces to the output that is currently focused.
3. Switch back to the workspace that was originally focused.

[0]: https://swaywm.org/

## Usage

Just run `sway-move-here` binary after building it.

One can also bind it to a key combo in Sway config (see `man 5 sway`), e.g.:

```
bindsym --to-code $mod+Control+apostrophe exec --no-startup-id /path/to/sway-move-here
```

If the tool doesn't work as expected, one can increase output verbosity:

* flag `-v` will display `swaymsg` commands and their exit codes
* flag `-vv` will additionally print standard output of those commands

## Building

The tool is written in [Rust lang][1] 2018 edition, uses [Cargo][2]
as a build tool and [rustup][3] as a toolchain manager.

The tool is statically compiled using [musl][4] by default, so one'll
need to install the appropriate target first:

```sh
rustup component add x86_64-unknown-linux-musl
```

Once that is done once, the tool can be built using [Cargo][2]:

```sh
cargo build --release
```

Binary `target/x86_64-unknown-linux-musl/release/sway-move-here` can
now be copied to any location.

[1]: https://www.rust-lang.org/
[2]: https://doc.rust-lang.org/cargo/
[3]: https://rustup.rs/
[4]: https://www.musl-libc.org/

## Testing

The tool doesn't have automated tests (yet), but it works. I swear!
