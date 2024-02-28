# How to Use clap crate

## Builder Method with command!()

### Prerequities

1. clap create should be added with cargo feature
2. positional arguments are the arguments which they don't need short or long specifier before they are used.
3. named arguments are the arguments which they need short or long specifier before they are used.

## Methods for clap

1. arg()
   1. It is used to create an argument.
2. get_matches()
   1. It invokes the parser.
3. short() and long() methods
   1. they are used to convert positional arguments into named arguments by specifiying long string or short character for the argument.
