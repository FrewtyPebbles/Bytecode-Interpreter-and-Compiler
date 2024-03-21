mod vm;
mod lexer;
use vm::{vm::Executor, bytecodes::BytecodeBuilder};

use crate::lexer::asm::Parser;
fn main() {
    let mut lex = Parser::new(String::from(
        "
        NUM ind 0
        NUM sum 0
        NUM inc 1
        NUM adder -1.289893
        NUM itterations 1000
        STR nl \"\\n\"

        BLOCK loop
            MUL additive ind adder
            ADD sum sum additive

        ADD ind ind inc
        LT loopcond ind itterations
        COND_JUMP loopcond loop
        JUMP finished

        START
            LT loopcond ind itterations
            COND_JUMP loopcond loop
            BLOCK finished
            STDOUT sum
            STDOUT nl
        "
    ));
    use std::time::Instant;
    let now = Instant::now();
    lex.run();
    let elapsed = now.elapsed();
    println!("runtime: {:.4?}", elapsed);
}
