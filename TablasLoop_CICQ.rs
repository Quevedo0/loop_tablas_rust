//Camacho Quevedo Carolina Isabel
fn main() {
  let mut i: i16=0;
  let mut j: i16=0;

  loop {
  i+=1;
    println!("-- Tabla del {i} --\n");
        loop {
            j+=1;
                println!("{i} x {j} = {}", i * j);
            if j>=10{
            break;
            }
        }
         println!();
        j=0;
  if i>=10{
     break;
   }
  }
}
