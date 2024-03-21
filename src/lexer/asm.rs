use std::str::Chars;
use crate::vm::{vm::Executor, bytecodes::BytecodeBuilder};
use std::collections::HashMap;
pub struct Parser {
    /// This is the raw source code
    src:String,
    /// This stores the variables
    vars:HashMap<String, u32>
}

macro_rules! escape_character {
    ($c:expr, $esc:expr, $escape:ident, $string:ident) => {
        {
            if $escape {
                $string.push($esc);
                $escape = false;
            } else {
                $string.push($c);
            }
        }
    };
}

macro_rules! binary_emit {
    ($self:ident, $emit:ident, $instr_prts:ident, $bb:ident) => {
        {
            if $self.vars.contains_key(&$instr_prts[1]) {
                $bb.$emit($self.vars[&$instr_prts[2]], $self.vars[&$instr_prts[3]], Some($self.vars[&$instr_prts[1]]));
            } else {
                $self.vars.insert($instr_prts[1].clone(), $bb.$emit($self.vars[&$instr_prts[2]], $self.vars[&$instr_prts[3]], None));
            }
        }
    };
}

impl Parser {
    pub fn new(src:String) -> Parser {
        return Parser { src:src, vars:HashMap::new()};
    }
    /// Converts self.src into an itterable of strings.
    fn parse_instr(&mut self, instr:String) -> Vec<String> {
        let mut chars = instr.chars();
        let mut ret_inst:Vec<String> = vec![];
        let mut str_buff = String::new();
        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    //string
                    ret_inst.push(self.parse_str(&mut chars));
                }
                ' ' | '\t' | '\r' | '\n' => {
                    //num
                    ret_inst.push(str_buff.clone());
                    str_buff.clear();
                }
                _ => {
                    //label
                    str_buff.push(c);
                }
            };
        }
        if str_buff != "" {
            ret_inst.push(str_buff);
        }
        return ret_inst
    }

    fn parse_str(&mut self, instr:&mut Chars) -> String {
        let mut escape = false;
        let mut string = String::new();
        while let Some(c) = instr.next() {
            match c {
                '\\' => {
                    escape = true;
                },
                '"' => {
                    if escape {
                        string.push('\"');
                        escape = false;
                    } else {
                        return string;
                    }
                },
                'n' => escape_character!('n', '\n', escape, string),
                't' => escape_character!('t', '\t', escape, string),
                'r' => escape_character!('r', '\r', escape, string),
                '0' => escape_character!('0', '\0', escape, string),
                _ => {
                    string.push(c);
                }
            }
        }
        return string;
    }

    pub fn run(&mut self) {
        let mut src = self.src.clone();
        let instructions = src.lines();
        let mut bb = BytecodeBuilder::new();
        let mut jumps:HashMap<usize, String> = HashMap::new();
        for raw_instr in instructions {
            let instr = raw_instr.trim().to_string();
            if instr == "" || instr.starts_with("#") {
                continue
            }
            let instr_prts = self.parse_instr(instr);
            match instr_prts[0].as_str() {
                "START" => {
                    bb.write_start();
                }
                "ALLOCA" => {
                    self.vars.insert(instr_prts[1].clone(), bb.write_alloca(None));
                },
                "STR" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_str(instr_prts[2].clone(), Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_str(instr_prts[2].clone(), None));
                    }
                },
                "NUM" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_num(instr_prts[2].clone().parse::<f32>().unwrap(), Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_num(instr_prts[2].clone().parse::<f32>().unwrap(), None));
                    }
                },
                "BOOL" => {
                    let value = if instr_prts[2]=="true" {true} else if instr_prts[2]=="false" {false} else {panic!("BOOL value must be either `true` or `false`.")};
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_bool(value, Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_bool(value, None));
                    }
                },
                "ADD" => binary_emit!(self, write_add, instr_prts, bb),
                "SUB" => binary_emit!(self, write_sub, instr_prts, bb),
                "MUL" => binary_emit!(self, write_mul, instr_prts, bb),
                "DIV" => binary_emit!(self, write_div, instr_prts, bb),
                "MOD" => binary_emit!(self, write_mod, instr_prts, bb),
                "EXP" => binary_emit!(self, write_exp, instr_prts, bb),
                "EQ" => binary_emit!(self, write_eq, instr_prts, bb),
                "NEQ" => binary_emit!(self, write_neq, instr_prts, bb),
                "GT" => binary_emit!(self, write_gt, instr_prts, bb),
                "LT" => binary_emit!(self, write_lt, instr_prts, bb),
                "GTE" => binary_emit!(self, write_gte, instr_prts, bb),
                "LTE" => binary_emit!(self, write_lte, instr_prts, bb),
                "STDOUT" => {
                    bb.write_stdout(self.vars[&instr_prts[1]]);
                },
                "BLOCK" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_block(Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_block(None));
                    }
                },
                "JUMP" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_jump(self.vars[&instr_prts[1]]);
                    } else {
                        bb.write_jump(0);
                        jumps.insert(bb.src.len() - 1, instr_prts[1].clone());
                    }
                },
                "COND_JUMP" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_cond_jump(self.vars[&instr_prts[2]], self.vars[&instr_prts[1]]);
                    } else {
                        bb.write_cond_jump(0, self.vars[&instr_prts[1]]);
                        jumps.insert(bb.src.len() - 3, instr_prts[1].clone());
                    }
                },
                _ => {}
            }
        }
        // todo: Substitute the jumps so they can be called on blocks that are defered in their initialization.
        for (key, val) in jumps.iter() {
            bb.src.set(*key, self.vars[val]);
        }
        let mut exec = Executor::new(bb.src);
        exec.run();
    }
}



#[cfg(test)]
mod tests {
    use crate::lexer::asm::Parser;

    #[test]
    fn asm_test_hello_world() {
        let mut lex = Parser::new(String::from(
            "
            START
                STR hello \"Hello world!\\n\"
                STDOUT hello
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_add() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                ADD num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_bool() {
        let mut lex = Parser::new(String::from(
            "
            START
                BOOL mybool true
                STDOUT mybool
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_sub() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                SUB num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_mul() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                MUL num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }
    
    #[test]
    fn asm_test_div() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                DIV num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_mod() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                MOD num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_exp() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                EXP num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_eq() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                EQ num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_neq() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                NEQ num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_gt() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                GT num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_lt() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                LT num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_gte() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                GTE num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_lte() {
        let mut lex = Parser::new(String::from(
            "
            START
                NUM num1 7
                NUM num2 2
                LTE num3 num1 num2
                STDOUT num3
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }

    #[test]
    fn asm_test_cond_jump_prerender() {
        let mut lex = Parser::new(String::from(
            "
            NUM ind 0
            NUM sum 0
            NUM inc 1
            NUM itterations 1000
            STR nl \"\\n\"

            BLOCK loop

                ADD sum sum ind

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

    #[test]
    fn asm_test_cond_jump() {
        let mut lex = Parser::new(String::from(
            "
            NUM ind 0
            NUM sum 0
            NUM inc 1
            NUM itterations 10

            BLOCK loop

                ADD sum sum ind

            LT loopcond ind itterations
            ADD ind ind inc
            COND_JUMP loopcond loop

            START
                LT loopcond ind itterations
                COND_JUMP loopcond loop
                STDOUT sum
                STR nl \"\\n\"
                STDOUT nl
            "
        ));
        use std::time::Instant;
        let now = Instant::now();
        lex.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
    }
}