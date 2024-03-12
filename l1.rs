#[derive(Debug)]
struct User {
    username: String,
    phone: u32,
    active: bool,
}

impl User {
    fn name_len(&self) -> u32 {
        self.username.len() as u32
    }
}

fn main() {
    let user1 = User {
        username: "Mrigesh".to_string(),
        phone: 123,
        active: true,
    };

    let user2 = User {
        username: String::from("Asrani"),
        ..user1
    };

    println!("{:#?}", user2);
    println!("{}", user2.name_len());
}
