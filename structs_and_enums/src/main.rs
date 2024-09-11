struct Usuario {
    nombre : String,
    email : String,
    edad : i32,
    activo : bool
}

fn main() {
    let user = Usuario {
        nombre : "Julio".to_string(),
        email : String::from("something@email.com"),
        edad : 32,
        activo : true
    };

    println!("Usuario {}, \nedad {}", user.nombre, user.edad);
    
    let user1 = nuevo_usuario(String::from("Andres"), String::from("some@email.com"));
    
    let user2 = Usuario {
        nombre : "Ana".to_string(),
        email : String::from("some@email.com"),
        ..user1
    }; 
    println!("\nUsuario {}, \nedad {}, \nemail {}, \nactivo {}", user2.nombre, user2.edad, user2.email, user2.activo);
}

fn nuevo_usuario(nombre: String, email: String) -> Usuario {
    return Usuario{
        nombre,
        email,
        edad : 32,
        activo : true
    };
}
