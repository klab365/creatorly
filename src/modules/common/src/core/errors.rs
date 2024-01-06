use std::fmt::Formatter;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    description: String,
    advice: Option<String>,
}

impl Error {
    pub fn new(description: String) -> Self {
        Self {
            description,
            advice: None,
        }
    }

    pub fn with_advice(description: String, advice: String) -> Self {
        Self {
            description,
            advice: Some(advice),
        }
    }
}

impl From<&str> for Error {
    fn from(description: &str) -> Self {
        Self::new(description.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(advice) = &self.advice {
            write!(f, "{} -> {}", self.description, advice)
        } else {
            write!(f, "{}", self.description)
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let error = Error::new("description".to_string());
        assert_eq!(error.to_string(), "description".to_string());

        let error = Error::with_advice("description".to_string(), "advice".to_string());
        assert_eq!(error.to_string(), "description -> advice".to_string());
    }
}
