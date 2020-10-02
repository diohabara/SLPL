use std::cmp::max;

use BinOp::*;
use Exp::*;
use Stm::*;

type Id = String;

enum BinOp {
    Plus,
    Minus,
    Times,
    Div,
}

enum Exp {
    IdExp(Id),
    NumExp(i32),
    OpExp(Box<Exp>, BinOp, Box<Exp>),
    ExpList(Stm, Box<Exp>),
}
enum Stm {
    CompoundStm(Box<Stm>, Box<Stm>),
    AssignStm(Id, Box<Exp>),
    PrintStm(Vec<Exp>),
}

fn main() {
    println!("This is straight-line programing language!")
}

fn maxargs_expr(expr: &Exp) -> i32 {
    match *expr {
        IdExp(_) => 0,
        NumExp(num) => num,
        OpExp(ref e1, _, ref e2) => max(maxargs_expr(e1), maxargs_expr(e2)),
        ExpList(ref stm, ref expr) => max(maxargs(stm), maxargs_expr(expr)),
    }
}

fn maxargs(stm: &Stm) -> i32 {
    match *stm {
        CompoundStm(ref s1, ref s2) => max(maxargs(s1), maxargs(s2)),
        AssignStm(_, ref expr) => maxargs_expr(expr),
        PrintStm(ref exprs) => {
            let maximum = exprs
                .iter()
                .map(|expr| maxargs_expr(expr))
                .max()
                .unwrap_or(0);
            maximum
        }
    }
}

#[test]
fn maxargs_test() {
    let prog = CompoundStm(
        Box::new(AssignStm(
            "a".to_string(),
            Box::new(OpExp(Box::new(NumExp(4)), Plus, Box::new(NumExp(3)))),
        )),
        Box::new(CompoundStm(
            Box::new(AssignStm(
                "b".to_string(),
                Box::new(ExpList(
                    PrintStm(vec![
                        IdExp("a".to_string()),
                        OpExp(Box::new(IdExp("a".to_string())), Minus, Box::new(NumExp(1))),
                    ]),
                    Box::new(OpExp(
                        Box::new(NumExp(10)),
                        Times,
                        Box::new(IdExp("a".to_string())),
                    )),
                )),
            )),
            Box::new(PrintStm(vec![IdExp("b".to_string())])),
        )),
    );
    assert_eq!(maxargs(&prog), 10);
}
