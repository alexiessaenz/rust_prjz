struct Usuario {
    nombre : String,
    email : String,
    web_site : Website,
    edad : i32,
    activo : bool,
    user_role : UserRole
}

struct People {
    nombre : String,
    email : String,
    nacimiento : i32,
    activo : bool
}
impl  People {
    fn edad(&self) -> i32{
        let actual = 2024;
        return actual - self.nacimiento;
    }
}

fn main() {
let role = UserRole::USER;

    let user = Usuario {
        nombre : "Julio".to_string(),
        email : String::from("something@email.com"),
        edad : 32,
        web_site : Website::INSTAGRAM( String::from("@alexiessaenz") ),
        activo : true,
        user_role : role
    };

    println!("Usuario {}, \nedad {}", user.nombre, user.edad);
    
    let user1 = nuevo_usuario(String::from("Andres"), String::from("some@email.com"));
    
    let user2 = Usuario {
        nombre : "Ana".to_string(),
        email : String::from("some@email.com"),
        ..user1
    }; 
    println!("\nUsuario {}, \nedad {}, \nemail {}, \nactivo {}, \nwebsite {:#?}", user2.nombre, user2.edad, user2.email, user2.activo, user2.web_site);

    //tuples struct
    struct Point(i32,i32,i32);
    let point_a = Point(12, 33, 66);

    let people = People {
        nombre : "Julio".to_string(),
        email : String::from("something@email.com"),
        nacimiento : 1993,
        activo : true
    };

    println!("\nPeople: \nnombre {}, \nedad {}, \nemail {}, \nactivo {}", people.nombre, people.edad(), people.email, people.activo);
    
    let access = hasAccess(user.user_role);
    println!("user tiene acceso? : {}", access);

}

fn nuevo_usuario(nombre: String, email: String) -> Usuario {
    return Usuario{
        nombre,
        email,
        edad : 32,
        activo : true,
        web_site : Website::INSTAGRAM( String::from("@alexiessaenz") ),
        user_role : UserRole::ADMIN
    };


}

enum UserRole {
    USER,
    ADMIN,
}

#[derive(Debug)]
enum Website {
    URL(String),
    INSTAGRAM(String),
    FACEBOOK(String),
    LINKEDIN(String)
}

fn hasAccess( user_role : UserRole ) -> bool {
    match user_role {
        UserRole::ADMIN => true,
        UserRole::USER => false,
    }
}