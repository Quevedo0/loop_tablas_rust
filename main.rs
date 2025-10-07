fn main() {
let mut resul:i8;
let mut m:i8=1;
let mut n:i8=1;

loop {
    println!("Tabla del {}",n);
    loop {
        resul= m*n;
        println!("{} x {} = {}", n,m,resul);

        if m==10{
            m=1;
            break;
        
        }else {
            m=m+1;
        }
    
}
   if n==10{
            break;
        
        }else {
            n=n+1;
        } 
println!()
}

}
