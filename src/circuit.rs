use std::collections::HashMap;

use crate::gates::{And, GateId, Gates, Not, Or, Gate};

#[derive(Debug)]
pub struct Circuit {
    gates: HashMap<GateId, Gates>,
}
impl Circuit {
    pub fn new() -> Self {
        Circuit {
            gates: HashMap::new(),
        }
    }
    pub fn add_gate(&mut self, gate: Gates) -> GateId {
        let id = GateId::new();
        self.gates.insert(id.clone(), gate);
        id
    }
    pub fn get_gate(&self, id: &GateId) -> Option<&Gates> {
        self.gates.get(id)
    }

    pub fn run(&self) {
        for i in self.gates.keys() {
            let gate = self.get_gate(i).unwrap();
            let r = match gate {
                Gates::And(and) => and.calculate(self),
                Gates::Or(or) => or.calculate(self),
                Gates::Not(not) => not.calculate(self),
            };
            println!("ID: {}; {:?} = {}", i.0, gate, r);
        }
    }
}
