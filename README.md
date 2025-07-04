# xmlstringify

Lite tool for converting xml to string version. Change `<` to `&lt;` and `>` to `&gt;`.

## Installation

You can install from source using `cargo install --path .` in project root directory.

## Usage examples

- `xmlstringify foo.xml -o bar.xml` - stringify xml from file `foo.xml` and
  save output to file `bar.xml`;
- `xmlstringify foo.xml` - stringify xml from file `foo.xml` and
  write output to stdout.
