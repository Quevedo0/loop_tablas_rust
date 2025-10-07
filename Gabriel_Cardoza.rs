fn main(){
    //Carlos Gabriel Cardoza Rios
let mut i:i8=1;
let mut j:i8=1;
let mut resul:i8;
loop {
    println!("Tabla del {i}");
    loop{
        resul=i*j;
        println!("{i}*{j}={resul}");
        if j==10 {
            j=1;
            break;
        } else {
            j=j+1;
        }
    }
    if i==10{
        break;
    } else {
        i=i+1;
    }
    println!();
}
}