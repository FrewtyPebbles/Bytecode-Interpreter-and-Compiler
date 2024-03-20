mod vm;
mod lexer;
use vm::{vm::Executor, bytecodes::BytecodeBuilder};
fn main() {
    let mut bb = BytecodeBuilder::new();
    
    bb.write_start();
    
    let index = bb.write_num(0, None);
    let max = bb.write_num(1000, None);
    let inc = bb.write_num(1, None);
    
    let h_str = bb.write_str("Hello!\n".to_string(), None);
    let h_str2 = bb.write_str("My name is William.\n".to_string(), None);
    

    let block = bb.write_block(None);
    bb.write_stdout(h_str);
    bb.write_stdout(h_str2);
    
    let lt = bb.write_lt(index, max, None);
    bb.write_add(index, inc, Some(index));
    bb.write_cond_jump(block, lt);

    let mut exec = Executor::new(bb.src);
    use std::time::Instant;
    let now = Instant::now();
    exec.run();
    let elapsed = now.elapsed();
    println!("runtime: {:.4?}", elapsed);
}
