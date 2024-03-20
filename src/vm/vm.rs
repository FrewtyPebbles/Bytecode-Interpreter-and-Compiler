use std::{collections::HashMap, io::stdin, ops::{Add, Div, Index, IndexMut, Mul, Rem, Sub}};

use super::bytecodes::{ByteCode, ByteType};

use super::bytecodes as bc;

#[derive(Clone)]
enum ScalarType {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    None
}

impl ScalarType {
    fn pow(&mut self, other:Self) -> f32 {
        return match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0.pow(r0 as u32) as f32,
            (Self::Float(l0), Self::Float(r0)) => l0.powf(r0),
            (Self::Int(l0), Self::Float(r0)) => (*l0 as f32).powf(r0),
            (Self::Float(l0), Self::Int(r0)) => l0.powf(r0 as f32),
            _ => 0.0,
        }
    }
}

impl Add for ScalarType {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Self::Int(l0), Self::Int(r0)) => Self::Int(l0 + r0),
            
            (Self::Float(l0), Self::Float(r0)) => Self::Float(l0 + r0),
            
            (Self::Int(l0), Self::Float(r0)) => Self::Float(l0 as f32 + r0),
            (Self::Float(l0), Self::Int(r0)) => Self::Float(l0 + r0 as f32),
            
            (Self::Str(l0), Self::Str(r0)) => Self::Str(l0 + &r0),
            
            (Self::Bool(l0), Self::Bool(r0)) => Self::Float((l0 as i32) as f32 + (r0 as i32) as f32),
            
            (Self::Int(l0), Self::Bool(r0)) => Self::Int(l0 + r0 as i32),
            (Self::Bool(l0), Self::Int(r0)) => Self::Int(l0 as i32 + r0),

            (Self::Float(l0), Self::Bool(r0)) => Self::Float(l0 + (r0 as i32) as f32),
            (Self::Bool(l0), Self::Float(r0)) => Self::Float((l0 as i32) as f32 + r0),
            
            (Self::Str(l0), Self::Bool(r0)) => Self::Str(l0 + if r0 {"true"} else {"false"}),
            (Self::Bool(l0), Self::Str(r0)) => Self::Str(if l0 {"true"} else {"false"}.to_string() + &r0),
            
            (Self::Str(l0), Self::Int(r0)) => Self::Str(format!("{}{}", l0, r0)),
            (Self::Int(l0), Self::Str(r0)) => Self::Str(format!("{}{}", l0, r0)),
            
            (Self::Str(l0), Self::Float(r0)) => Self::Str(format!("{}{}", l0, r0)),
            (Self::Float(l0), Self::Str(r0)) => Self::Str(format!("{}{}", l0, r0)),

            (Self::Str(l0), Self::None) => Self::Str(format!("{}Null", l0)),
            (Self::None, Self::Str(r0)) => Self::Str(format!("Null{}", r0)),
            _ => panic!("Illegal ADD operation!"),
        }
    }
}

impl Sub for ScalarType {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Self::Int(l0), Self::Int(r0)) => Self::Int(l0 - r0),
            
            (Self::Float(l0), Self::Float(r0)) => Self::Float(l0 - r0),
            
            (Self::Int(l0), Self::Float(r0)) => Self::Float(l0 as f32 - r0),
            (Self::Float(l0), Self::Int(r0)) => Self::Float(l0 - r0 as f32),
            
            (Self::Str(l0), Self::Str(r0)) => Self::Str(l0.replace(&r0, "")),
            
            (Self::Bool(l0), Self::Bool(r0)) => Self::Float((l0 as i32) as f32 - (r0 as i32) as f32),
            
            (Self::Int(l0), Self::Bool(r0)) => Self::Int(l0 - r0 as i32),
            (Self::Bool(l0), Self::Int(r0)) => Self::Int(l0 as i32 - r0),

            (Self::Float(l0), Self::Bool(r0)) => Self::Float(l0 - (r0 as i32) as f32),
            (Self::Bool(l0), Self::Float(r0)) => Self::Float((l0 as i32) as f32 - r0),
            
            (Self::Str(l0), Self::Bool(r0)) => Self::Str(l0.replace(if r0 {"true"} else {"false"}, "")),
            
            (Self::Str(l0), Self::Int(r0)) => Self::Str(l0.replace(format!("{}", r0).as_str(), "")),
            
            (Self::Str(l0), Self::Float(r0)) => Self::Str(l0.replace(format!("{}", r0).as_str(), "")),

            (Self::Str(l0), Self::None) => Self::Str(l0.replace("Null", "")),
            
            _ => panic!("Illegal SUB operation!"),
        }
    }
}

