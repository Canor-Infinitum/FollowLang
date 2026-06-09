#[derive(Debug, Clone, PartialEq)]
pub struct MarkovTypes {
    pub markov: Markov,
    pub chain: Chain,
    pub generator: Generator,
    pub process: Process,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Markov {
    pub state_space: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chain {
    pub transitions: Vec<(String, String, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Generator {
    pub rates: Vec<(String, String, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub name: String,
    pub chain: Chain,
}