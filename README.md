# Rupit

A simple CLI tool to run long, complex, and multistep terminal commands by aliases.

using a configuration file `rupit.json`, that contains the list of aliases for rupit to use.

## How to use

For Linux users you can use the pre-built binary from the releases [page](https://github.com/mohamedeliwa/rupit/releases) , it's built and tested on **Ubuntu 22.04.4 LTS**,

or you can build from source for linux, windows, and macOS systems using the following steps:

1. Pull the repo: `git pull git@github.com:mohamedeliwa/rupit.git`
2. Build the project with: `cargo build --release`
3. Put the release binary in a suitable execution path
4. Create `rupit.json` file, with a structure similar to the following:

```json
{
  "aliases": {
    "greetings": "echo \"Hello World\"",
    "multistep": "cd ~; touch test.txt"
  }
}
```

> Note: depending on your OS the path at which to put the file will be

> **Linux**: /home/\<user>/.config/rupit/rupit.json

> **Windows**: C:\Users\\\<user>\AppData\Roaming\Foo Corp\Bar App\rupit.json

> **macOS**: /Users/\<user>/Library/Application Support/com.Foo-Corp.Bar-App/rupit.json

5. Run the command with an alias name `rupit <ALIAS>`

```sh
$ rupit greetings
```

this will execute the following:

```sh
$ echo "Hello World"

Hello World
```
