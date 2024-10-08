 This crate offers the `tri!` macro, a tool for concisely writing
 tedious **try-except** statements.

 Having to unwrap an option or result from a function is a
 common task. Although the `?` operator can be useful, it
 forwards exceptions rather than handling them.

```toml
[dependencies]
tri_ton = "0.0.2"
```

```rust
use tri_ton::tri;
```

```rust
// Try Formats
tri!(a => b $$ c);
tri!(a => b(A) $$ c);
tri!(a => b[B] $$ c);
tri!(a => [R] $$ c);
```
 
 * `$$` - A Tri Operator
 * `a` - The Expression to Evaluate
 * `b` - The Expected Output of **a**
 * `c` - Alternative Expression(s).
 * `A` - A Field of the Enum Variant **b**
 * `B` - A Field of the Enum Variant **b**
 * `R` - A Pattern-Rule to Compare to the Output of **a**

`a` can be any form of expression in the rust language.
 **foo()**, **foo::BAR**, and **5_usize** are acceptable expressions.

 `b` can be most enum variants and paths. Items such as
 **None** and **crate::foo::\<bar>::cin** are acceptable paths.

 `c` can be a single or multiple alternate expressions. These
 expressions are usually evaluated in some form when the
 output or value of `a` does not match `b`.

`A` can be a variable declaration or null pattern. Multiple
 comma-separated items can be specified if the enum variant
 has multiple fields. **foo**, **ref mut bar**, and **_** are all
 acceptable items.

 In the expression`tri!(foo => Some(bar) $$ ...)`, *bar* is
 returned like the output of a function. In the expression
 `let cin = tri!(foo => Some(bar) $$ ...)`, the value *bar*
 will be bound to the variable *cin*.

`B` is similar to `A`. However, variables declared like **B**
 are automatically bound within the local scope. In the
 expression `tri!(foo => bar[cin] $$ ...)`, the variable *cin* is
 automatically bound within the same scope as the *tri* macro.

`R` is for matching non-enum values to patterns. **..foo**, 
**_**, and **(FOO, 0..=bar)** are all acceptable patterns.

 # Tri Expressions

 `tri!` has five operators for handling exceptions.
 - Tri-Fall
 - Tri-Fail
 - Tri-Return
 - Tri-Until
 - Tri-While

 ### Tri-Fall

 The `<>` operator can be used to provide a fallback value
 if an expression doesn't match the given term.

 ```rust
 use tri_ton::tri;
 
fn main() {
    let foo = Some(true);
    
    // If `foo` is Some(bar), `cin` is initialized with `bar`.
    // If `foo` isn't Some,  `cin` is initialized as false.
    let cin = tri!(foo => Some(bar) <> false);
}
 ```

 ### Tri-Fail

 The `->` operator returns the trailing expression in an error
 if the expression doesn't match the given term.

 ```rust
use tri_ton::tri;

fn foo_bar() -> Result<bool, &'static str> {
    let foo = Some(true);
    
    // If `foo` isn't Some, Err("Error!") is automatically returned.
    tri!(foo => Some[bar] -> "Error!");
    
    // `bar` is now an accessible variable.
    Ok(bar)
}
 ```

 ### Tri-Return

 The `#>` operator returns the trailing expression without an
 error wrapper. It can also be used as a break expression.

 ```rust
 #[no_std]
use tri_ton::tri;

fn foo_loop() -> Result<(), &'static str> {
    let foo = Some(true);
    
    // If `foo` isn't Some, Err("Custom Error!") gets returned.
    tri!(foo => Some[bar] #> Err("Custom Error!"));
    
    'a: loop {
        // If `bar` isn't false, the loop will be broken.
        tri!(bar => [false] #> break 'a);
    }
}
```

 ### Tri-Until

 The `%>` operator repeatedly evaluates the leading expression
 until it matches the given term. For every time the expression
 does not match the given term, the tailing expression is
 evaluated.

```rust
use tri_ton::tri;

fn main() {
     let mut foo: u8 = 0;
    
    // Until `foo` equals 10, the loop will increment `foo`.
     tri!(foo => [10] %> foo += 1);
}
 ```

 ### Tri-While

 The `>>` operator is similar to a *do-while* loop. The tailing
 expression is evaluated with an initial set of values. The
 leading expression is then evaluated in a loop, and for every
 time that it matches the given term, the trailing expression
 is evaluated with those values.

```rust
use tri_ton::tri;

fn main() { 
    let foo = |a: u8| -> Option<u8> {
        if a >= 10 { None }
        else { Some(a) }
    };
    
    // This is performed until foo(bar) returns None.
    tri!(foo(bar) => Some[mut bar = 0] >> bar += 1);
    assert_eq!(bar, 10);
    
    // "bar += 1" is performed before "foo(bar)" is checked.
    tri!(foo(bar) => Some[mut bar = bar] >> bar += 1);
    assert_eq!(bar, 11);
}
```
