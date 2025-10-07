fn main() {
   
   // Loops
   
   let mut contador1: i8 = 1;

   loop {
        println!("La tabla de {}",contador1);
        let mut contador2: i8  = 1;
    loop {
        println!("{} x {} = {}",contador1, contador2, contador1 * contador2);
        contador2 +=1;
        if contador2 == 11 {
            break;
        }
    }
    println!("\n");
    if contador1 == 10 {
            break;
    }
    contador1 +=1;
   }
}
