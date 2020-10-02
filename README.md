# Straight-line programming language

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