impl Mul for ScalarType {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Self::Int(l0), Self::Int(r0)) => Self::Int(l0 * r0),
            
            (Self::Float(l0), Self::Float(r0)) => Self::Float(l0 * r0),
            
            (Self::Int(l0), Self::Float(r0)) => Self::Float(l0 as f32 * r0),
            (Self::Float(l0), Self::Int(r0)) => Self::Float(l0 * r0 as f32),
                        
            (Self::Bool(l0), Self::Bool(r0)) => Self::Float((l0 as i32) as f32 * (r0 as i32) as f32),
            
            (Self::Int(l0), Self::Bool(r0)) => Self::Int(l0 * r0 as i32),
            (Self::Bool(l0), Self::Int(r0)) => Self::Int(l0 as i32 * r0),

            (Self::Float(l0), Self::Bool(r0)) => Self::Float(l0 * (r0 as i32) as f32),
            (Self::Bool(l0), Self::Float(r0)) => Self::Float((l0 as i32) as f32 * r0),
                        
            (Self::Str(l0), Self::Int(r0)) => todo!(),
            (Self::Int(l0), Self::Str(r0)) => todo!(),
            
            (Self::Str(l0), Self::Float(r0)) => todo!(),
            (Self::Float(l0), Self::Str(r0)) => todo!(),

            _ => panic!("Illegal MUL operation!"),
        }
    }
}

impl Div for ScalarType {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Self::Int(l0), Self::Int(r0)) => Self::Int(l0 / r0),
            
            (Self::Float(l0), Self::Float(r0)) => Self::Float(l0 / r0),
            
            (Self::Int(l0), Self::Float(r0)) => Self::Float(l0 as f32 / r0),
            (Self::Float(l0), Self::Int(r0)) => Self::Float(l0 / r0 as f32),
                        
            (Self::Bool(l0), Self::Bool(r0)) => Self::Float((l0 as i32) as f32 / (r0 as i32) as f32),
            
            (Self::Int(l0), Self::Bool(r0)) => Self::Int(l0 / r0 as i32),
            (Self::Bool(l0), Self::Int(r0)) => Self::Int(l0 as i32 / r0),

            (Self::Float(l0), Self::Bool(r0)) => Self::Float(l0 / (r0 as i32) as f32),
            (Self::Bool(l0), Self::Float(r0)) => Self::Float((l0 as i32) as f32 / r0),

            _ => panic!("Illegal DIV operation!"),
        }
    }
}

impl Rem for ScalarType {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Self::Int(l0), Self::Int(r0)) => Self::Int(l0 % r0),
            
            (Self::Float(l0), Self::Float(r0)) => Self::Float(l0 % r0),
            
            (Self::Int(l0), Self::Float(r0)) => Self::Float(l0 as f32 % r0),
            (Self::Float(l0), Self::Int(r0)) => Self::Float(l0 % r0 as f32),
                        
            (Self::Bool(l0), Self::Bool(r0)) => Self::Float((l0 as i32) as f32 % (r0 as i32) as f32),
            
            (Self::Int(l0), Self::Bool(r0)) => Self::Int(l0 % r0 as i32),
            (Self::Bool(l0), Self::Int(r0)) => Self::Int(l0 as i32 % r0),

            (Self::Float(l0), Self::Bool(r0)) => Self::Float(l0 % (r0 as i32) as f32),
            (Self::Bool(l0), Self::Float(r0)) => Self::Float((l0 as i32) as f32 % r0),

            _ => panic!("Illegal MUL operation!"),
        }
    }
}

impl PartialEq for ScalarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for ScalarType {
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 < r0,
            (Self::Float(l0), Self::Float(r0)) => l0 < r0,
            (Self::Str(l0), Self::Str(r0)) => l0 < r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 < r0,
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 <= r0,
            (Self::Float(l0), Self::Float(r0)) => l0 <= r0,
            (Self::Str(l0), Self::Str(r0)) => l0 <= r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 <= r0,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 > r0,
            (Self::Float(l0), Self::Float(r0)) => l0 > r0,
            (Self::Str(l0), Self::Str(r0)) => l0 > r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 > r0,
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 >= r0,
            (Self::Float(l0), Self::Float(r0)) => l0 >= r0,
            (Self::Str(l0), Self::Str(r0)) => l0 >= r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 >= r0,
            _ => false,
        }
    }
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        else if self > other {
            return Some(std::cmp::Ordering::Greater);
        }
        else if self < other {
            return Some(std::cmp::Ordering::Less);
        }
        else {
            return None;
        }
    }
}

