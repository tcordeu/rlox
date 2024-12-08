use crate::literal::Literal;
use crate::token::Token;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Option<Rc<dyn Literal>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Rc<dyn Literal>>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Option<&Option<Rc<dyn Literal>>> {
        if self.values.contains_key(name.lexeme()) {
            return Some(self.values.get(name.lexeme()).unwrap());
        }

        None
    }

    pub fn assign(&mut self, name: Token, value: Option<Rc<dyn Literal>>) -> Option<()> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);

            return Some(());
        }

        None
    }
}
