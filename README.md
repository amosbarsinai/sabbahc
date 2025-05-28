### From Amos Bar Sinai:
# `sabbahc` - the CLI for the Sabbah Language Compiler

```sabbahc - cli for the Sabbah language

Usage:
  sabbahc <INPUT> [OPTIONS]

Options:
  -o, --output <file>     Specify output filename
  -s, --asm               Compile only; do not assemble or link
  -b, --object            Compile and assemble; do not link
  -m, --mode <mode>       Manually specify the output mode
  -v, --version           Show version information and exit
  -h, --help              Show this help message and exit
  -f, --force             Ignore nonfatal errors

Description:
  sabbahc is a command-line compiler for the Sabbah programming language. It
  supports compiling to assembly, object code, and fully linked executables.

Examples:
  sabbahc main.sabbah         # Compile and link (default output: <filename>.out)
  sabbahc main.sabbah -o prog # Compile and link, output to 'prog'
  sabbahc main.sabbah -s      # Compile to Assembly only