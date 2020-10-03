use std::cmp::max;
use std::collections::HashMap;

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
    println!("This program should print\n8 7\n80");
    println!("Program is booting...");
    let prog = CompoundStm(
        Box::new(AssignStm(
            "a".to_string(),
            Box::new(OpExp(Box::new(NumExp(6)), Div, Box::new(NumExp(2)))),
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
    let mut interpreter = Interpreter::new();
    interpreter.eval_stm(&prog);
}

fn maxargs_expr(expr: &Exp) -> i32 {
    match *expr {
        IdExp(_) => 0,
        NumExp(_num) => 0,
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
            max(maximum, exprs.len() as i32)
        }
    }
}

#[test]
fn test_maxargs() {
    let prog = CompoundStm(
        Box::new(AssignStm(
            "a".to_string(),
            Box::new(OpExp(Box::new(NumExp(5)), Plus, Box::new(NumExp(3)))),
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
    assert_eq!(maxargs(&prog), 2);
}

struct Interpreter {
    env: HashMap<Id, i32>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    fn eval_expr(&mut self, expr: &Exp) -> i32 {
        match *expr {
            IdExp(ref id) => self.env[id],
            NumExp(num) => num,
            OpExp(ref e1, ref op, ref e2) => {
                let val1 = self.eval_expr(&e1);
                let val2 = self.eval_expr(&e2);
                match *op {
                    Plus => val1 + val2,
                    Minus => val1 - val2,
                    Times => val1 * val2,
                    Div => val1 / val2,
                }
            }
            ExpList(ref stm, ref e) => {
                self.eval_stm(&stm);
                self.eval_expr(e)
            }
        }
    }

    fn eval_stm(&mut self, stm: &Stm) {
        match *stm {
            CompoundStm(ref stm1, ref stm2) => {
                self.eval_stm(stm1);
                self.eval_stm(stm2);
            }
            AssignStm(ref id, ref expr) => {
                let val = self.eval_expr(&expr);
                self.env.insert(id.clone(), val);
            }
            PrintStm(ref exprs) => {
                let exprs = exprs.iter().map(|expr| self.eval_expr(expr));
                for expr in exprs {
                    print!("{} ", expr);
                }
                println!();
            }
        }
    }
}
