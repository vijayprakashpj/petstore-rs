#[derive(Debug, PartialEq, Eq)]
pub enum PetStatus {
    Available,
    Unavailable,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PetTag(pub String, pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub status: PetStatus,
    pub category: String,
    pub tags: Vec<PetTag>,
}
