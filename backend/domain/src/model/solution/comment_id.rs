use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommentId(Uuid);

impl CommentId {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for CommentId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<CommentId> for Uuid {
    fn from(value: CommentId) -> Self {
        value.into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::CommentId;

    #[test]
    fn preserves_uuid() {
        let value = Uuid::now_v7();
        let comment_id = CommentId::new(value);

        assert_eq!(comment_id.into_uuid(), value);
    }
}
