use std::any::Any;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum LiteralType {
    False,
    True,
    Number,
    String,
}

pub trait Literal: Any {
    fn ltype(&self) -> LiteralType;
    fn value(&self) -> &dyn Any;
    fn as_any(&self) -> &dyn Any;
}

impl fmt::Display for dyn Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val: String = match self.ltype() {
            LiteralType::False => "false".to_string(),
            LiteralType::True => "true".to_string(),
            LiteralType::Number => {
                let f_val: &f64 = self.value().downcast_ref::<f64>().unwrap();

                format!("{}", f_val)
            }
            LiteralType::String => {
                let s_val: &str = self.value().downcast_ref::<String>().unwrap();

                s_val.to_string()
            }
        };

        write!(f, "{}", val)
    }
}

pub struct StrLiteral {
    ltype: LiteralType,
    value: String,
}

impl StrLiteral {
    pub fn new(value: String) -> StrLiteral {
        StrLiteral {
            ltype: LiteralType::String,
            value,
        }
    }
}

impl Literal for StrLiteral {
    fn ltype(&self) -> LiteralType {
        self.ltype
    }

    fn value(&self) -> &dyn Any {
        &self.value
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NumberLiteral {
    ltype: LiteralType,
    value: f64,
}

impl NumberLiteral {
    pub fn new(value: f64) -> NumberLiteral {
        NumberLiteral {
            ltype: LiteralType::Number,
            value,
        }
    }
}

impl Literal for NumberLiteral {
    fn ltype(&self) -> LiteralType {
        self.ltype
    }

    fn value(&self) -> &dyn Any {
        &self.value
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BoolLiteral {
    ltype: LiteralType,
    value: bool,
}

impl BoolLiteral {
    pub fn new(value: bool) -> BoolLiteral {
        let ltype = if value {
            LiteralType::True
        } else {
            LiteralType::False
        };

        BoolLiteral { ltype, value }
    }
}

impl Literal for BoolLiteral {
    fn ltype(&self) -> LiteralType {
        self.ltype
    }

    fn value(&self) -> &dyn Any {
        &self.value
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
