# Storli
<div align="center">
<img src="https://user-images.githubusercontent.com/63192115/174482213-e726fe0e-0524-4775-9804-819b8bacb91b.png" alt="logo" height="128" width="128" />
  <br />
  
  <p align="center">
  <img src="https://img.shields.io/crates/v/secli"> <img src="https://img.shields.io/crates/d/secli" /> <img src="https://img.shields.io/github/license/AnishDe12020/secli?color=blue"> <img src="https://img.shields.io/tokei/lines/github/AnishDe12020/secli?color=pink&label=lines%20of%20code"> <img src="https://img.shields.io/github/languages/top/AnishDe12020/secli?color=%230xfffff">
</p>

**A CLI to store secrets locally**
  
  
  </div>
 <br />

## Why secli?

If you use a plugin that displays suggestions for commands (like [`zsh-autosuggestions`](https://github.com/zsh-users/zsh-autosuggestions)), it poses a security risk as the secret can be exposed as a part of the suggestion. There are however more use cases, like when you are recording a tutorial, you can use `secli` instead of exposing a secret.

## Is it secure?

All secrets are locally stored in your data directory in a sqlite database. The app is completely offline and no data is sent across the internet. It is also open-source so you can have a look at the source code :)

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
