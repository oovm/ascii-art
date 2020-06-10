#[derive(Debug, Clone)]
pub enum Error {}

pub type LowPolyResult<T> = std::result::Result<T, Error>;
