fn main() {
    let mut n = 1;

    
    loop {
        if n > 10 {
            break; 
        }
        
        println!("Tabla de multiplicar del {}\n", n);

        let mut u = 1;
        
        loop {
            if u > 10 {
                break; 
            }

            println!("{} x {} = {}", n, u, n * u);
            u += 1; 
        }

        println!(""); 
        n += 1; 
    }
}