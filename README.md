# secli

A CLI to store secrets locally.

## Usage

```sh-session
$ cargo install secli

$ secli add secret
  ? Enter the name/key for this secret: supersecret
  ? Enter the value for this secret: supersecretvalue

$ secli get supersecret
  supersecretvalue
```

## Commands

- [`secli add [NAME]`](#secli-add-name)
- [`secli get [NAME]`](#secli-get-name)
- [`secli list`](#secli-list)
- [`secli delete [NAME]`](#secli-delete-name)
- [`secli update [NAME]`](#secli-update-name)
- [`secli help [COMMAND]`](#secli-help-command)

### `secli add [NAME]`

Add a secret

```
USAGE
  $ secli add [NAME]

ARGS
    <name>  Name of the secret

DESCRIPTION
  Add a secret
```

### `secli get [NAME]`

Get a secret

```
USAGE
  $ secli get [NAME]

ARGS
    <name>  Name of the secret

DESCRIPTION
  Get a secret
```

### `secli list`

List all secrets

```
USAGE
  $ secli list

DESCRIPTION
  List all secrets
```

### `secli delete [NAME]`

Delete a secret

```
USAGE
  $ secli delete [NAME]

ARGS
    <name>  Name of the secret

DESCRIPTION
  Delete a secret
```

### `secli update [NAME]`

Update a secret

```
USAGE
  $ secli update [NAME]

ARGS
    <name>  Name of the secret

DESCRIPTION
  Update a secret
```

### `secli help [COMMAND]`

Print this message or the help of the given subcommand(s)

```
USAGE
  $ secli help [COMMAND]

ARGS
    <command>  Command to get help for

DESCRIPTION
  Print this message or the help of the given subcommand(s)
```
