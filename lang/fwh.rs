// follow-header/src/fwh.rs

#[derive(Debug, Clone, PartialEq)]
pub struct FollowHeaderSpec {
    pub primitives: PrimitiveTypes,
    pub units: UnitTypes,
    pub tensor: TensorTypes,
    pub markov: MarkovTypes,
}