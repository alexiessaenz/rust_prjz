use random::Source;

fn main() {
    let mut _nume = 23;
    _nume=2;
    let mut source = random::default(42);
    let num = source.read::<i32>();
    println!("Tu numero random: {}", num);
    let _rev: i32 = selecion(8);
}

fn selecion(arg: i32) -> i32 {
    return arg;
}
