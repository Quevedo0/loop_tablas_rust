fn main() {
    let mut i: i8 = 1;
    let mut j: i8 = 1;
    let mut resultado: i8;
print!("tablas de multiplicar\n");

loop {
    if i == 1 && j == 1 {
        println!("Tabla del {}", i);
    } else if j == 1 {
        println!("\nTabla del {}", i);
        
    }
    resultado = i * j;
    println!("{} x {} = {}", i, j, resultado);
    if j < 10 {
        j += 1;
    } else {
        j = 1;
        i += 1;
        
    }
    if i > 10 {
        break;
    }

}
}
