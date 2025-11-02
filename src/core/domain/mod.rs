#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum PetStatus {
    Available,
    Unavailable,
}

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
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::core::domain::{Pet, PetStatus, PetTag};

    #[rstest]
    #[case(PetStatus::Available, true)]
    #[case(PetStatus::Unavailable, false)]
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
}
