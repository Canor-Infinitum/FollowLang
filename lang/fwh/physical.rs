#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalTypes {
    pub atom: Atom,
    pub change: Change,
    pub force: Force,
}

#[derive(Debug, Clone, PartialEq)] pub struct Atom { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Change { pub rate: Option<f64> }
#[derive(Debug, Clone, PartialEq)] pub struct Force { pub magnitude: Option<f64> }