use std::fmt;
use std::str::FromStr;

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

impl FromStr for BodySection {

    type Err = ();

    fn from_str(input: &str) -> Result<BodySection, Self::Err> {
        match input {
            "upper"  => Ok(BodySection::Upper),
            "core"  => Ok(BodySection::Core),
            "lower"  => Ok(BodySection::Lower),
            "full" => Ok(BodySection::Full),
            _      => Err(()),
        }
    }
}