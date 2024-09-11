

fn main() {
    mostrar_bienvenida();
    let mut _n: i32;
    _n = 32;
    let num = {
        10
    };

    funcion_por_referencia(&num);
    println!("{} al cuadrado: {}", num, funcion_cuadrada(num) );

}

fn funcion_cuadrada(num: i32) -> i32{
    return num * num;
}

fn funcion_por_referencia(num: &i32){
    println!("Tu numero por refencia: {}", num);
}

fn mostrar_bienvenida() {
    println!("Bienvenidos a Rust!")
}
