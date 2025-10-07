fn main() {
let mut i = 1;
	loop {
	println!("Tabla del {}", i);
        	
	let mut j = 1; 
	loop{
        	let multi: i16 = i * j;
        	println!("{} X {} = {}",i ,j , multi);

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
