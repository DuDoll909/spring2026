/*

fn assignment1(){

let testF2C: f64 = 32.0;
let testC2F: f64 = 0.0;
let mut num: f64 = 1.0;


fn fahrenheit_to_celsius(f: f64) -> f64{
    let x: f64 = 32.0;
    let f ={
        f - x
    };
    f * 0.55
    //(C * 1.8) + 32;
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    let x: f64 = 1.8;
    let f ={
        c * x
    };
    f + 32.0
    //(C * 1.8) + 32;

}

    
println!("32 from f to c is {} while 0 from c to f is {}",fahrenheit_to_celsius(testF2C),celsius_to_fahrenheit(testC2F));


    /*
    The code below loops through the functions a certain amount of times
    while incrementing the number
    */
    loop {
        println!("{} from f to c is {:.2} while {} from c to f is {:.2}",(testF2C + num),fahrenheit_to_celsius(testF2C + num),(testC2F + num),celsius_to_fahrenheit(testC2F + num));
        num += 1.0;

        if num == 6.0 {
            break;
        }
    }
}
*/
/*
fn assignment2()
{
    let numbers = [1,2,3,4,5,57,6,7,8,9,10,15];

    let mut total = 0;
    let mut largest = numbers[0];
        //This code ONLY determines if the given number is even or odd
        fn isEven(num:i32) -> bool
        {
        num % 2 == 0 
        }
            /*The code below will iterate through the array and first check
            if there's a number divisable by both 3 and 5, then itll check
            if there's a number divisible solely by either 3 or 5 
            */ 
            for n in numbers.iter() 
            {
                if n % 3 == 0 && n % 5 == 0
                {
                    println!("FizzBuzz");
                }
                else if n % 3 == 0
                {
                    println!("Fizz");
                }
                else if n % 5 == 0
                {
                    println!("Buzz");
                }
                else
                {
                    println!("{}", isEven(*n));
                }
            /*
            The code below will iterate through the array a numbers.len()
            amount of times and add the current number its on till it 
            reaches the end
            */
            }
            let mut i = 0;
            while i < numbers.len(){
                for n in numbers.iter(){
                    total = total + *n;
                    i+=1;
                }
            }
            println!("The total number is {}", total);  
            /*
            The code below starts with "largest" as the first number in the
            array and iterates through using "current" to determine if the 
            next number is greater. if so, largest is updated. If not,
            it continues.
            
            */
            for n in numbers.iter(){
                let current = *n;
                if current > largest{
                    largest = current;
                }
            }
            println!("The largest number is {}", largest);
}
*/
    
fn assignment3(){
let secret = 25;
let mut guess = 30;
let mut loops = 0;
    fn check_guess(guess: i32, secret: i32) -> i32{
        if guess == secret
        {
            0
        }
        else if guess > secret
        {
            1
        }
        else
        {
            -1
        }

    }
    println!("The answer is {}(should be 1)", check_guess(guess, secret));

    loop{
        if check_guess(guess,secret) == 0{
            loops += 1;
            break;
        }
        if check_guess(guess,secret) == -1{
            guess += 1;
            loops += 1;
        }
        else{
            guess -= 1;
            loops += 1;
        }

    }
println!("The amount of tries you took was {}", loops);
}

fn main(){
 assignment3();
}