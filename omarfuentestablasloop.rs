fn main() {
    let mut i = 1;

    
    loop {
        if i > 10 {
            break; 
        }
        
        println!("Tabla de multiplicar del {}", i);

        let mut j = 1;
        
        loop {
            if j > 10 {
                break; 
            }

            println!("{} x {} = {}", i, j, i * j);
            j += 1; 
        }

        println!(""); 
        i += 1; 
    }
}
