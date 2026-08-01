use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SolutionId(Uuid);

impl SolutionId {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for SolutionId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<SolutionId> for Uuid {
    fn from(value: SolutionId) -> Self {
        value.into_uuid()
    }
}
