fn main() {
    let mut contador = 1;
    
    println!("------Las Tablas de multiplicar--------");
    
loop {
        println!("--------tabla del {}---------", contador);
        let mut num= 1;
loop {
        println!("{} * {} = {}", contador, num, contador * num);
        num += 1;
        if num > 10 {
      break;
                 }
    }
        contador += 1;
        if contador > 10 {
            break;
                       }   
   }

}
