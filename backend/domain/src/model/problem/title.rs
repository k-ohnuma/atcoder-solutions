use crate::error::problem::ProblemValidationError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemTitle(String);

impl ProblemTitle {
    pub fn try_new(value: String) -> Result<Self, ProblemValidationError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(ProblemValidationError::BlankTitle);
        }
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ProblemTitle;
    use crate::error::problem::ProblemValidationError;

    #[test]
    fn normalizes_valid_title() {
        assert_eq!(
            ProblemTitle::try_new(" A - Example ".to_string())
                .expect("valid")
                .as_str(),
            "A - Example"
        );
    }

    #[test]
    fn rejects_blank_title() {
        assert_eq!(
            ProblemTitle::try_new("   ".to_string()),
            Err(ProblemValidationError::BlankTitle)
        );
    }
}
