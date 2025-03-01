use crate::{
    circuit::{self, Circuit},
    Input,
};
use rand::Rng;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Gates {
    And(And),
    Or(Or),
    Not(Not),
    Output(Output),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct GateId(pub Uuid);
impl GateId {
    pub fn new() -> Self {
        GateId(Uuid::new_v4())
    }
}

pub trait Gate {
    fn calculate(&self, circuit: &mut Circuit) -> bool;
    fn get_name(&self) -> String;
}
#[derive(Debug, Clone)]
pub struct And {
    inputs: [Input; 2],
    name: String,
}
impl Gate for And {
    fn calculate(&self, circuit: &mut Circuit) -> bool {
        println!("And");
        self.inputs[0].get(circuit) && self.inputs[1].get(circuit)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl And {
    pub fn new(a: Input, b: Input, name: Option<String>) -> Self {
        let name = match name {
            Some(name) => name,
            None => format!("And-{}", rand::rng().random::<i16>()),
        };
        And {
            inputs: [a, b],
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Or {
    inputs: [Input; 2],
    name: String,
}
impl Gate for Or {
    fn calculate(&self, circuit: &mut Circuit) -> bool {
        println!("Or");
        self.inputs[0].get(circuit) || self.inputs[1].get(circuit)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Or {
    pub fn new(a: Input, b: Input, name: Option<String>) -> Self {
        let name = match name {
            Some(name) => name,
            None => format!("Or-{}", rand::rng().random::<u16>()),
        };

        Or {
            inputs: [a, b],
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Not {
    inputs: [Input; 1],
    name: String,
}
impl Gate for Not {
    fn calculate(&self, circuit: &mut Circuit) -> bool {
        println!("Not");
        !self.inputs[0].get(circuit)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl Not {
    pub fn new(a: Input, name: Option<String>) -> Self {
        let name = match name {
            Some(name) => name,
            None => format!("Not-{}", rand::rng().random::<i16>()),
        };
        Not { inputs: [a], name }
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    inputs: [Input; 1],
    name: String,
}
impl Gate for Output {
    fn calculate(&self, circuit: &mut Circuit) -> bool {
        println!("Output");
        self.inputs[0].get(circuit)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl Output {
    pub fn new(a: Input, name: Option<String>) -> Self {
        let name = match name {
            Some(name) => name,
            None => format!("Output-{}", rand::rng().random::<i16>()),
        };

        Output { inputs: [a], name }
    }
}
