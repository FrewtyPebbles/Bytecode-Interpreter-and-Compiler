use std::str::Chars;
use crate::vm::{vm::Executor, bytecodes::BytecodeBuilder};
use std::collections::HashMap;
pub struct Parser {
    /// This is the raw source code
    src:String,
    /// This stores the variables
    vars:HashMap<String, u32>
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
                'n' => {
                    if escape {
                        string.push('\n');
                        escape = false;
                    } else {
                        string.push('n');
                    }
                },
                't' => {
                    if escape {
                        string.push('\t');
                        escape = false;
                    } else {
                        string.push('t');
                    }
                }
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
                        bb.write_num(instr_prts[2].clone().parse::<u32>().unwrap(), Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_num(instr_prts[2].clone().parse::<u32>().unwrap(), None));
                    }
                },
                "ADD" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_add(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_add(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "SUB" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_sub(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_sub(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "MUL" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_mul(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_mul(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "DIV" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_div(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_div(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "MOD" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_mod(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_mod(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "EXP" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_exp(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_exp(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "EQ" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_eq(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_eq(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "NEQ" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_neq(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_neq(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "GT" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_gt(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_lt(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "LT" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_lt(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_lt(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "GTE" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_gte(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_gte(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "LTE" => {
                    if self.vars.contains_key(&instr_prts[1]) {
                        bb.write_lte(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], Some(self.vars[&instr_prts[1]]));
                    } else {
                        self.vars.insert(instr_prts[1].clone(), bb.write_lte(self.vars[&instr_prts[2]], self.vars[&instr_prts[3]], None));
                    }
                },
                "STDOUT" => {
                    bb.write_stdout(self.vars[&instr_prts[1]]);
                },
                _ => {}
            }
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
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
        assert_eq!(true, true);
    }
}