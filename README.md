# Rupit

A simple CLI tool to run long, complex, and multistep terminal commands by aliases.

using a configuration file `rupit.json`, that contains the list of aliases for rupit to use.

## How to use

1. create `rupit.json` file, with a structure similar to the following:
   > Note: the path at which to put the file is not yet determined

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
