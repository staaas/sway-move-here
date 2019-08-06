# `sway-move-here`

`sway-move-here` is a helper tool to move all the workspaces to the
current output in [Sway][swaywm] window manager.

In a nutshell it does the following:

1. Retrieve all the outputs (displays) and workspaces.
2. Move all the workspaces to the output that is currently focused.
3. Switch back to the workspace that was originally focused.

The project is developed on [sourcehut][sourcehut], there is also a
[github][github] mirror.

[swaywm]: https://swaywm.org/
[sourcehut]: https://git.sr.ht/~staaas/sway-move-here
[github]: https://github.com/nott/sway-move-here

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

The tool is written in [Rust lang][rustlang] 2018 edition, uses [Cargo][cargo]
as a build tool and [rustup][rustup] as a toolchain manager.

The tool is statically compiled using [musl][musl] by default, so one'll
need to install the appropriate target first:

```sh
rustup component add x86_64-unknown-linux-musl
```

Once that is done once, the tool can be built using [Cargo][cargo]:

```sh
cargo build --release
```

Binary `target/x86_64-unknown-linux-musl/release/sway-move-here` can
now be copied to any location.

[rustlang]: https://www.rust-lang.org/
[cargo]: https://doc.rust-lang.org/cargo/
[rustup]: https://rustup.rs/
[musl]: https://www.musl-libc.org/

## Testing

The tool doesn't have automated tests (yet), but it works. I swear!
