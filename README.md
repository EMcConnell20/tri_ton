This crate provides the `tri!` macro, an alternative to the `?`
operator that handles exceptions rather than forwarding them.

There are five "Tri Operators" that each have unique behaviors.

Tri-Fail `->` automatically returns a given expression as *Result::Err* when
a match fails.

Tri-Fall `<>` uses a given set of expressions as a fallback values when a
match fails.

Tri-Return `#>` returns a given expression without a *Result::Err* wrapper.
It can also be used as a break expression.

Tri-Until `%>` evaluates an expression until it yields a specific value.

Tri-While `>>` is like a do-while loop that evaluates an expression with
a set of values returned from the while condition.
(Read the documentation for more on that one.)
