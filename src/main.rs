use itertools;
use std::ops;
use std::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimension (i32, i32, i32, i32, i32);

impl Dimension {
    fn new() -> Dimension {
        Dimension(0, 0, 0, 0, 0)
    }
}

impl ops::Add<Dimension> for Dimension {
    type Output = Dimension;
    fn add(self, rhs: Dimension) -> Dimension {
        Dimension(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
            self.4 + rhs.4,
        )
    }
}

impl ops::AddAssign<Dimension> for Dimension {
    fn add_assign(&mut self, rhs: Dimension) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
        self.4 += rhs.4;
    }
}

impl ops::Mul<i32> for Dimension {
    type Output = Dimension;
    fn mul(self, rhs: i32) -> Dimension {
        Dimension(
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs,
            self.3 * rhs,
            self.4 * rhs,
        )
    }
}

#[derive(Debug)]
enum Unit<'u> {
    Base(BaseUnit),
    Compound(CompoundUnit<'u>),
}

#[derive(Debug)]
struct BaseUnit {
    dimension: Dimension,
    name: String,
    scalar: f32,
}

#[derive(Debug)]
struct CompoundUnit<'u> {
    dimension: Dimension,
    units: Vec<(&'u Unit<'u>, i32)>,
    name: String,
    scalar: f32,
}

impl Unit<'_> {
    fn get_dimension(&self) -> Dimension {
        match self {
            Unit::Base(unit) => unit.dimension,
            Unit::Compound(unit) => unit.dimension,
        }
    }

    fn get_scalar(&self) -> f32 {
        match self {
            Unit::Base(unit) => unit.scalar,
            Unit::Compound(unit) => unit.scalar,
        }
    }

    fn get_name(&self) -> &String {
        match self {
            Unit::Base(unit) => &unit.name,
            Unit::Compound(unit) => &unit.name,
        }
    }
}

impl<'u> CompoundUnit<'u> {
    fn new(units: Vec<(&'u Unit<'u>, i32)>, name: Option<String>) -> CompoundUnit {
        let mut dimension = Dimension::new();
        let mut scalar: f32 = 1.0;
        for (unit, exp) in &units {
            dimension += unit.get_dimension() * (*exp);
            scalar *= unit.get_scalar().powf(*exp as f32);
        }
        let name = match name {
            Some(n) => n,
            None => {
                let strs = (&units).into_iter().map(
                    |(unit, exp)| {
                        if *exp == 1 {
                            unit.get_name().clone()
                        } else if *exp != 0 {
                            format!("{}^{}", unit.get_name(), exp)
                        } else {
                            String::from("")
                        }
                    });
                itertools::join(strs, " * ")
            }
        };
        CompoundUnit {
            dimension: dimension,
            units: units,
            name: name,
            scalar: scalar,
        }
    }
}

fn main() {
    let meter = Unit::Base(BaseUnit {
        dimension: Dimension(1, 0, 0, 0, 0),
        name: String::from("m"),
        scalar: 1.0,
    });
    let second = Unit::Base(BaseUnit {
        dimension: Dimension(0, 1, 0, 0, 0),
        name: String::from("s"),
        scalar: 1.0,
    });
    let kilogram = Unit::Base(BaseUnit {
        dimension: Dimension(0, 0, 1, 0, 0),
        name: String::from("kg"),
        scalar: 1.0,
    });
    let ampere = Unit::Base(BaseUnit {
        dimension: Dimension(0, 0, 0, 1, 0),
        name: String::from("A"),
        scalar: 1.0,
    });
    let newton = Unit::Compound(CompoundUnit::new(
        vec![(&meter, 1), (&kilogram, 1), (&second, -2)], None
    ));
    let joule = Unit::Compound(CompoundUnit::new(
        vec![(&newton, 1), (&meter, 1)], None
    ));
    let watt = Unit::Compound(CompoundUnit::new(
        vec![(&joule, 1), (&second, -1)], None
    ));
    let volt = Unit::Compound(CompoundUnit::new(
        vec![(&watt, 1), (&ampere, -1)], None
    ));
    println!("{:?}", volt);
}