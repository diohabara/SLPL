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

fn eval(s: &str) {
    unimplemented!();
}

#[test]
fn eval_test() {
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
}

fn main() {
    println!("SLPL")
}
