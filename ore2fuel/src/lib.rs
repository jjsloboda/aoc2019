use std::collections::HashMap;
use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct ChemQty {
    name: String,
    qty: u32,
}
impl ChemQty {
    pub fn new(name: &str, qty: u32) -> Self {
        ChemQty{ name: name.to_string(), qty: qty, }
    }
    pub fn from(raw: &str) -> Self {
        let mut itr = raw.trim().splitn(2, ' ');
        let qty = itr.next().expect("input fail").parse::<u32>().expect("parse fail");
        let name = itr.next().expect("input fail");
        ChemQty::new(name, qty)
    }
}

#[derive(Clone)]
pub struct Rxn {
    product: ChemQty,
    reactants: Vec<ChemQty>,
}

fn parse_rxn(raw_rxn: &str) -> Rxn {
    let mut raw_rxn_iter = raw_rxn.trim().splitn(2, "=>");
    let raw_reactants = raw_rxn_iter.next().expect("bad line format");
    let raw_product = raw_rxn_iter.next().expect("bad line format");

    let product = ChemQty::from(raw_product);
    let reactants = raw_reactants.trim().split(',')
        .map(|rr| ChemQty::from(rr))
        .collect();

    Rxn{ product: product, reactants: reactants, }
}

pub fn load_rxns<R: BufRead>(rxn_lines: R) -> Vec<Rxn> {
    rxn_lines.lines()
        .map(|line| parse_rxn(&line.expect("bad input")))
        .collect()
}

fn make_rxn_map(rxns: &Vec<Rxn>) -> HashMap<String, Rxn> {
    let mut rm = HashMap::new();
    for r in rxns {
        rm.insert(r.product.name.clone(), r.clone());
    }
    rm
}

fn calculate_chem_degree(chem: &String, rxn_map: &HashMap<String, Rxn>) -> u32 {
    if *chem == "ORE" {
        0
    } else {
        1 + rxn_map.get(chem).expect("no rxn found")
            .reactants.iter()
            .map(|chqty| calculate_chem_degree(&chqty.name, rxn_map))
            .max().expect("no max")
    }
}

fn make_rxn_order(rxn_map: &HashMap<String, Rxn>) -> Vec<String> {
    let mut chems_w_degree = rxn_map.keys()
        .map(|k| (k.clone(), calculate_chem_degree(k, rxn_map)))
        .collect::<Vec<_>>();
    chems_w_degree.sort_by(|a, b| a.1.cmp(&b.1));
    chems_w_degree.iter().map(|(n, _)| n.clone()).collect()
}

pub fn min_ore_qty_for_fuel(rxns: &Vec<Rxn>) -> u32 {
    const ORE: &str = "ORE";
    const FUEL: &str = "FUEL";

    let rxn_map = make_rxn_map(rxns);

    let mut rxn_order = make_rxn_order(&rxn_map);
    let mut chem_map = HashMap::new();
    chem_map.insert(String::from(FUEL), 1);
    while !rxn_order.is_empty() {
        if let Some(chemname) = rxn_order.pop() {
            if chemname != ORE {
                let rxn = rxn_map.get(&chemname).expect("no rxn found");
                let chemqty = *chem_map.get(&chemname).unwrap_or(&0);
                let qty_multiplier =
                    (chemqty as f32 / rxn.product.qty as f32).ceil() as u32;
                *chem_map.get_mut(&chemname).expect("no chem fnd") = 0;
                for ch in &rxn_map.get(&chemname).expect("no rxn found").reactants {
                    *chem_map.entry(ch.name.clone()).or_insert(0) += ch.qty * qty_multiplier;
                }
            }
        }
    }
    *chem_map.get(ORE).expect("no ore")
}

#[cfg(test)]
mod tests {
    use super::{ChemQty, Rxn};

    #[test]
    fn check_parse_rxn() {
        use super::parse_rxn;

        let rxn1 = parse_rxn("9 ORE => 2 A\n");
        assert_eq!("A", rxn1.product.name);
        assert_eq!(2, rxn1.product.qty);
        assert_eq!(1, rxn1.reactants.len());
        assert_eq!("ORE", rxn1.reactants[0].name);
        assert_eq!(9, rxn1.reactants[0].qty);

        let rxn2 = parse_rxn("2 AB, 3 BC, 4 CA => 1 FUEL\n");
        assert_eq!("FUEL", rxn2.product.name);
        assert_eq!(1, rxn2.product.qty);
        assert_eq!(3, rxn2.reactants.len());
        assert_eq!("AB", rxn2.reactants[0].name);
        assert_eq!(2, rxn2.reactants[0].qty);
        assert_eq!("BC", rxn2.reactants[1].name);
        assert_eq!(3, rxn2.reactants[1].qty);
        assert_eq!("CA", rxn2.reactants[2].name);
        assert_eq!(4, rxn2.reactants[2].qty);

        let rxn3 = parse_rxn("7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL");
        assert_eq!("PLWSL", rxn3.product.name);
        assert_eq!(4, rxn3.product.qty);
        assert_eq!(7, rxn3.reactants.len());
    }

