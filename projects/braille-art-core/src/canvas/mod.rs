mod draw;
mod save;

#[derive(Debug, Clone)]
pub struct BrailleCanvas {
    pub data: Vec<Vec<char>>,
}
