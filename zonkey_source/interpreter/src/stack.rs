#[derive(Debug, Clone)]
pub struct Stack {
    pub integer: usize,
    pub float: usize,
    pub string: usize,
    pub boolean: usize,
    pub object: usize,
}