    fn cq(s: &str, q: u32) -> ChemQty {
        ChemQty::new(s, q)
    }
    fn rn(p: ChemQty, r: Vec<ChemQty>) -> Rxn {
        Rxn{ product: p, reactants: r, }
    }

    #[test]
    fn min_ore_for_fuel_ex_1() {
        use super::min_ore_qty_for_fuel;
        let rxns = vec![
            rn(cq("A", 2), vec![cq("ORE", 9)]),
            rn(cq("B", 3), vec![cq("ORE", 8)]),
            rn(cq("C", 5), vec![cq("ORE", 7)]),
            rn(cq("AB", 1), vec![cq("A", 3), cq("B", 4)]),
            rn(cq("BC", 1), vec![cq("B", 5), cq("C", 7)]),
            rn(cq("CA", 1), vec![cq("C", 4), cq("A", 1)]),
            rn(cq("FUEL", 1), vec![cq("AB", 2), cq("BC", 3), cq("CA", 4)]),
        ];
        assert_eq!(165, min_ore_qty_for_fuel(&rxns));
    }

    #[test]
    fn min_ore_for_fuel_ex_2() {
        use super::min_ore_qty_for_fuel;
        let rxns = vec![
            rn(cq("NZVS", 5), vec![cq("ORE", 157)]),
            rn(cq("DCFZ", 6), vec![cq("ORE", 165)]),
            rn(cq("FUEL", 1), vec![
                cq("XJWVT", 44), cq("KHKGT", 5), cq("QDVJ", 1),
                cq("NZVS", 29), cq("GPVTF", 9), cq("HKGWZ", 48),
            ]),
            rn(cq("QDVJ", 9), vec![
                cq("HKGWZ", 12), cq("GPVTF", 1), cq("PSHF", 8),
            ]),
            rn(cq("PSHF", 7), vec![cq("ORE", 179)]),
            rn(cq("HKGWZ", 5), vec![cq("ORE", 177)]),
            rn(cq("XJWVT", 2), vec![
                cq("DCFZ", 7), cq("PSHF", 7),
            ]),
            rn(cq("GPVTF", 2), vec![cq("ORE", 165)]),
            rn(cq("KHKGT", 8), vec![
                cq("DCFZ", 3), cq("NZVS", 7), cq("HKGWZ", 5), cq("PSHF", 10),
            ]),
        ];
        assert_eq!(13312, min_ore_qty_for_fuel(&rxns));
    }

    #[test]
    fn check_overall_ex_3() {
        use super::{load_rxns, min_ore_qty_for_fuel};
        const EX3: &[u8] =
            b"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
              17 NVRVD, 3 JNWZP => 8 VPVL\n\
              53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
              22 VJHF, 37 MNCFX => 5 FWMGM\n\
              139 ORE => 4 NVRVD\n\
              144 ORE => 7 JNWZP\n\
              5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
              5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
              145 ORE => 6 MNCFX\n\
              1 NVRVD => 8 CXFTF\n\
              1 VJHF, 6 MNCFX => 4 RFSQX\n\
              176 ORE => 6 VJHF" as &[u8];
        assert_eq!(180697, min_ore_qty_for_fuel(&load_rxns(EX3)));
    }

    #[test]
    fn check_overall_ex_4() {
        use super::{load_rxns, min_ore_qty_for_fuel};
        const EX4: &[u8] =
            b"171 ORE => 8 CNZTR\n\
              7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
              114 ORE => 4 BHXH\n\
              14 VRPVC => 6 BMBT\n\
              6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
              6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
              15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
              13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
              5 BMBT => 4 WPTQ\n\
              189 ORE => 9 KTJDG\n\
              1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
              12 VRPVC, 27 CNZTR => 2 XDBXC\n\
              15 KTJDG, 12 BHXH => 5 XCVML\n\
              3 BHXH, 2 VRPVC => 7 MZWV\n\
              121 ORE => 7 VRPVC\n\
              7 XCVML => 6 RJRHP\n\
              5 BHXH, 4 VRPVC => 5 LTCX" as &[u8];
        assert_eq!(2210736, min_ore_qty_for_fuel(&load_rxns(EX4)));
    }
}
