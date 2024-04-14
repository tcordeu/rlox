use crate::error::RuntimeError;
use crate::literal::Literal;
use crate::token::Token;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Option<Rc<dyn Literal>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Environment {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Rc<dyn Literal>>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<&Option<Rc<dyn Literal>>, RuntimeError> {
        if self.values.contains_key(name.lexeme()) {
            return Ok(self.values.get(name.lexeme()).unwrap());
        }
        if self.enclosing.is_some() {
            let inner_env: &Environment = self.enclosing.as_ref().unwrap();

            return inner_env.get(name);
        }

        Err(RuntimeError::new(
            format!("Undefined var '{}'", name.lexeme()),
            name,
        ))
    }

    pub fn assign(
        &mut self,
        name: Token,
        value: Option<Rc<dyn Literal>>,
    ) -> Result<(), RuntimeError> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);

            return Ok(());
        }
        if self.enclosing.is_some() {
            let inner_env: &mut Environment = self.enclosing.as_mut().unwrap();

            return inner_env.assign(name, value);
        }

        Err(RuntimeError::new(
            format!("Undefined var '{}'", name.lexeme()),
            name,
        ))
    }
}
