use crate::error::problem::ProblemValidationError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContestCode(String);

impl ContestCode {
    pub fn try_new(value: String) -> Result<Self, ProblemValidationError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(ProblemValidationError::BlankContestCode);
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
    use super::ContestCode;
    use crate::error::problem::ProblemValidationError;

    #[test]
    fn normalizes_valid_code() {
        assert_eq!(
            ContestCode::try_new(" abc300 ".to_string())
                .expect("valid")
                .as_str(),
            "abc300"
        );
    }

    #[test]
    fn rejects_blank_code() {
        assert_eq!(
            ContestCode::try_new("   ".to_string()),
            Err(ProblemValidationError::BlankContestCode)
        );
    }
}
