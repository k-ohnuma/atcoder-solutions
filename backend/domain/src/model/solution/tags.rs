use std::collections::BTreeSet;

use crate::error::solution::SolutionValidationError;

const MAX_TAG_COUNT: usize = 6;
const MAX_TAG_LENGTH: usize = 24;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagName(String);

impl TagName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tags(Vec<TagName>);

impl Tags {
    pub fn try_new(tags: Vec<String>) -> Result<Self, SolutionValidationError> {
        let tags = tags
            .into_iter()
            .filter_map(|tag| {
                let normalized = tag.trim();
                (!normalized.is_empty()).then(|| TagName(normalized.to_string()))
            })
            .collect::<BTreeSet<_>>();

        if tags.len() > MAX_TAG_COUNT {
            return Err(SolutionValidationError::TooManyTags { max: MAX_TAG_COUNT });
        }
        if tags
            .iter()
            .any(|tag| tag.as_str().chars().count() > MAX_TAG_LENGTH)
        {
            return Err(SolutionValidationError::TagTooLong {
                max: MAX_TAG_LENGTH,
            });
        }

        Ok(Self(tags.into_iter().collect()))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TagName> {
        self.0.iter()
    }

    pub fn into_strings(self) -> Vec<String> {
        self.0.into_iter().map(|tag| tag.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Tags;
    use crate::error::solution::SolutionValidationError;

    #[test]
    fn normalizes_tags() {
        let tags = Tags::try_new(vec![
            " dp ".to_string(),
            "".to_string(),
            "graph".to_string(),
            "dp".to_string(),
            "   ".to_string(),
        ])
        .expect("valid tags");

        assert_eq!(
            tags.iter().map(|tag| tag.as_str()).collect::<Vec<_>>(),
            vec!["dp", "graph"]
        );
    }

    #[test]
    fn rejects_more_than_six_normalized_tags() {
        let tags = (1..=7).map(|index| format!("tag{index}")).collect();

        assert_eq!(
            Tags::try_new(tags),
            Err(SolutionValidationError::TooManyTags { max: 6 })
        );
    }

    #[test]
    fn rejects_tag_longer_than_twenty_four_characters() {
        assert_eq!(
            Tags::try_new(vec!["a".repeat(25)]),
            Err(SolutionValidationError::TagTooLong { max: 24 })
        );
    }
}
