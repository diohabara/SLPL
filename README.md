# Straight-line programming language

![CI](https://github.com/diohabara/SLPL/workflows/CI/badge.svg)

## Grammar

```text
Stm -> Stm; Stm
Stm -> id := Exp
Stm -> print( ExpList )
Exp -> id
Exp -> Exp Binop Exp
Exp -> ( Stm, Exp )
ExpList -> Exp, ExpList
ExpList -> Exp
Binop -> +
Binop -> -
Binop -> *
Binop -> /
```