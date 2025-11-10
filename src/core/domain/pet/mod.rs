use std::{error::Error, fmt};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum PetStatus {
    Available,
    Pending,
    Sold,
}

#[derive(Debug, PartialEq)]
pub struct PetStatusError(String);
impl fmt::Display for PetStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for PetStatusError {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PetTag(pub String, pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub status: PetStatus,
    pub category: String,
    pub tags: Vec<PetTag>,
}

#[allow(dead_code)]
impl Pet {
    pub fn is_available(self: &Pet) -> bool {
        self.status == PetStatus::Available
    }

    pub fn tag(self: &mut Pet, tag: PetTag) {
        self.tags.push(tag);
    }

    pub fn can_sell(self: &Pet) -> bool {
        self.status == PetStatus::Available
    }

    // TODO: To be extended with buyer data
    pub fn sell(self: &mut Pet) -> Result<(), PetStatusError> {
        match self.status {
            PetStatus::Available => {
                self.status = PetStatus::Sold;
                Ok(())
            }
            PetStatus::Pending => Err(PetStatusError(String::from(
                "Pet sale is being completed by another customer",
            ))),
            PetStatus::Sold => Err(PetStatusError(String::from(
                "Pet is not available for adoption",
            ))),
        }
    }
}

#[allow(dead_code)]
pub struct PetQuery {
    id: Option<i64>,
    name: Option<String>,
    status: Option<PetStatus>,
    category: Option<String>,
}

#[allow(dead_code)]
pub trait PetRepository {
    fn save(pet: Pet) -> Result<Pet, Box<dyn Error>>;
    fn get_by_id(pet_id: i64) -> Option<Pet>;
    fn get_by_query(query: PetQuery) -> Vec<Pet>;
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::core::domain::pet::{Pet, PetStatus, PetStatusError, PetTag};

    #[rstest]
    #[case(PetStatus::Available, true)]
    #[case(PetStatus::Pending, false)]
    #[case(PetStatus::Sold, false)]
    fn test_pet_availability(#[case] status: PetStatus, #[case] expected: bool) {
        let pet = Pet {
            id: 1,
            name: String::from("Tommy"),
            status,
            category: String::from("Dog"),
            tags: vec![PetTag(String::from("size"), String::from("small"))],
        };

        assert_eq!(pet.is_available(), expected);
    }

    #[rstest]
    pub fn test_add_tag(
        #[values(vec!(), vec!(PetTag(String::from("size"), String::from("small"))))] existing_tags: Vec<
            PetTag,
        >,
    ) {
        // GIVEN
        let mut pet = Pet {
            id: 1,
            name: String::from("Tommy"),
            status: PetStatus::Available,
            category: String::from("Dog"),
            tags: existing_tags.clone(),
        };
        assert_eq!(pet.tags, existing_tags);
        let new_tag = PetTag(String::from("colour"), String::from("brown"));

        // WHEN
        pet.tag(new_tag.clone());

        // THEN
        let mut expected_tags = existing_tags;
        expected_tags.push(new_tag);
        assert_eq!(pet.tags, expected_tags);
    }

    #[rstest]
    #[case(PetStatus::Available, true)]
    #[case(PetStatus::Sold, false)]
    fn test_can_sell(#[case] status: PetStatus, #[case] expected_result: bool) {
        // GIVEN
        let pet = Pet {
            id: 1,
            name: String::from("Unicorn"),
            status,
            category: String::from("Imaginary"),
            tags: vec![],
        };

        // WHEN
        // THEN
        assert_eq!(pet.can_sell(), expected_result);
    }

    #[rstest]
    #[case(PetStatus::Available, Ok(()))]
    #[case(PetStatus::Pending, Err(PetStatusError("Pet sale is being completed by another customer".to_string())))]
    #[case(PetStatus::Sold, Err(PetStatusError("Pet is not available for adoption".to_string())))]
    fn test_pet_sale(
        #[case] status: PetStatus,
        #[case] expected_result: Result<(), PetStatusError>,
    ) {
        // GIVEN
        let mut pet = Pet {
            id: 1,
            name: String::from("Sher"),
            status,
            category: String::from("Tiger"),
            tags: vec![PetTag(String::from("Family"), String::from("Cat"))],
        };

        // WHEN
        let result = pet.sell();

        // THEN
        assert_eq!(result, expected_result);
    }
}
