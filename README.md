# Lithp

A simple toy language I made.  
Everything in Lithp is a function. Here are the current builtin functions:  
```
=(name, value)
==(arg1, arg2, arg3, ...)
<(arg1, arg2, arg3, ...)
>(arg1, arg2, arg3, ...)
*(arg1, arg2, arg3, ...)
/(arg1, arg2, arg3, ...)
+(arg1, arg2, arg3, ...)
-(arg1, arg2, arg3, ...)
ifElse(boolean, execIfTrue, execIfFalse)
print(value)
```
See the `examples` directory for examples.  
To run a Lithp program, pass in the path of the file as an argument, e.g. if running from Cargo use `cargo run /path/to/lithp/file.lthp` or `./lithp /path/to/lithp/file.lthp` if running from a binary.
