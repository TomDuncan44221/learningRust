use user_input_string:: get_string;
use user_input_number:: get_number;
use std::io;

fn main() 
{
    let input: f32 = get_number("Please enter the temperature number \n");

    let input_metric: String = get_string("Please enter the temperature number \n");

    println!("Are we working in Celsius?  Y/N");

    
    
    

}

fn convert_celsius(c: f32) -> f32
{
    // C + 32 * 1.8 = F is our formula

    return 2.0;
}

fn convert_fahrenheit(f: f32) -> f32
{
    // F - 32 / 1.8 = C

    return 1.0;
}

mod user_input_string
{
    //use std::io;

    pub fn get_string(prompt: &str) -> String 
    {
        println!("{}", prompt);

        let mut input = String::new();

        return input;

    }
}

mod user_input_number
{
    //use std::io;

    pub fn get_number(prompt: &str) -> f32
    {
        println!("{}", prompt);

        let mut input = f32::new();

        return input;

    }
}