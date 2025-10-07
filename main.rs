//KEVIN SALAS -- CIENCIAS DE DATOS -- TABLAS DE MULTIPLICAR CON LOOP


fn main(){
 let mut multiplicando:i8=1;
 let mut multiplicador:i8=1;

    loop{
    println!("Tabla del numero: {}",multiplicando);

            loop{
             println!("{} x {} = {}", multiplicando ,multiplicador, multiplicando * multiplicador);
                
                multiplicador += 1;

                if multiplicador > 10{
                    multiplicador = 0;
                    break;
                }
            }

            multiplicando += 1;

            println!("");

         if multiplicando >10{
            break;
            }   

    }

}