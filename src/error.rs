use std::fmt;

// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Result<T> = std::result::Result<T, JrnlError>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum JrnlErrorKind {
    EmptyEntry,
    InvalidTitleLine,
    MissingJournalConfig,
    TopLevelJournalConfig,
    InvalidJrnlOverrideConfig,
}

impl fmt::Display for JrnlErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::EmptyEntry => "entry is empty",
            Self::InvalidTitleLine => "failed to parse entry title",
            Self::MissingJournalConfig => "no such journal configured",
            Self::TopLevelJournalConfig => "illegal 'journal' key found at top level",
            Self::InvalidJrnlOverrideConfig => {
                "journal-specific config specifies multiple journals"
            }
        };
        write!(f, "{msg}")
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct JrnlError(pub JrnlErrorKind);

impl fmt::Display for JrnlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind())
    }
}

impl JrnlError {
    pub fn kind(&self) -> JrnlErrorKind {
        self.0
    }
}

impl std::error::Error for JrnlError {}
