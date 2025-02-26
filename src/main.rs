mod circuit;
mod gates;

use circuit::Circuit;
use gates::{And, Gates, Not, Or, GateId, Gate};


#[derive(Debug)]
pub enum Input {
    Bool(bool),
    Gate(GateId),
}
impl Input {
    pub fn get(&self, circuit:&Circuit) -> bool {
        match self {
            Input::Bool(b) => *b,
            Input::Gate(id) => {
                let gate = circuit.get_gate(id).unwrap();
                match gate {
                    Gates::And(and) => and.calculate(circuit),
                    Gates::Or(or) => or.calculate(circuit),
                    Gates::Not(not) => not.calculate(circuit),
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut circuit = Circuit::new();
    let inputA = Input::Bool(true);
    let inputB = Input::Bool(false);
    let inputC = Input::Bool(false);
    let or = Or::new(inputA, inputB);
    let not = Not::new(inputC);

    let IDor = circuit.add_gate(Gates::Or(or));
    let IDnot = circuit.add_gate(Gates::Not(not));

    let and = And::new(Input::Gate(IDor), Input::Gate(IDnot));

    let IDand = circuit.add_gate(Gates::And(and));



    circuit.run();

    
}
