
pub fn fuel_4_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn fuel_integration(mass: i32) -> i32 {
    let mut total = 0;
    let mut incremental = fuel_4_mass(mass);
    while incremental > 0 {
        total += incremental;
        incremental = fuel_4_mass(incremental);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{fuel_4_mass, fuel_integration};

    #[test]
    fn one_time() {
        assert!(2 == fuel_4_mass(12));
        assert!(2 == fuel_4_mass(14));
        assert!(654 == fuel_4_mass(1969));
        assert!(33583 == fuel_4_mass(100756));
    }

    #[test]
    fn integral() {
        assert!(2 == fuel_integration(14));
        assert!(966 == fuel_integration(1969));
        assert!(50346 == fuel_integration(100756));
    }
}
