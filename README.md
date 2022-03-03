# `kvstore`

# A command-line key-value database tool.

Inspired by the Microsoft Reactor Rust programming talk __[Rust Programming: Moving Beyond “Hello World”](https://www.youtube.com/watch?v=5dRT_v3hrZ0)__ by Ryan Levick.

```
kvstore is a key-value keeper.

Usage:
    kvstore [<key>] [<value>]

Args:
    <key>       label to identify data
    <value>     data to store behind a label

Discussion:
    kvstore's database is a 'kv.db' file located where the program is ran
    unless the enviornment variable KVSTORE_HOME is set to an existing 
    directory.
```

## Editing

Adding/updating a key will require you to pass the new value for it as `value`. Values will be overridden if one already exists for the given `key`.

## Viewing

Viewing a key's value only requires the `key` argument to be passed. A key that does not exist will return a blank line.

## Ideas/Extensions

- [ ] `--preview` show the before/after state of key before asking user if it's okay to save when editing. 

- [ ] `--init` could set the key-value pair as an env variable in the current working terminal session. Having no key & value arg will default to initialize all key-values as environment variables. 

- [ ] `--home=<dir>` to override a particular location of `kv.db` file for the given program call. Has precedence over `KVSTORE_HOME`.

- [ ] `--all` to view all key-value pairs.

- [x] allow user to define where to place `kv.db` file using an env variable `KVSTORE_HOME`.

- [ ] allow special syntax to prepend/append to an existing key-value pair.

    `$ kvstore rpath @:/usr/local/bin`
    
    Here the `@` symbol copies the existing value for key `rpath` to be restored with `:/usr/local/bin` appended. A current workaround is to use shell like: 

    `$ kvstore rpath $(kvstore rpath):/usr/local/bin`
