# What is this?

dialler-rs is a simple terminal application that allows you to manage your contacts and use your pre-installed softphone to make a call.

It's simple to use, shortcut keys are displayed at the bottom, and ESC to exit.

# How do i install it?

You can install (compile) with `cargo`:

```
cargo install dialler-rs
```

Then to launch it:
```
dialler-rs
```

## Configuration
For configuring dialler-rs, you can use the following environment variables:
```
DIALLER_PROGRAM="path/to/your/softphone"
```