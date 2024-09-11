struct Usuario {
    nombre : String,
    email : String,
    edad : i32,
    activo : bool
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

}

fn nuevo_usuario(nombre: String, email: String) -> Usuario {
    return Usuario{
        nombre,
        email,
        edad : 32,
        activo : true
    };


}
