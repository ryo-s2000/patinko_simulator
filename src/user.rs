pub struct User {
    pub balance: isize,
    pub name: String,
    pub max_traials: usize,
}

impl User {
    pub fn new(name: String, max_traials: usize) -> Self {
        User {
            balance: 0,
            name,
            max_traials,
        }
    }
}
