fn main() {
    let nuevo = User {
        name : "Julio".to_string(),
        username : Some( "julix".to_string() ),
        age : 12,
    };

    let age = nuevo.get_age();
    let username : nuevo.get_username();
    match username {
        Some(username) => println!("username: {}", username),
        _ => (),
    }
    println!("edad {}", age);
}

struct User { 
    name : String,
    username : Option<String>,
    age : i32,
}

impl User {
    fn get_age(&self) -> i32 {
        self.age
    }
    fn get_username(&self) -> Option<String> {
        self.username;
    }
}