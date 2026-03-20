Rust code uses _snake case_ as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

We define a function in Rust by entering `fn` followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.

We can call any function we’ve defined by entering its name followed by a set of parentheses. Because `another_function` is defined in the program, it can be called from inside the `main` function. Note that we defined `another_function` _after_ the `main` function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

In function signatures, you *must* declare the type of each parameter. This is a deliberate decision in Rust's design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more-helpful error messages if it knows what types the function expects.

When defining multiple parameters, separate the parameter declarations with commas.

## Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression. So far, the functions we've covered included an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages do not have the same distinctions, so let's look at what statements and expressions are and how their differences affect the bodies of functions.

- *Statements* are instructions that perform some action and do not return a value
- *Expressions* evaluate to a resultant value

Creating a variable and assigning a value to it with the `let` keyword is a statement. `let y = 6;` is a statement.

Function definitions are also statements;

Statements do not return values. Therefore, you can't assign a `let` statement to another variable, as the following code tries to do; you will get an error:

```Rust
let x = (let y = 6);
```

The `let y = 6` statement does not return a value, so there isn't anything for x to bind to.

Expressions evaluate to a value and make up most of the rest of the code you'll write in Rust. Consider a math operation, such as 5+6 , which is an expression that evaluates to the value 11. Expressions can be part of statements. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression.

Expressions do not include ending semicolon. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions...

## Functions with Return values

Functions can return values to the code that calls them. We don't name return values, but we must declare their type after an arrow (`->`). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.