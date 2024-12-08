use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::literal::Literal;
use crate::token::Token;
use std::rc::Rc;

pub struct Scope {
    envs: Vec<Environment>,
}

impl Scope {
    pub fn new() -> Scope {
        let mut s = Scope { envs: Vec::new() };
        s.envs.insert(0, Environment::new());

        s
    }

    pub fn wrap(&mut self) {
        self.envs.insert(0, Environment::new());
    }

    pub fn unwrap(&mut self) {
        self.envs.remove(0);
    }

    pub fn define(&mut self, name: String, value: Option<Rc<dyn Literal>>) {
        self.envs.first_mut().unwrap().define(name, value);
    }

    pub fn get(&self, name: Token) -> Result<&Option<Rc<dyn Literal>>, RuntimeError> {
        for e in self.envs.iter() {
            if let Some(val) = e.get(name.clone()) {
                return Ok(val);
            }
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
        for e in self.envs.iter_mut() {
            if e.assign(name.clone(), value.clone()).is_some() {
                return Ok(());
            }
        }

        Err(RuntimeError::new(
            format!("Undefined var '{}'", name.lexeme()),
            name,
        ))
    }
}
