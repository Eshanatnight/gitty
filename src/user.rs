use uuid::Uuid;

pub struct User {
    uid: Uuid,
    user_name: String,
    name: String,
    home_dir: String, // probably change to Path later
}

impl User {
    pub fn new(user_name: String, name: String) -> User {
        let random_bytes = rand::random();
        User {
            uid: uuid::Builder::from_random_bytes(random_bytes).into_uuid(),
            user_name,
            name,
            home_dir: String::new()
        }
    }
}