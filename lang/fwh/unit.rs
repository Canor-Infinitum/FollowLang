#[derive(Debug, Clone, PartialEq)]
pub struct UnitTypes {
    pub unit: Unit,
    pub unity: Unity,
    pub uniter: Uniter,
    pub unital: Unital,

    pub point: Point,
    pub axis: Axis,
    pub composition: Composition,
    pub unbeknownst: Unbeknownst,
}

#[derive(Debug, Clone, PartialEq)] pub struct Unit { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Unity { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Uniter { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Unital { pub name: String }

#[derive(Debug, Clone, PartialEq)] pub struct Point { pub dim: usize }
#[derive(Debug, Clone, PartialEq)] pub struct Axis { pub dim: usize }
#[derive(Debug, Clone, PartialEq)] pub struct Composition { pub elements: Vec<String> }
#[derive(Debug, Clone, PartialEq)] pub struct Unbeknownst { pub description: String }