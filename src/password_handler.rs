use rand::{distributions::Alphanumeric, Rng};

pub fn generate_salt() -> String {
    let s:String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();

    return s;
}
pub fn salt_password(password:String) -> (String, String) {
    let salt:String = generate_salt();
    let s:String = format!("{}{}",password, salt).to_string();
    return (s, salt);
}
pub fn hash_password(salted_password:String) -> Result<String, bcrypt::BcryptError> {
    return bcrypt::hash(salted_password,8);
}