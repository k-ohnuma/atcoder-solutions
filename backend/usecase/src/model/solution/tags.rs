use itertools::Itertools;

use crate::error::SolutionError;

const MAX_TAG_COUNT: usize = 6;
const MAX_TAG_LENGTH: usize = 24;

pub(crate) fn normalize_tags(tags: &[String]) -> Result<Vec<String>, SolutionError> {
    let tags = tags
        .iter()
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty())
        .map(str::to_string)
        .sorted()
        .dedup()
        .collect_vec();

    if tags.len() > MAX_TAG_COUNT {
        return Err(SolutionError::BadRequest(format!(
            "tags must be {MAX_TAG_COUNT} or fewer"
        )));
    }
    if tags.iter().any(|tag| tag.chars().count() > MAX_TAG_LENGTH) {
        return Err(SolutionError::BadRequest("tag is too long".to_string()));
    }

    Ok(tags)
}

#[cfg(test)]
mod tests {
    use super::normalize_tags;
    use crate::error::SolutionError;

    #[test]
    fn normalizes_tags() {
        let tags = vec![
            " dp ".to_string(),
            "".to_string(),
            "graph".to_string(),
            "dp".to_string(),
            "   ".to_string(),
        ];

        assert_eq!(
            normalize_tags(&tags).expect("valid tags"),
            vec!["dp".to_string(), "graph".to_string()]
        );
    }

    #[test]
    fn rejects_more_than_six_normalized_tags() {
        let tags = (1..=7)
            .map(|index| format!("tag{index}"))
            .collect::<Vec<_>>();

        assert!(matches!(
            normalize_tags(&tags),
            Err(SolutionError::BadRequest(message)) if message == "tags must be 6 or fewer"
        ));
    }

    #[test]
    fn rejects_tag_longer_than_twenty_four_characters() {
        let tags = vec!["a".repeat(25)];

        assert!(matches!(
            normalize_tags(&tags),
            Err(SolutionError::BadRequest(message)) if message == "tag is too long"
        ));
    }
}
