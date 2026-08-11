use std::cmp::Ordering;

use crate::error::problem::ProblemValidationError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProblemIndex(String);

impl ProblemIndex {
    pub fn try_new(value: String) -> Result<Self, ProblemValidationError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(ProblemValidationError::BlankIndex);
        }
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn cmp_for_display(&self, other: &Self) -> Ordering {
        let own_index = self.0.to_lowercase();
        let other_index = other.0.to_lowercase();

        match (own_index.as_str(), other_index.as_str()) {
            ("ex", "ex") => Ordering::Equal,
            ("ex", _) => Ordering::Greater,
            (_, "ex") => Ordering::Less,
            _ => own_index.cmp(&other_index),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::ProblemIndex;
    use crate::error::problem::ProblemValidationError;

    #[test]
    fn normalizes_valid_index() {
        assert_eq!(
            ProblemIndex::try_new(" A ".to_string())
                .expect("valid")
                .as_str(),
            "A"
        );
    }

    #[test]
    fn rejects_blank_index() {
        assert_eq!(
            ProblemIndex::try_new("   ".to_string()),
            Err(ProblemValidationError::BlankIndex)
        );
    }

    #[test]
    fn sorts_ex_after_other_indexes() {
        let a = ProblemIndex::try_new("A".to_string()).expect("valid");
        let ex = ProblemIndex::try_new("Ex".to_string()).expect("valid");

        assert_eq!(a.cmp_for_display(&ex), Ordering::Less);
        assert_eq!(ex.cmp_for_display(&a), Ordering::Greater);
    }
}
