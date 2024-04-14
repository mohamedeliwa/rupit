# Rupit

A simple CLI tool to run long, complex, and multistep terminal commands by aliases.

using a configuration file `rupit.json`, that contains the list of aliases for rupit to use.

## How to use

after pulling the repo, and building the project with `cargo build --release`,<br />
put the release binary in a suitable execution path then do the following:

1. create `rupit.json` file, with a structure similar to the following:

   > Note: the path at which to put the file depends on yours OS as follows

   > **Linux**: /home/<user>/.config/rupit/rupit.json

   > **Windows**: C:\Users\<user>\AppData\Roaming\Foo Corp\Bar App\rupit.json

   > **macOS**: /Users/<user>/Library/Application Support/com.Foo-Corp.Bar-App/rupit.json

the `rupit.json` file must have at least an empty `{}` for json deserialzation to work correctly.

an example of `rupit.json` file

```json
{
  "aliases": {
    "greetings": "echo \"Hello World\"",
    "multistep": "cd ~; touch test.txt"
  }
}
```

2. run the command with an alias name `rupit <ALIAS>`

```sh
$ rupit greetings
```

this will execute the following:

```sh
$ echo "Hello World"

Hello World
```
