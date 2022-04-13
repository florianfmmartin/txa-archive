# Texo assembly

The assembly language as a token based syntax

It can use any defined VM and stack operations

It may define procedures with this syntax :

```asm
define $square
    local @n
    @n @n mul
endef
```

To interact with the procedure scope use :

- `local @IDENTIFIER` to pop a value from the stack and store it in scope
- `@IDENTIFIER` to push a value stored in scope to the stack

While searching for an `IDENTIFIER` in the scope, if it is not found, the search will continue into the parent's scope until the root scope as been reached

The other VM operations are :

- `:IDENTIFIER` makes a label to jump to
- `BOOL jump :IDENTIFIER` jumps to the label if the `BOOL` is truthy

Comments are done with `#[_` to open a comment and `_]#` to end it where `_` is a necessary whitespace 

More complete example :

```asm
define $fizzbuzz #[ n -- ]#
    local @n

    @n 3 mod
    0 eq
    local @three

    @n 5 mod
    0 eq
    local @five

    "fizzbuzz" @three @five and jump :return
    pop

    "fizz" @three jump :return
    pop

    "buzz" @five jump :return
    pop

    @n

:return
    print
endef

define $count-down #[ n -- ]#
    local @n
:loop
    @n print

    @n 1 sub
    local @n

    @n 0 lst
    jump :end

    1 jump :loop
:end
endef

define $main
    17 $fizzbuzz #[ prints 17 ]#
    15 $fizzbuzz #[ prints "fizzbuzz" ]#
    9 $fizzbuzz #[ prints "fizz" ]#
    10 $fizzbuzz #[ prints "buzz" ]#

    5 $count-down
    #[
        prints 5 4 3 2 1 0
    ]#
endef
```

## Todo

- [x] check for loops
- [x] debug flag
- [x] Optimize labels with `jump index_modifier` for less hashmap.get
- [x] stack manipulation operations (dup swp ovr nip rot)
- [ ] return stack (trs tsr rop)

