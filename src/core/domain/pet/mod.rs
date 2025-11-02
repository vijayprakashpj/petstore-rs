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

    pub fn tag(self: &mut Pet, tag: &PetTag) {
        self.tags.push(tag.to_owned());
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
            PetStatus::Pending => Err(PetStatusError(
                "Pet sale is being completed by another customer".to_owned(),
            )),
            PetStatus::Sold => Err(PetStatusError(
                "Pet is not available for adoption".to_owned(),
            )),
        }
    }
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
            name: "Tommy".to_owned(),
            status: status,
            category: "Dog".to_owned(),
            tags: vec![PetTag("size".to_owned(), "small".to_owned())],
        };

        assert_eq!(pet.is_available(), expected);
    }

    #[rstest]
    pub fn test_add_tag(
        #[values(vec!(), vec!(PetTag("size".to_owned(), "small".to_owned())))] existing_tags: Vec<
            PetTag,
        >,
    ) {
        // GIVEN
        let mut pet = Pet {
            id: 1,
            name: "Tommy".to_owned(),
            status: PetStatus::Available,
            category: "Dog".to_owned(),
            tags: existing_tags.clone(),
        };
        assert_eq!(pet.tags, existing_tags);
        let new_tag = PetTag("colour".to_owned(), "brown".to_owned());

        // WHEN
        pet.tag(&new_tag);

        // THEN
        assert_eq!(
            pet.tags,
            existing_tags
                .iter()
                .chain(vec![new_tag].iter())
                .cloned()
                .collect::<Vec<PetTag>>()
        );
    }

    #[rstest]
    #[case(PetStatus::Available, true)]
    #[case(PetStatus::Sold, false)]
    fn test_can_sell(#[case] status: PetStatus, #[case] expected_result: bool) {
        // GIVEN
        let pet = Pet {
            id: 1,
            name: "Unicorn".to_owned(),
            status,
            category: "Imaginary".to_owned(),
            tags: vec![],
        };

        // WHEN
        // THEN
        assert_eq!(pet.can_sell(), expected_result);
    }

    #[rstest]
    #[case(PetStatus::Available, Ok(()))]
    #[case(PetStatus::Pending, Err(PetStatusError("Pet sale is being completed by another customer".to_owned())))]
    #[case(PetStatus::Sold, Err(PetStatusError("Pet is not available for adoption".to_owned())))]
    fn test_pet_sale(
        #[case] status: PetStatus,
        #[case] expected_result: Result<(), PetStatusError>,
    ) {
        // GIVEN
        let mut pet = Pet {
            id: 1,
            name: "Sher".to_owned(),
            status,
            category: "Tiger".to_owned(),
            tags: vec![PetTag("Family".to_owned(), "Cat".to_owned())],
        };

        // WHEN
        let result = pet.sell();

        // THEN
        assert_eq!(result, expected_result);
    }
}
