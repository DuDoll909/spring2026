
/*
//const MAX_POINTS: u32 = 100_000;//this will work
//const MAX_POINTS = 100_000; //This will NOT work

fn main() {
    
/*
let x = 5;
let y = 10;

println!("{}*2={}",x,y); 
*/


/*
    let mut result = 100; /*when creating a variable,
    you need to let compiler know if you're planning
    to change it(mut)*/
    result += 100;
    println!("{}",result);  */


    //let mut result: f32 /* <-- datatype*/ = 0.0;
    //let mut result = 0f32;//Datatype can also be used like this
    //result += 1.5;
    /*in other programming languages,
    data types get converted implicitly!
    Example:
    int x = 0;
    f y = 1.5;
    x+y



    println!("{}", result);
    */

    /*
    let x = 10;
    let y = 2.0;

    //let result = x*y; <- this doesn't let you convert datatypes
    let result = x*(y as i32); // <-- THIS lets you convert datatypes

    println!("{}", result);
*/

/*
    let result = MAX_POINTS*10;//using the const at the top

    println!("{}", result);

    */




    
}

*/

//PREVIEW 2: SHADOWING


/*
fn main(){
/*
    //let num = "25";//works with i32
    let num = "25.5";//works with f32
    let result: Result<f32,_> = num.parse().expect("I am sure");
    println!("{:?}",result);
*/

    let x = 10;
    //x += 10;// x is immutable, can't change its value
    let x = x + 10;


    println!("{}",x);
}
*/

//PREVIEW 5: FUNCTIONS

/*
fn get_number(y:i32) -> i32 {

    let x = 5;
    //return x + y;
    //x + y //no semicolon; it's an expression
    let y = {
        let x = x + 3;
        y + x
    };
    y//gives 18
    // y + x //gives 23 because "let x = x + 3" is in a scope and isn't used outside of it


}

fn main(){

    println!("{}",get_number(10));

}
*/

//PREVIEW 9: ASSIGNMENTS

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

/*
fn assignment2()
{
    let mut total = 0;
    

    let numbers = [1,2,3,4,5,57,6,7,8,9,10,15];


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
*/





fn main(){
 assignment1();
}
// 1/29/26

/*

fn main() {
    // Creating an array
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    // Accessing array elements
    let first = months[0];
    let second = months[1];
    println!("First month: {}, Second month: {}", first, second);
    
    // Array with explicit type and size
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Array with repeated values
    let repeated = [3; 5]; // Equivalent to [3, 3, 3, 3, 3]
    
    println!("Third number: {}, First repeated: {}", numbers[2], repeated[0]);

    // Printing entire array (debug format)
    println!("Numbers: {:?}", numbers);

    // Demonstrating bounds checking
    let index = 10;
    // Uncommenting the next line would cause a runtime panic
    // let element = numbers[index];
}

*/
/*
fn color(c:char) -> (u8,u8,u8){

    match c {
        'R' => (255,0,0),
        'G' => (0,255,0),
        'B' => (0,0,255),
        _=> (0,0,0),

    }
    /*
        if c == 'R'{
            return (255,0,0);
        }
        if c == 'G'{
            return (0,255,0);
        }
        if c == 'B'{
            return (0,0,255);
        }
        (0,0,0)
        */


    }

fn main(){
    //we are going to accept a letter like RGB
    //and we should return a tuple 
    // RED tuple (255,0,0) 
    // GREEN tuple (0,255,0)
    // BLUE tuple (0,0,255)

    //write a function which accepts char 'R' 'G' 'B'
    //ans return above specified tuple

    /*
    let res = color('R');
    println!("{:?}",res);
    let res = color('G');
    println!("{:?}",res);
    let res = color('B');
    println!("{:?}",res);
    */

    let letters = ['R','G','B'];

    /*
    for l in letters.iter(){
        let res = color(*1);
        println!("{:?}",res);
    */

    for idx in 0..letters.len(){
        let res = color(letters[idx]);
        println!("{:?}",res);

    }
}
*/
/**/
