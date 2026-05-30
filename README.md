## What is it?
This is my study project where I build a math expressions parser.  
The crate implements the Pratt algorithm, which is very flexible and extensible.  
Development was guided by this [article](https://dronperminov.ru/articles/math-expressions-parsing-in-vanilla-javascript-part-five-pratt-parser).

The crate includes functionality for parsing math expressions to an AST:  

`3*(4/4.54)-34`  

...and a calculator to get the result of evaluating the expression.

### Example:
```rust
let calculation_result: f64 = calculate("23.34^3/3*(3-4)").unwrap();
```

I plan to use this crate as both a library and a binary.  
With the binary, you can enter an interactive mode, write expressions and get a result, or pass an argument to the binary and get the result as output.
