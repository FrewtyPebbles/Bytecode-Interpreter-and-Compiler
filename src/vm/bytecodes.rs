use std::{borrow::BorrowMut, cell::RefCell, collections::{HashMap, HashSet}, ops::Index, rc::{Rc, Weak}};

pub const __MAX_INSTR_INT__:u32 = 0x32;
pub const ENDL:u32 = 0xA;
pub const ALLOCA:u32 = 0xB;
pub const STORE:u32 = 0xC;
pub const DEL:u32 = 0xD;
pub const ADD:u32 = 0xE;
pub const SUB:u32 = 0xF;
pub const MUL:u32 = 0x10;
pub const DIV:u32 = 0x11;
pub const MOD:u32 = 0x12;
pub const JUMP:u32 = 0x13;
pub const BLOCK:u32 = 0x14;
pub const COND_JUMP:u32 = 0x15;
pub const EQ:u32 = 0x16;
pub const GT:u32 = 0x17;
pub const LT:u32 = 0x18;
pub const GTE:u32 = 0x19;
pub const LTE:u32 = 0x1A;
pub const NUM:u32 = 0x1B;
pub const STDOUT:u32 = 0x1C;
pub const STDIN:u32 = 0x1D;
pub const EXP:u32 = 0x1E;
pub const STR:u32 = 0x1F;
pub const FMT:u32 = 0x20;
pub const BEGIN_SCOPE:u32 = 0x21;
pub const END_SCOPE:u32 = 0x22;
pub const NEQ:u32 = 0x23;
pub const CAST_NUM:u32 = 0x24;
pub const CAST_STR:u32 = 0x25;
pub const FMT_NUM:u32 = 0x26;
pub const START:u32 = 0x27;

pub enum ByteType {
    Str(String),
    Num(u32)
}

impl PartialEq<u32> for ByteType {
    fn eq(&self, other: &u32) -> bool {
        if let ByteType::Num(this) = self {
            return other == this;
        }
        return false;
    }
}

#[derive(Clone, Debug)]
pub struct ByteCode {
    bytecode: Vec<u32>,
    strings: HashMap<u32, String>,
    current: u32,
    id_manager: Rc<RefCell<IDManager>>
}

impl ByteCode {
    pub fn new(id_manager:Rc<RefCell<IDManager>>) -> ByteCode {
        return ByteCode {
            bytecode: vec![],
            strings: HashMap::new(),
            current: 0,
            id_manager: id_manager
        };
    }

    pub fn len(&self) -> usize{
        self.bytecode.len()
    }
    pub fn append(&mut self, mut item: ByteType) {
        if let ByteType::Str(_str) = item {
            let cid = RefCell::borrow_mut(&self.id_manager).current_id();
            self.strings.insert(cid, _str);
            item = ByteType::Num(cid);
        }
        match item {
            ByteType::Num(_item) => self.bytecode.push(_item),
            _ => {},
        }
    }
    pub fn extend(&mut self, items:Vec<ByteType>) {
        let mut bc: Vec<u32> = vec![];
        for item in items.iter() {
            match item {
                ByteType::Str(_str) => {
                    let cid = RefCell::borrow_mut(&self.id_manager).current_id();
                    self.strings.insert(cid, _str.clone());
                    bc.push(cid);
                },
                ByteType::Num(_num) => {
                    bc.push(*_num);
                },
            }
        }
        self.bytecode.extend(bc);
    }

    pub fn set(&mut self, index: usize, item:u32) {
        self.bytecode[index] = item;
    }

    pub fn at(&self, index: usize) -> ByteType {
        let cur = self.bytecode[index];
        if self.strings.contains_key(&cur) {
            return ByteType::Str(self.strings[&cur].clone());
        }
        else {
            return ByteType::Num(cur);
        }
    }
}

impl Iterator for ByteCode {
    type Item = ByteType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.bytecode.len() as u32 {
            return None;
        }
        let cur = self.bytecode[self.current as usize];
        self.current += 1;
        if self.strings.contains_key(&cur) {
            return Some(ByteType::Str(self.strings[&cur].clone()));
        }
        else {
            return Some(ByteType::Num(cur));
        }
    }
}

#[derive(Debug)]
struct IDManager {
    _current_id:u32,
    existing_ids:HashSet<u32>,
}

impl IDManager {

    pub fn new() -> IDManager {
        IDManager {
            _current_id: __MAX_INSTR_INT__,
            existing_ids: HashSet::new()
        }
    }

    fn current_id(&mut self) -> u32 {
        self._current_id += 0x1;
        while self.existing_ids.contains(&self._current_id) {
            self._current_id += 0x1;
        }
        return self._current_id
    }
    
