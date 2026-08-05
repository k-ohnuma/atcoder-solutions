use crate::error::user::UserValidationError;

const MAX_USER_NAME_LENGTH: usize = 120;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserName(String);

impl UserName {
    pub fn try_new(value: String) -> Result<Self, UserValidationError> {
        if value.trim().is_empty() {
            return Err(UserValidationError::BlankName);
        }
        if value.chars().count() > MAX_USER_NAME_LENGTH {
            return Err(UserValidationError::NameTooLong {
                max: MAX_USER_NAME_LENGTH,
            });
        }
        if !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
        {
            return Err(UserValidationError::InvalidNameFormat);
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

    use super::UserName;

    #[test]
    fn accepts_ascii_alphanumeric_and_underscore() {
        let name = UserName::try_new("Alice_123".to_string()).expect("valid user name");

        assert_eq!(name.as_str(), "Alice_123");
    }

    #[test]
    fn rejects_blank_name() {
        assert_eq!(
            UserName::try_new("  ".to_string()),
            Err(UserValidationError::BlankName)
        );
    }

    #[test]
    fn rejects_name_longer_than_120_characters() {
        assert_eq!(
            UserName::try_new("a".repeat(121)),
            Err(UserValidationError::NameTooLong { max: 120 })
        );
    }

    #[test]
    fn rejects_unsupported_characters() {
        assert_eq!(
            UserName::try_new("alice-name".to_string()),
            Err(UserValidationError::InvalidNameFormat)
        );
    }
}
