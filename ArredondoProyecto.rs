fn main() {
    let mut contador = 1;
    
    println!("tablas de multiplicar de 1 a 10");
    
    loop {
        println!("tabla del {}", contador);
        let mut numero = 1;
        loop {
            println!("{} x {} = {}", contador, numero, contador * numero);
            numero += 1;
            if numero > 10 {
                break;
            }
        }
        contador += 1;
        if contador > 10 {
            break;
        }
    
    
    
    
    
    }

}
