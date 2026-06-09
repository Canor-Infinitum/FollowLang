use crate::core::formula::Formula;
use crate::semantic::SemanticNode;

use crate::maneuvers::ManeuverGroupSpec;

#[derive(Debug, Clone, PartialEq)]
pub struct UnitedSchema {
    pub name: String,
    pub members: Vec<(String, String)>,
    pub semantic: SemanticNode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnitedInterchangeSpec {
    pub maneuver_group: ManeuverGroupSpec,
    pub runtime_extraction: Formula,
}

impl UnitedInterchangeSpec {
    pub fn new(maneuver_group: ManeuverGroupSpec) -> Self {
        // DELTA_g(f) = (1 / sqrt(g)) * partial_alpha(sqrt(g)) * g^(alpha beta) * partial_beta(f)
        let runtime_extraction = Formula::Text(
            "DELTA_g(f) = (1 / sqrt(g)) * partial_alpha(sqrt(g)) * g^(alpha beta) * partial_beta(f)"
                .into(),
        );

        Self {
            maneuver_group,
            runtime_extraction,
        }
    }
}