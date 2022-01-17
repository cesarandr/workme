use std::fmt;
// Body Sections
#[derive(Debug, Clone)]
pub enum BodySection {
    Upper,
    Core,
    Lower,
    Full
}

impl fmt::Display for BodySection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}