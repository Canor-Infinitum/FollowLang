use crate::ast::{Sigil, Invocation};

#[derive(Debug, Clone, PartialEq)]
pub enum Domain {
    Flow,
    Action,
    Maneuvers,
    United,
    Elite,
    Follower,
    Ether,
    Core,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticNode {
    pub domain: Domain,
    pub sigil: Option<Sigil>,
    pub identifier: String,
    pub invocations: Vec<Invocation>,
    pub templates: usize,
    pub definition_kind: Option<String>,
    pub source_repr: String,
}

impl SemanticNode {
    pub fn classify(mut self) -> Self {
        self.domain = match (self.sigil.as_ref(), self.identifier.as_str()) {
            (Some(crate::ast::Sigil::Type), "flow") => Domain::Flow,
            (Some(crate::ast::Sigil::Type), "action") => Domain::Action,
            (Some(crate::ast::Sigil::Type), "maneuver")
            | (Some(crate::ast::Sigil::Type), "maneuvers") => Domain::Maneuvers,
            (Some(crate::ast::Sigil::Type), "united") => Domain::United,
            (Some(crate::ast::Sigil::Type), "elite") => Domain::Elite,
            (Some(crate::ast::Sigil::Type), "follower") => Domain::Follower,
            (Some(crate::ast::Sigil::Type), "ether") => Domain::Ether,

            // Suite feature names
            (Some(crate::ast::Sigil::Type), "r_s")
            | (Some(crate::ast::Sigil::Type), "l_p")
            | (Some(crate::ast::Sigil::Type), "flow_scale") => Domain::Flow,

            (Some(crate::ast::Sigil::Type), "C_N1")
            | (Some(crate::ast::Sigil::Type), "C_N2")
            | (Some(crate::ast::Sigil::Type), "action_symbol") => Domain::Action,

            (Some(crate::ast::Sigil::Type), "delta_a")
            | (Some(crate::ast::Sigil::Type), "maneuver_group") => Domain::Maneuvers,

            (Some(crate::ast::Sigil::Type), "delta_g")
            | (Some(crate::ast::Sigil::Type), "united_schema") => Domain::United,

            (Some(crate::ast::Sigil::Type), "r_f")
            | (Some(crate::ast::Sigil::Type), "elite_schema") => Domain::Elite,

            (Some(crate::ast::Sigil::Type), "metric")
            | (Some(crate::ast::Sigil::Type), "header")
            | (Some(crate::ast::Sigil::Type), "fwp")
            | (Some(crate::ast::Sigil::Type), "fwh")
            | (Some(crate::ast::Sigil::Type), "fws")
            | (Some(crate::ast::Sigil::Type), "fwb") => Domain::Follower,

            (Some(crate::ast::Sigil::Type), "eps")
            | (Some(crate::ast::Sigil::Type), "epsilon") => Domain::Ether,

            (Some(crate::ast::Sigil::Rule), _) => Domain::Core,
            (Some(crate::ast::Sigil::Function), _) => Domain::Core,
            (Some(crate::ast::Sigil::Variable), _) => Domain::Core,

            _ => Domain::Core,
        };

        self
    }
}
