use crate::error::problem::ProblemValidationError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProblemId(String);

impl ProblemId {
    pub fn try_new(value: String) -> Result<Self, ProblemValidationError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(ProblemValidationError::BlankId);
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
    use super::ProblemId;
    use crate::error::problem::ProblemValidationError;

    #[test]
    fn normalizes_valid_id() {
        assert_eq!(
            ProblemId::try_new(" abc300_a ".to_string())
                .expect("valid")
                .as_str(),
            "abc300_a"
        );
    }

    #[test]
    fn rejects_blank_id() {
        assert_eq!(
            ProblemId::try_new("   ".to_string()),
            Err(ProblemValidationError::BlankId)
        );
    }
}
