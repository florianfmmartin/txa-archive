# Texo lang

## VM

The VM as two primitive types 64-bit integers and strings

It is a stack based machine

It reads tokens that can define procedures acting on the stack

Procedures have their own scope to reuse values seen on the stack via named variables or a scoped stack

## Texo assembly `.txa`

[More details here](./txa/)

[Vscode syntax extension here](./vscode-txa/txa-lang/)

Quick example

```asm
define $count-down #[ (n --) will print stuff ]#
    local @n
:loop
    @n print
    @n 1 sub
    local @n
    @n 0 lst jump :end
    1 jump :loop
:end
endef
```
