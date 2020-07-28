fn main() 
{
    let number = 5;
    let a = [">--->", "#>--->", "##>--->", "####>--->", "#######>--->", "##########>--->", "#############>--->"];
    let mut counter = 0;
    let mut result =  loop  // This is a loop
        {
            
            if number < 5 
            {
            println!("condition was true");
            counter += 2;
            } else 
            {
            println!("condition was false");
            counter += 1;
            }
            if counter == 10
            {
                break counter * 2  // This is how you return a value
            }
            println!("again!");
        
        };
    println!("The result is {}", result);

    
    while result != 0  // While loop
    {
        println!("{}!", result);

        result -= 1;
    }
    println!("LIFTOFF!!!");

    for element in a.iter()  // For loop that iterates through array
    {
        println!("{}", element);
    }

    println!("Fuel leak, she cannae take nae more!");

    for number in (1..4).rev()  // Iterates backwards through array
    {
        println!("{}!", number);
    }

    println!("KABOOM!");
}   