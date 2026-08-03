use crate::error::solution::SolutionValidationError;

const MAX_TITLE_LENGTH: usize = 120;
const MAX_BODY_LENGTH: usize = 20_000;
const MAX_SUBMIT_URL_LENGTH: usize = 500;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolutionTitle(String);

impl SolutionTitle {
    pub fn try_new(title: String) -> Result<Self, SolutionValidationError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(SolutionValidationError::BlankTitle);
        }
        if title.chars().count() > MAX_TITLE_LENGTH {
            return Err(SolutionValidationError::TitleTooLong {
                max: MAX_TITLE_LENGTH,
            });
        }
        Ok(Self(title.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolutionBody(String);

impl SolutionBody {
    pub fn try_new(body: String) -> Result<Self, SolutionValidationError> {
        if body.trim().is_empty() {
            return Err(SolutionValidationError::BlankBody);
        }
        if body.chars().count() > MAX_BODY_LENGTH {
            return Err(SolutionValidationError::BodyTooLong {
                max: MAX_BODY_LENGTH,
            });
        }
        Ok(Self(body))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubmitUrl(String);

impl SubmitUrl {
    pub fn try_new(url: String) -> Result<Self, SolutionValidationError> {
        let url = url.trim();
        if url.chars().count() > MAX_SUBMIT_URL_LENGTH {
            return Err(SolutionValidationError::SubmitUrlTooLong {
                max: MAX_SUBMIT_URL_LENGTH,
            });
        }
        if !(url.is_empty() || url.starts_with("https://") || url.starts_with("http://")) {
            return Err(SolutionValidationError::InvalidSubmitUrl);
        }
        Ok(Self(url.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{SolutionBody, SolutionTitle, SubmitUrl};
    use crate::error::solution::SolutionValidationError;

    #[test]
    fn creates_valid_values_and_normalizes_title_and_url() {
        let title = SolutionTitle::try_new(" title ".to_string()).expect("valid title");
        let body = SolutionBody::try_new(" body ".to_string()).expect("valid body");
        let submit_url =
            SubmitUrl::try_new(" https://example.com ".to_string()).expect("valid URL");

        assert_eq!(title.as_str(), "title");
        assert_eq!(body.as_str(), " body ");
        assert_eq!(submit_url.as_str(), "https://example.com");
    }

    #[test]
    fn rejects_blank_or_too_long_title() {
        assert_eq!(
            SolutionTitle::try_new("   ".to_string()),
            Err(SolutionValidationError::BlankTitle)
        );
        assert_eq!(
            SolutionTitle::try_new("a".repeat(121)),
            Err(SolutionValidationError::TitleTooLong { max: 120 })
        );
    }

    #[test]
    fn rejects_blank_or_too_long_body() {
        assert_eq!(
            SolutionBody::try_new("   ".to_string()),
            Err(SolutionValidationError::BlankBody)
        );
        assert_eq!(
            SolutionBody::try_new("a".repeat(20_001)),
            Err(SolutionValidationError::BodyTooLong { max: 20_000 })
        );
    }

    #[test]
    fn accepts_blank_http_and_https_urls_and_rejects_other_schemes() {
        for url in ["", "http://example.com", "https://example.com"] {
            assert!(SubmitUrl::try_new(url.to_string()).is_ok());
        }

        assert_eq!(
            SubmitUrl::try_new("ftp://example.com".to_string()),
            Err(SolutionValidationError::InvalidSubmitUrl)
        );
    }
}
