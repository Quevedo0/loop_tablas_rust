//Tablas de Multiplicar con Loop
//Loaiza Ramirez FAbian Leonardo

fn main(){

    let mut num: i32 = 1;

    loop {
        
        let mut num2: i32 = 1;

        println!("\nTabla del {num}");
        
        loop {
            
            println!("{num} x {num2} = {}", num * num2);

            num2 += 1;

            if num2 > 10 {
                break;
            }

        }
        
        num += 1;

        if num > 10 {
            break;

        }
    
    }
}