use std::collections::HashMap;

use crate::gates::{And, Gate, GateId, Gates, Not, Or};

#[derive(Debug, Clone)]
struct Cache {
    cache: HashMap<GateId, bool>,
}
impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new(),
        }
    }
    fn get(&self, id: &GateId) -> Option<&bool> {
        self.cache.get(id)
    }
    fn insert(&mut self, id: GateId, value: bool) {
        self.cache.insert(id, value);
    }
}

#[derive(Debug, Clone)]
pub struct Circuit {
    gates: HashMap<GateId, Gates>,
    cache: Cache,
}
impl Circuit {
    pub fn new() -> Self {
        Circuit {
            gates: HashMap::new(),
            cache: Cache::new(),
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

    pub fn get_gate_name(&self, id: &GateId) -> Option<String> {
        match self.get_gate(id) {
            Some(gate) => match gate {
                Gates::And(and) => Some(and.get_name()),
                Gates::Or(or) => Some(or.get_name()),
                Gates::Not(not) => Some(not.get_name()),
                Gates::Output(output) => Some(output.get_name()),
            },
            None => None,
        }
    }

    pub fn run(&mut self) {
        for i in self.clone().gates.keys() {
            if let Some(cache) = self.clone().get_cached(i) {
                println!(
                    "Cache hit {} for {} {:?}",
                    cache,
                    self.get_gate_name(i).unwrap(),
                    i
                );
            } else {
                let tmp = self.clone();
                let gate = tmp.get_gate(i).unwrap();
                let r = match gate {
                    Gates::And(and) => and.calculate(self),
                    Gates::Or(or) => or.calculate(self),
                    Gates::Not(not) => not.calculate(self),
                    Gates::Output(output) => output.calculate(self),
                };
                self.insert_cache(i.clone(), r);
            }

            println!(
                "{} = {} ID: {}",
                self.get_gate_name(i).unwrap(),
                self.get_cached(i).unwrap(),
                i.0
            );
        }
    }

    pub fn get_cached(&self, id: &GateId) -> Option<bool> {
        self.cache.get(id).cloned()
    }
    pub fn insert_cache(&mut self, id: GateId, value: bool) {
        self.cache.insert(id, value);
    }

    pub fn print_output(&self) {
        println!("------------ Output ------------");
        for i in self.clone().gates.keys() {
            if let Gates::Output(_) = self.get_gate(i).unwrap() {
                if let Some(r) = self.get_cached(i) {
                    println!("{} = {}, ID: {}", self.get_gate_name(i).unwrap(), r, i.0);
                } else {
                    println!("{} = None, ID: {}", self.get_gate_name(i).unwrap(), i.0);
                }
            }
        }
        println!("------------- End --------------");
    }

    pub fn print_all_gates(&self) {
        println!("------------ All Gates ------------");
        for id in self.gates.keys() {
            if let Some(name) = self.get_gate_name(id) {
                println!("Gate: {}, ID: {}", name, id.0);
            }
        }
        println!("------------- End --------------");
    }
}
