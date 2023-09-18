use uuid::Uuid;

pub struct User {
    uid: Uuid,
    user_name: String,
    name: String,
    home_dir: String, // probably change to Path later
}
