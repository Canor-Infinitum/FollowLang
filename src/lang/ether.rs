use follow_core::core::formula::Formula;

#[derive(Debug, Clone, PartialEq)]
pub enum EtherNode {
    Symbol(String),
    Constant(String),
    Invoke {
        target: String,
        method: String,
        args: Vec<String>,
    },
    Compose {
        lhs: Box<EtherNode>,
        op: String,
        rhs: Box<EtherNode>,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EtherIR {
    pub nodes: Vec<EtherNode>,
    pub libraries: Vec<String>,
}

impl EtherIR {
    pub fn push_node(&mut self, node: EtherNode) {
        self.nodes.push(node);
    }

    pub fn link<S: Into<String>>(&mut self, lib: S) {
        self.libraries.push(lib.into());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpsilonSpec {
    pub eps: Formula,
    pub observed: bool,
    pub located: bool,
    pub tracked: bool,
    pub mapped: bool,
    pub error_checked: bool,
    pub mitigated: bool,
}

impl EpsilonSpec {
    pub fn default_spec() -> Self {
        Self {
            eps: Formula::Text("eps := upd_rate / err_rate".into()),
            observed: true,
            located: true,
            tracked: true,
            mapped: true,
            error_checked: true,
            mitigated: false,
        }
    }
}

