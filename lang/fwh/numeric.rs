#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveTypes {
    pub number: Number,
    pub arbitrary: Arbitrary,
    pub length: Length,

    pub elementary: Elementary,
    pub composite: Composite,
    pub unique: Unique,
    pub unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq)] pub struct Number { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Arbitrary { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Length { pub name: String }

#[derive(Debug, Clone, PartialEq)] pub struct Elementary { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Composite { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Unique { pub name: String }
#[derive(Debug, Clone, PartialEq)] pub struct Unknown { pub name: String }