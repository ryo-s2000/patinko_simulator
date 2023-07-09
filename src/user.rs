pub struct User {
    pub balance: isize,
    pub name: String
}

impl User {
    pub fn new(name: String) -> Self {
        User {
            balance: 0,
            name,
        }
    }
}
