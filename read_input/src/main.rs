use std::io;

fn main(){
    let mut input=String::new();

    println!("Hey Mate Say something--");

    match io::stdin().read_line(&mut input){
        Ok(_) =>{
            println!("You said-{}",input);
        },
        Err(e) => println!("OOPS,somthing went wromg {}",e)
    }
}