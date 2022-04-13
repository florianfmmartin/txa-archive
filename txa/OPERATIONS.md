# Operations

## Push values
`6` makes ` -- 6`

`"hello"` makes ` -- "hello"`

## Stack manipulations
`pop` makes `a -- `

`dup` makes `a -- a a`

`swp` makes `a b -- b a`

`ovr` makes `a b -- a b a`

`rot` makes `a b c -- b c a`

`print` makes `a -- `   with printing `a`

## Arithmetics
`add` makes `3 4 -- 7`; makes `"hello, " "world!" -- "hello, world!"`

`sub` makes `6 1 -- 5`

`mul` makes `3 4 -- 12`; makes `"yo" 3 -- "yoyoyo"`

`div` makes `13 3 -- 4`

`mod` makes `13 3 -- 1`

## Boolean operations
`#t` is a truthy value; an int bigger than 0

`#f` is a falsy value; an int smaller or equal to zero

`( )` is a 1 if true; a 0 if false

`and` makes `a b -- (a && b)`

`ior` makes `a b -- (a || b)`

`xor` makes `a b -- (a X| b)`

`not` makes `#t -- 0`; makes `#f -- 1`

## Comparison
`equ` makes `a b -- (a == b)`

`neq` makes `a b -- (a != b)`

`lst` makes `3 4 -- 1`; makes `4 3 -- 0`; makes `4 4 -- 0`

`lst` makes `3 4 -- 0`; makes `4 3 -- 1`; makes `4 4 -- 0`

## Return stack
All previous operations operate only on the general stack

The general stack will be noted `s`, and the scoped stack `r`

`tsr` makes `s: a -- ` `r: -- a`

`trs` makes `s: -- a` `r: a -- `

`csr` makes `s: a -- a` `r: -- a`

`crs` makes `s: -- a` `r: a -- a`

`rpop` makes `s: a -- a` `r: a -- `
