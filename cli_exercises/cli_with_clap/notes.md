# How to Use clap crate

## Builder Method with command!()

### Prerequities

1. clap create should be added with cargo feature
2. positional arguments are the arguments which they don't need short or long specifier before they are used.
3. named arguments are the arguments which they need short or long specifier before they are used.

## Methods for clap

1. arg()
   1. It is used to create an argument.
2. subcommand()
   1. It is used to make different command for different arguments and group them together
   2. If the global argument is defined which is not included in any subcommand should be called before the command when it is used. (fluffy in the code is global argument)
3. about()
   1. Used to add help text to explain what the application does to show up along with help menu
4. get_matches()
   1. It invokes the parser.
5. short() and long() methods
   1. they are used to convert positional arguments into named arguments by specifiying long string or short character for the argument.
6. alias() or aliases() methods
   1. They are used to be alternative for the named arguments.
   2. They dont't show up along with the help menu
   3. They help different user for different usage
   4. If a named argument is --lname, a user can use it --lastname or --last-name If they are defined as aliases
7. required()
   1. If an argument has to be used we can specify it by using required method with boolean true.
8. help()
   1. It is used to add description for the argument to show up along with help menu
9. conflicts_with()
   1. We can add one of other arguments as a conflicted argument which can not be used together anymore
