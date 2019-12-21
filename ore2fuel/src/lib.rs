use std::collections::BTreeMap;

#[derive(Eq, PartialEq)]
struct ChemSet {
    chems: BTreeMap<String, i32>,
}
impl ChemSet {
    pub fn new(chems: BTreeMap<String, i32>) -> Self {
        ChemSet{ chems: chems, }
    }
    pub fn can_react(&self, rxn: &Rxn) -> bool {
        for (chem, amt) in rxn.inputs.chems.iter() {
            let self_amt = match self.chems.get(chem) {
                Some(x) => *x,
                None => 0,
            };
            if *amt > self_amt {
                return false;
            }
        }
        true
    }
    pub fn react(&mut self, rxn: &Rxn) {
        if !self.can_react(rxn) {
            panic!("cannot react");
        }
        for (chem, amt) in rxn.inputs.chems.iter() {
            if let Some(self_amt) = self.chems.get_mut(chem) {
                *self_amt -= *amt;
            }
        }
        for (chem, amt) in rxn.outputs.chems.iter() {
            *self.chems.entry(chem.to_string()).or_insert(0) += *amt;
        }
    }
    pub fn to_string(&self) -> String {
        // TODO
        //self.chems.iter().map(|(chem, amt)| 
    }
}

struct Rxn {
    inputs: ChemSet,
    outputs: ChemSet,
}
impl Rxn {
    pub fn from_string(s: &str) -> Self {
        let mut inputs = BTreeMap::new();
        let mut outputs = BTreeMap::new();
        // TODO split and parse string into components
        // load components into hashmap
        // return rxn with hashmaps constructed
        Rxn{ inputs: ChemSet::new(inputs), outputs: ChemSet::new(outputs), }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
