use uuid::Uuid;
use crate::{circuit::{self, Circuit}, Input};
#[derive(Debug)]
pub enum Gates {
    And(And),
    Or(Or),
    Not(Not),
}



#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct GateId(pub Uuid);
impl GateId {
    pub fn new() -> Self {
        GateId(Uuid::new_v4())
    }
}

pub trait Gate {
    fn calculate(&self, circuit: &Circuit) -> bool;
}
#[derive(Debug)]
pub struct And {
    inputs: [Input; 2]
}
impl Gate for And {
    fn calculate(&self, circuit: &Circuit) -> bool {
        self.inputs[0].get(circuit) && self.inputs[1].get(circuit)
    }
}

impl And {
    pub fn new(a: Input, b: Input) -> Self {
        And {
            inputs: [a, b],
        }
    }
    
}
#[derive(Debug)]
pub struct Or {
    inputs: [Input; 2],
}
impl Gate for Or {
    fn calculate(&self, circuit: &Circuit) -> bool {
        self.inputs[0].get(circuit) || self.inputs[1].get(circuit)
    }
}

impl Or {
    pub fn new(a: Input, b: Input) -> Self {
        Or {
            inputs: [a, b],
        }
    }
}
#[derive(Debug)]
pub struct Not {
    inputs: [Input; 1],
}
impl Gate for Not {
    fn calculate(&self, circuit: &Circuit) -> bool {
        !self.inputs[0].get(circuit)
    }
}
impl Not {
    pub fn new(a: Input) -> Self {
        Not {
            inputs: [a],
        }
    }
}
