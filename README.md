# Better Shell

Improved the basic anyrun shell plugin to display the output of the entered command

# Build

To build the plugin, execute the following commands:


```sh
$ cargo build --release
```

You need to copy the compiled library to the plugin directory (if it doesn't exist, create one)
```sh 
$ cp target/release/libbetter_shell.so ~/.config/anyrun/plugins/
```

Include the library in in your `~/.config/anyrun/config.ron`

```diff
Config(
    plugins: [
-       "libshell.so",
+       "libbetter_shell.so",
        ...
    ]
)
```