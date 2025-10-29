use::std::io;
use::rand::Rng;
use::std::cmp::Ordering;
// ask for user input
//process the input
//check nd validate input in expected format

fn main()-> io::Result<()>{

    loop {
    println!("Welcome Brave, Here you comes for Gussing a Number , Right ?");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please Enter a Number to  Guess the input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You Guessed :{}", input.trim());
    println!("actual: {}", secret_number);

    let input:u32 =  match input.trim().parse(){
        Ok(num)=>num,
        Err(_) => continue,
    };
    
    match input.cmp(&secret_number) {
        Ordering::Equal => {
            println!("You win!");
            break;
        },
        Ordering::Greater => println!("Too big"),
        Ordering::Less => println!("Too small")
    }
    }
    // we will use rand to create secret no and make it guess
    Ok(())

}
