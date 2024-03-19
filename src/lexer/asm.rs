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
                "STDOUT" => {
                    bb.write_stdout(self.vars[&instr_prts[1]]);
                },
                _ => {}
            }
        }
        let mut exec = Executor::new(bb.src);
        use std::time::Instant;
        let now = Instant::now();
        exec.run();
        let elapsed = now.elapsed();
        println!("runtime: {:.4?}", elapsed);
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
        lex.run();
        assert_eq!(true, true);
    }
}