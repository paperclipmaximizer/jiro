use uuid::Uuid;

struct Course {
    id: Uuid,
    name: String,
    description: String,
    image: String,
    creator_id: Uuid,
    language: String,
}

impl Course {
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        image: String,
        creator_id: Uuid,
        language: String,
    ) -> Course {
        Course {
            id: Uuid::new_v4(),
            name,
            description,
            image,
            creator_id,
            language,
        }
    }
}