    fn add_id(&mut self, id:u32) -> Result<(), String> {
        if self.existing_ids.contains(&id) {
            return Err(format!("The dynamic id {} was instantiated twice.", id))
        }
        self.existing_ids.insert(id);
        Ok(())
    }

    fn remove_id(&mut self, id:u32) -> Result<(), String> {
        if !self.existing_ids.contains(&id) {
            return Err(format!("The dynamic id {} was deleted twice.", id))
        }
        self.existing_ids.remove(&id);
        Ok(())
    }
}

// BYTECODE BUILDER

pub struct BytecodeBuilder {
    id_manager:Rc<RefCell<IDManager>>,
    pub src:Box<ByteCode>
}

impl BytecodeBuilder {
    pub fn new() -> BytecodeBuilder {
        let id_manager = Rc::new(RefCell::new(IDManager::new()));
        return BytecodeBuilder {
            id_manager: id_manager.clone(),
            src: Box::new(ByteCode::new(id_manager.clone()))
        };
    }
    
    fn conv_vec_bt_num(vector:Vec<u32>) -> Vec<ByteType> {
        let mut new_vec = vec![];
        for bt in vector {
            new_vec.push(ByteType::Num(bt));
        }
        return new_vec;
    }

    pub fn write_eq(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![EQ, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_gt(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![GT, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_lt(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![LT, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_gte(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![GTE, cid, lhs, rhs, ENDL]));
        return cid;
    }
    
    pub fn write_lte(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![LTE, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_neq(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![NEQ, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_alloca(&mut self, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![ALLOCA, cid, ENDL]));
        return cid;
    }

    pub fn write_store(&mut self, id:u32, value:u32) -> u32 {

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![STORE, id, value, ENDL]));
        return id;
    }

    pub fn write_del(&mut self, id:u32) -> u32 {

        RefCell::borrow_mut(&self.id_manager).remove_id(id);

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![DEL, id, ENDL]));
        return id;
    }

    pub fn write_add(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![ADD, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_sub(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![SUB, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_mul(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![MUL, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_div(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![DIV, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_exp(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![EXP, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_mod(&mut self, lhs:u32, rhs:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![MOD, cid, lhs, rhs, ENDL]));
        return cid;
    }

    pub fn write_jump(&mut self, block:u32) -> u32 {
        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![JUMP, block]));
        return block;
    }

    pub fn write_cond_jump(&mut self, block:u32, cond:u32) -> u32 {
        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![COND_JUMP, block, cond, ENDL]));
        return block;
    }

    pub fn write_block(&mut self, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![BLOCK, cid, ENDL]));
        return cid;
    }

    pub fn write_begin_scope(&mut self) -> () {
        self.src.as_mut().append(ByteType::Num(BEGIN_SCOPE));
    }

    pub fn write_end_scope(&mut self) -> () {
        self.src.as_mut().append(ByteType::Num(END_SCOPE));
    }

    pub fn write_start(&mut self) -> () {
        self.src.as_mut().append(ByteType::Num(START));
    }

    pub fn write_num(&mut self, _num:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        let mut num = vec![];
        for i in _num.to_string().chars() {
            num.push(i.to_digit(10).unwrap());
        }

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![vec![NUM, cid].as_slice(), num.as_slice(), vec![ENDL].as_slice()].concat()));
        return cid;
    }

    pub fn write_cast_num(&mut self, id:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![CAST_NUM, cid, id, ENDL]));
        return cid;
    }

    pub fn write_cast_str(&mut self, id:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![CAST_STR, cid, id, ENDL]));
        return cid;
    }

    pub fn write_str(&mut self, string:String, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(vec![ByteType::Num(STR), ByteType::Num(cid), ByteType::Str(string), ByteType::Num(ENDL)]);
        return cid;
    }

    pub fn write_fmt(&mut self, string:u32, items:Vec<u32>, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![vec![FMT, cid, string].as_slice(), items.as_slice(), vec![ENDL].as_slice()].concat()));
        return cid;
    }

    pub fn write_fmt_num(&mut self, num:u32, precision:u32, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![FMT_NUM, cid, num, precision, ENDL]));
        return cid;
    }

    pub fn write_stdout(&mut self, out:u32) -> () {
        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![STDOUT, out, ENDL]));
    }

    pub fn write_stdin(&mut self, _cid:Option<u32>) -> u32 {
        let cid = match _cid {
            Some(__cid) => __cid,
            None => RefCell::borrow_mut(&self.id_manager).current_id(),
        };

        self.src.as_mut().extend(Self::conv_vec_bt_num(vec![STDIN, cid, ENDL]));
        return cid;
    }

}
