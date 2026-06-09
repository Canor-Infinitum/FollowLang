#[derive(Debug, Clone, PartialEq)]
pub struct TensorTypes {
    pub tensor: Tensor,
    pub tensorial: Tensorial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    pub name: String,
    pub rank: usize,
    pub indices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tensorial {
    pub components: Vec<Tensor>,
}