type StackFrame = HashMap<u32, ScalarType>;

struct ScopeStack {
    stack:Vec<StackFrame>
}

impl ScopeStack {
    fn new() -> ScopeStack {
        ScopeStack {
            stack: vec![HashMap::new()]
        }
    }

    fn top(&mut self) -> &mut StackFrame {
        return self.stack.last_mut().unwrap();
    }

    fn new_scope(&mut self) -> () {
        self.stack.push(HashMap::new())
    }

    fn alloca(&mut self, key:u32) -> () {
        self.top().insert(key, ScalarType::None);
    }

    fn set(&mut self, key:u32, val:ScalarType) {
        self.top().insert(key, val);
    }

    fn get(&mut self, key:u32) -> Result<ScalarType, String> {
        self.stack.reverse();
        for scope in self.stack.iter_mut() {
            if scope.contains_key(&key) {
                let item = scope[&key].clone();
                self.stack.reverse();
                return Ok(item);
            }
        }
        Err(format!("Unknown memory {} referenced.", key))
    }

    fn remove(&mut self, key:u32) -> Result<(), String> {
        self.stack.reverse();
        for scope in self.stack.iter_mut() {
            if scope.contains_key(&key) {
                scope.remove(&key);
                self.stack.reverse();
                return Ok(());
            }
        }
        Err(format!("Tried to delete memory {} which does not exist.", key))
    }

    fn pop_scope(&mut self) -> () {
        self.stack.pop();
    }
}

#[derive(Clone)]
struct ByteCursor {
    src:Box<ByteCode>,
    cursor: i32
}

impl ByteCursor {
    fn new(src:Box<ByteCode>) -> ByteCursor {
        return ByteCursor {
            src: src,
            cursor: -1
        }
    }

    fn finished(&self) -> bool {
        return self.cursor+1 == self.src.len() as i32;
    }

    fn _next(&mut self) -> Result<ByteType, ()> {
        if self.finished() {
            self.cursor = -1;
            return Err(());
        }
        self.cursor += 1;

        return Ok(self.src.at(self.cursor as usize));
    }

    fn at(&mut self, index: usize) -> ByteType {
        self.cursor = index as i32;
        let ret = self.src.at(index);
        return ret;
    }
    
    fn set(&mut self, index: usize, item:u32) {
        self.cursor = index as i32;
        self.src.set(index, item);
    }

    fn prev(&mut self) -> ByteType {
        self.cursor -= 1;
        return self.src.at(self.cursor as usize);
    }

    fn jump(&mut self, ind:u32) -> () {
        self.cursor = ind as i32;
    }

    fn current(&self) -> ByteType {
        return self.src.at(self.cursor as usize);
    }
}

impl Iterator for ByteCursor {
    type Item = ByteType;

    fn next(&mut self) -> Option<Self::Item> {
        return match self._next() {
            Ok(_data) => Some(_data),
            Err(_) => None,
        }
    }
}

pub struct Executor {
    blocks: HashMap<u32, u32>,
    bytecode: ByteCursor,
    stack: ScopeStack
}

impl Executor {
    pub fn new(bytecode:Box<ByteCode>) -> Executor {
        Executor {
            blocks: HashMap::new(),
            bytecode: ByteCursor::new(bytecode),
            stack: ScopeStack::new()
        }
    }

    pub fn run(&mut self) {
        for byt in self.bytecode.clone() {
            if byt == bc::BLOCK {
                self._block();
            }
        }

        self.bytecode.cursor = -1;

        let mut start = false;

        while !self.bytecode.finished() {
            if let ByteType::Num(byt) = self.bytecode._next().unwrap() {
                if start {
                    match byt {
                        bc::ENDL => {},
                        bc::ALLOCA => self._alloca(),
                        bc::STORE => self._store(),
                        bc::DEL => self._del(),
                        bc::EQ => self._eq(),
                        bc::GT => self._gt(),
                        bc::GTE => self._gte(),
                        bc::LT => self._lt(),
                        bc::LTE => self._lte(),
                        bc::NEQ => self._neq(),
                        bc::ADD => self._add(),
                        bc::SUB => self._sub(),
                        bc::MUL => self._mul(),
                        bc::DIV => self._div(),
                        bc::MOD => self._mod(),
                        bc::EXP => self._exp(),
                        bc::NUM => self._num(),
                        bc::STR => self._str(),
                        bc::FMT => self._fmt(),
                        bc::STDOUT => self._stdout(),
                        bc::STDIN => self._stdin(),
                        bc::BEGIN_SCOPE => self.stack.new_scope(),
                        bc::END_SCOPE => self.stack.pop_scope(),
                        bc::BLOCK => self._block(),
                        bc::JUMP => self._jump(),
                        bc::COND_JUMP => self._cond_jump(),
                        bc::CAST_STR => self._cast_str(),
                        bc::CAST_NUM => self._cast_num(),
                        bc::FMT_NUM => self._fmt_num(),
                        _=> ()
                    }
                }
                else {
                    match byt {
                        bc::ALLOCA => {self._alloca()},
                        bc::NUM => {self._num()},
                        bc::STR => {self._str()},
                        bc::START => {start = true},
                        _ => ()
                    }
                }
            }
        }
    }

