fn main() {
//Diego Santos Treviño Camacho    

let mut i: i8=0;
let mut j: i8=0;

loop{
    i += 1;
    println!("Tabla del {}.",i);
    loop{
        j += 1;
        println!("{} x {} = {}",i,j,i*j);
        if j ==10{
            break;
        }
    }
    println!("\n");
    j = 0;
    if i ==10{
            break;
}
}
}