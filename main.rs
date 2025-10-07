
//Esparza Gaxiola Edgardo
fn main() {
    let mut i : u8 = 1;
    let mut j : u8;
    loop {
            j = 1;
        loop {
            println!("{}x{}={}",i,j,i*j);
            if j == 10 {
                break;
            }
            j+=1;                
        }
        println!("");
        if i == 10 {
            break;            
        }
        i+=1;
    
    }
}
