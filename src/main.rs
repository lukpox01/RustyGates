mod circuit;
mod gates;

use circuit::Circuit;
use gates::{And, Gate, GateId, Gates, Not, Or, Output};

#[derive(Debug, Clone)]
pub enum Input {
    Bool(bool),
    Gate(GateId),
}
impl Input {
    pub fn get(&self, circuit: &mut Circuit) -> bool {
        match self {
            Input::Bool(b) => *b,
            Input::Gate(id) => {
                if let Some(cache) = circuit.clone().get_cached(id) {
                    println!(
                        "Cache hit {} for {} {:?}",
                        cache,
                        circuit.get_gate_name(id).unwrap(),
                        id
                    );
                    return cache;
                }
                let tmp = circuit.clone();
                let gate = tmp.get_gate(id).unwrap();
                let res = match gate {
                    Gates::And(and) => and.calculate(circuit),
                    Gates::Or(or) => or.calculate(circuit),
                    Gates::Not(not) => not.calculate(circuit),
                    Gates::Output(output) => output.calculate(circuit),
                };
                circuit.insert_cache(id.clone(), res);
                res
            }
        }
    }
}

fn main() {
    let mut circuit = Circuit::new();
    let inputA = Input::Bool(true);
    let inputB = Input::Bool(false);
    let inputC = Input::Bool(false);
    let or = Or::new(inputA, inputB, None);
    let not = Not::new(inputC, None);

    let IDor = circuit.add_gate(Gates::Or(or));
    let IDnot = circuit.add_gate(Gates::Not(not));

    let and = And::new(Input::Gate(IDor), Input::Gate(IDnot), None);

    let IDand = circuit.add_gate(Gates::And(and));

    let output1 = Output::new(Input::Gate(IDand), Some("Out1".to_string()));

    let IDoutput1 = circuit.add_gate(Gates::Output(output1));
    circuit.print_all_gates();
    circuit.run();
    circuit.print_output();
}