    fn _next(&mut self) -> Result<u32, String> {
        if let ByteType::Num(bcode) = self.bytecode._next().unwrap() {
            return Ok(bcode);
        }
        return Err(String::from("Expected a u32"));
    }

    fn _next_str(&mut self) -> Result<String, String> {
        if let ByteType::Str(bcode) = self.bytecode._next().unwrap() {
            return Ok(bcode);
        }
        return Err(String::from("Expected a u32"));
    }

    fn _block(&mut self) {
        let block = self._next().unwrap();
        self.blocks.insert(block, self.bytecode.cursor as u32);
        self.bytecode._next().unwrap();
    }

    fn _alloca(&mut self) {
        let cid = self._next().unwrap();
        self.stack.alloca(cid);
        self.bytecode._next().unwrap();
    }

    fn _store(&mut self) {
        let id = self._next().unwrap();
        let _val = self._next().unwrap();
        let val = self.stack.get(_val).unwrap();
        self.stack.set(id, val);
        
        self.bytecode._next().unwrap();
    }

    fn _del(&mut self) {
        let id = self._next().unwrap();
        self.stack.remove(id);
        self.bytecode._next().unwrap();
    }

    fn _eq(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, ScalarType::Bool(lhs == rhs));
        self.bytecode._next().unwrap();
    }

    fn _gt(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, ScalarType::Bool(lhs > rhs));
        self.bytecode._next().unwrap();
    }

    fn _lt(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, ScalarType::Bool(lhs < rhs));
        self.bytecode._next().unwrap();
    }

    fn _gte(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, ScalarType::Bool(lhs >= rhs));
        self.bytecode._next().unwrap();
    }

    fn _lte(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, ScalarType::Bool(lhs <= rhs));
        self.bytecode._next().unwrap();
    }

    fn _neq(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, ScalarType::Bool(lhs != rhs));
        self.bytecode._next().unwrap();
    }

    fn _add(&mut self) {
        let cid = self._next().unwrap();
        
        let _lhs = self._next().unwrap();
        
        let lhs = self.stack.get(_lhs).unwrap();
        
        let _rhs = self._next().unwrap();
        
        let rhs = self.stack.get(_rhs).unwrap();

        self.stack.set(cid, lhs + rhs);
        self.bytecode._next().unwrap();
    }

    fn _sub(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, lhs - rhs);
        self.bytecode._next().unwrap();
    }

    fn _mul(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, lhs * rhs);
        self.bytecode._next().unwrap();
    }

    fn _div(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, lhs / rhs);
        self.bytecode._next().unwrap();
    }

    fn _mod(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, lhs % rhs);
        self.bytecode._next().unwrap();
    }

    fn _exp(&mut self) {
        let cid = self._next().unwrap();
        let _lhs = self._next().unwrap();
        let mut lhs = self.stack.get(_lhs).unwrap();
        let _rhs = self._next().unwrap();
        let rhs = self.stack.get(_rhs).unwrap();
        self.stack.set(cid, ScalarType::Float(lhs.pow(rhs)));
        self.bytecode._next().unwrap();
    }

    fn _stdin(&mut self) {
        let cid = self._next().unwrap();
        let mut inp = String::new();
        stdin().read_line(&mut inp).unwrap();
        if let Some('\n') = inp.chars().next_back() {
            inp.pop();
        }
        if let Some('\r') = inp.chars().next_back() {
            inp.pop();
        }
        self.stack.set(cid, ScalarType::Str(inp));
        self.bytecode._next().unwrap();
    }

    fn _stdout(&mut self) {
        let _msg = self._next().unwrap();
        match self.stack.get(_msg).unwrap() {
            ScalarType::Str(msg) => {
                print!("{}", msg);
            },
            ScalarType::Int(msg) => {
                print!("{}", msg);
            },
            ScalarType::Bool(msg) => {
                print!("{}", if (msg){ "true" } else { "false" });
            },
            ScalarType::Float(msg) => {
                print!("{}", msg);
            },
            ScalarType::None => {
                print!("Null");
            }
        }
        self.bytecode._next().unwrap();
    }

    fn _num(&mut self) {
        let cid = self._next().unwrap();
        let mut num = String::new();
        let mut byt = self._next().unwrap();
        while byt != bc::ENDL {
            num += format!("{}", byt).as_str();
            byt = self._next().unwrap();
        }
        
        self.stack.set(cid, ScalarType::Float(num.parse::<f32>().unwrap()));
    }

    fn _str(&mut self) {
        let cid = self._next().unwrap();
        let _str = self._next_str().unwrap();
        self.stack.set(cid, ScalarType::Str(_str));
        self.bytecode._next().unwrap();
    }
    
    fn _cast_num(&mut self) {
        let cid = self._next().unwrap();
        let num_id = self._next().unwrap();
        if let ScalarType::Str(f) = self.stack.get(num_id).unwrap() {
            self.stack.set(cid, ScalarType::Float(f.parse::<u32>().unwrap() as f32));
        }
        self.bytecode._next().unwrap();
    }

    fn _fmt_num(&mut self) {
        let cid = self._next().unwrap();
        let _num = self._next().unwrap();
        if let ScalarType::Float(num) = self.stack.get(_num).unwrap() {
        let _precision = self._next().unwrap();
        if let ScalarType::Float(precision) = self.stack.get(_precision).unwrap() {
            if precision == 0.0 {
                self.stack.set(cid, ScalarType::Str(format!("{}", num as i32)));
            }
            else {
                self.stack.set(cid, ScalarType::Str(format!("{:.prec$}", num, prec = precision as usize)));
            }
        }}
        self.bytecode._next().unwrap();
    }

    fn _cast_str(&mut self) {
        let cid = self._next().unwrap();
        let item = self._next().unwrap();
        match self.stack.get(item).unwrap() {
            ScalarType::Int(_val) => {
                self.stack.set(cid, ScalarType::Str(format!("{}", _val)));
            },
            ScalarType::Float(_val) => {
                self.stack.set(cid, ScalarType::Str(format!("{}", _val)));
            },
            ScalarType::Bool(_val) => {
                self.stack.set(cid, ScalarType::Str(format!("{}", if _val {"true"} else {"false"})));
            },
            ScalarType::None => {
                self.stack.set(cid, ScalarType::Str(String::from("Null")));
            },
            ScalarType::Str(_val) => {},
        }
        self.bytecode._next().unwrap();
    }

    fn dyn_format(src:String, mut arguments:Vec<String>) -> String {
        arguments.reverse();
        let mut ret_str = String::new();
        for c in src.chars() {
            let mut formatting = false;
            match c {
                '{' => {
                    formatting = !formatting;
                    if !formatting {
                        ret_str.push('{');
                    }
                }
                '}' => {
                    if !formatting {
                        ret_str.push('}');
                    }
                    else {
                        ret_str += arguments.pop().expect("Incorrect number of arguments for format string.").as_str();
                    }
                    formatting = false;
                }
                _ => {
                    ret_str.push(c);
                }
            }
        }
        return ret_str;
    }

    fn _fmt(&mut self) {
        let cid = self._next().unwrap();
        let __string = self._next().unwrap();
        if let ScalarType::Str(_string) = self.stack.get(__string).unwrap() {
            let mut fmt_args = vec![];
            let mut byt = self._next().unwrap();
            while byt != bc::ENDL {
                match self.stack.get(byt).unwrap() {
                    ScalarType::Int(_val) => {
                        fmt_args.push(format!("{}", _val));
                    },
                    ScalarType::Float(_val) => {
                        fmt_args.push(format!("{}", _val));
                    },
                    ScalarType::Bool(_val) => {
                        fmt_args.push(format!("{}", _val));
                    },
                    ScalarType::Str(_val) => {
                        fmt_args.push(_val);
                    },
                    ScalarType::None => {
                        fmt_args.push(String::from("Null"));
                    },
                }
                byt = self._next().unwrap();
            }
            
            self.stack.set(cid, ScalarType::Str(Self::dyn_format(_string, fmt_args)));
        }
    }

    fn _jump(&mut self) {
        let block = self._next().unwrap();
        self.bytecode.jump(self.blocks[&block]);
    }

    fn _cond_jump(&mut self) {
        let block = self._next().unwrap();
        let _cond = self._next().unwrap();
        if let ScalarType::Bool(cond) = self.stack.get(_cond).unwrap() {
            if cond {
                self.bytecode.jump(self.blocks[&block]);
            } else {
                self.bytecode._next().unwrap();
            }
        }
    }
}