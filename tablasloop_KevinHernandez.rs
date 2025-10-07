//kevin omar hernandez martinez
fn main() {
    let mut i:u8 = 1; 
    
    loop {
        println!("Tabla de multiplicar del {}", i);
        
        let mut j:u8 = 1;
        
        loop {
            println!("{} x {} = {}", i, j, i * j);
            j += 1;

            if j > 10 {
                break; 
            }
        }

        println!(); 
        i += 1;

        if i > 10 {
            break; 
        }
    }
}