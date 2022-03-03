# `kvstore`

# A command-line key-value database tool.

Inspired by the Microsoft Reactor Rust programming talk __[Rust Programming: Moving Beyond “Hello World”](https://www.youtube.com/watch?v=5dRT_v3hrZ0)__ by Ryan Levick.

```
USAGE:  
    kvstore <key> [value]

ARGS:
    <key>       label for data

OPTIONS:
    value       data to attach to <key>
```

## Editing

Adding/updating a key will require you to pass the new value for it as `value`. Values will be overridden if one already exists for the given `key`.

## Viewing

Viewing a key's value only requires the `key` argument to be passed. A key that does not exist will return a blank line.

## Ideas/Extensions

- customizable flags. `--preview` show the before/after state of key before asking user if it's okay to save when editing. `--set` could set the key-value pair as an env variable in the current working terminal session.

- allow user to define where to place `kv.db` file using an env variable `KVSTORE_PATH`.


- allow special syntax to prepend/append to an existing key-value pair. Example: `kvstore rpath @:/usr/local/bin`. Here the `@` symbol copies the existing value for key `rpath` to be restored with `:/usr/local/bin` appended
