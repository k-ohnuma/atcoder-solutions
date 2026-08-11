use crate::error::user::UserValidationError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn try_new(value: String) -> Result<Self, UserValidationError> {
        if value.trim().is_empty() {
            return Err(UserValidationError::BlankId);
        }
        Ok(Self(value))
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
    use crate::error::user::UserValidationError;

    use super::UserId;

    #[test]
    fn preserves_valid_id() {
        let id = UserId::try_new("firebase-uid".to_string()).expect("valid user ID");

        assert_eq!(id.as_str(), "firebase-uid");
    }

    #[test]
    fn rejects_blank_id() {
        assert_eq!(
            UserId::try_new("  ".to_string()),
            Err(UserValidationError::BlankId)
        );
    }
}
