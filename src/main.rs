use std::{io};
use rand::{self, Rng};

fn main() {
  let selectprogram = rand::thread_rng().gen_range(1..100);
  println!("{selectprogram}");
  if selectprogram < 50 {
      tempconverter()
  } else if selectprogram > 50 {
      loginpro()
  }
}


fn tempconverter()
{
    let mut select = String::new();
    let selectint : i8;
    let result: i32;
    let mut temperature = String::new();
    let temperatureint: i32;


    println!("Temperature converter written in Rust!");
    println!("Convert from:");
    println!("1- Celsius to Farenheit");
    println!("2- Farenheit to Celsius");

    io::stdin()
        .read_line(&mut select)
        .expect("Lel");

    selectint = select.trim().parse::<i8>().unwrap();
    
    if selectint > 2 || selectint < 1 {
        println!("Program crashed because of wrong input.");
        std::process::exit(0)
    }

    println!("Great! Now enter the temperature to convert");

    io::stdin()
        .read_line(&mut temperature)
        .expect("LMAO BRO");
    
    temperatureint = temperature.trim().parse::<i32>().unwrap();

    if selectint == 1
    {
        result = c_to_f(temperatureint)
    } else if selectint == 2 
    {
        result = f_to_c(temperatureint)    
    } else {
        panic!("Something happened and the program crashed.")
    }

    println!("Calculation done! Result is {}, converted from {}!", result, temperatureint)

}
fn c_to_f(c: i32) -> i32 
{   
    c * 9 / 5 + 32
}

fn f_to_c(f: i32) -> i32
{
    (f - 32) * 5 / 9
}

fn loginpro()
{
    let mut placeholdername = String::new();
    let mut placeholderpass = String::new();

    struct Login
    {
        name: String,
        password: String
    }

    println!("HELLO THIS IS PRO'S PROGRAM NICE TO HAVE YOU HERE! \n Create your account to use our program! \nStart with a name for your account:");

    io::stdin()
        .read_line(&mut placeholdername)
        .expect("Wrong input!");

    println!("Great! Now the password for your account:");

    io::stdin()
        .read_line(&mut placeholderpass)
        .expect("Wrong input!");

    let account = Login{name: placeholdername.to_uppercase(), password:placeholderpass.to_uppercase()};
    println!("\nAccount created! Your account's name is {} and your password is {}!", account.name, account.password);
}