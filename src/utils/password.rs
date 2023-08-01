use pwhash::bcrypt;

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash)
}
