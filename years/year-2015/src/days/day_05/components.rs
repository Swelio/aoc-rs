#[derive(Debug, Clone, derive_more::AsRef, derive_more::Constructor)]
#[as_ref(str)]
pub struct Line(String);
