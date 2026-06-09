use std::collections::BTreeMap;
use crate::{
    action::ActionSymbolSpec,
    elite::EliteSchemaSpec,
    ether::EpsilonSpec,
    flow::FlowScaleSpec,
    follower::FollowProjectSpec,
    maneuvers::ManeuverGroupSpec,
    united::UnitedInterchangeSpec,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SuiteSpec {
    FlowScale(FlowScaleSpec),
    ActionSymbol(ActionSymbolSpec),
    ManeuverGroup(ManeuverGroupSpec),
    UnitedInterchange(UnitedInterchangeSpec),
    EliteSchema(EliteSchemaSpec),
    FollowProject(FollowProjectSpec),
    Epsilon(EpsilonSpec),
}

#[derive(Debug, Clone, Default)]
pub struct SuiteRegistry {
    pub entries: BTreeMap<String, SuiteSpec>,
}

impl SuiteRegistry {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn insert<S: Into<String>>(&mut self, key: S, spec: SuiteSpec) -> Result<(), String> {
        let key = key.into();
        if self.entries.contains_key(&key) {
            return Err(format!("SuiteSpec already defined: {key}"));
        }
        self.entries.insert(key, spec);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&SuiteSpec> {
        self.entries.get(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }
}

