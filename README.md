# curld

A simple command-line tool for uploading local directories to HTTP servers.

## Installation

### From crates.io

```bash
cargo install curld
```

### From source

```bash
git clone https://github.com/w2moon/curld.git
cd curld
cargo install --path .
```

## Usage

```bash
curld -d <local-directory> <target-url>
```

Example:

```bash
curld -d ./my-files http://xxx.xxx.xxx.xxx:xxxx/new-path/path-to-file
```

## Features

- Recursively traverse all files in the specified directory
- Maintain directory structure when uploading files
- Display upload progress and results
- Support asynchronous operations for better performance
