/*
 Convert temperatures between Fahrenheit and Celsius. 
 My Command-line task by G-Time on Rusty-morning
Next challenge is to ask the user to exit after completion 
 or ask user to press 0 to go back to previous menu and to modulize the functions
*/


use std::io;

pub fn game(){
    authenticate()
}

// getting celcius
fn degree_celcuis(celcius:f32){
    println!("++::converting from degrees to fahrenheit");
    println!("++::type the number of degrees");
    let fehrenheit:f32 = 32.0;
    let formula:f32 = (celcius*1.8)+fehrenheit;
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
    println!("Answer>> The temperature for {}°© is {}F",celcius, formula);
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
}

// getting fehrenhiet
fn fehrenhiets(fehrenhiet:f32){
    let formula:f32 = (fehrenhiet-32.0)*1.8;
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
    println!("Answer>> The temperature for {}F is {}°©",fehrenhiet, formula);
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
}


// getting input from the user
fn user_input()->f32{    
    let mut _input = String::new();
 //checking for correct input
    loop{
        //refreshing the input field to accept new data::lesson from G-Time
        _input = "".to_string();
    //the input area
        io::stdin()
            .read_line( &mut _input)
            .expect("error");
        match _input.trim().parse::<f32>() {
    //the value parameter matches the input parser
            Ok(value)=>{
                return value
            }
    //error checker, >>error::1 = line 113 gives err with this type ['Err(_)']
            Err(_) => { 
                println!("========INVALID INPUT=========");
                println!(":::::type an integer");
                println!("==============================");
                continue
            }
        }
    };
    
}

fn authenticate(){
    println!("==========SELECT FROM THE LIST===========");
    println!("1: to convert celsius to fahrenheit");
    println!("2: to convert fehrenheit to celcius");
    println!("=========================================");
    let checker = user_input();
/*
    my error here from the loop is, it looks like the checker flooting
    value is depicated, so if there is the need to convert it. >>error::2
*/

    // cheching the user input and prompting back for verification
    loop{
        match checker {
            //check if user presses one
            1.0 => {
            println!("===========YOU SELECTED ONE==================");
            println!("::converting from celcius to fahrenheit");
            println!("::type the number of degrees");
            println!("=============================================");
            degree_celcuis(user_input());
                break;
        },
         //check if user presses two
         2.0 => { 
            println!("============YOU SELECTED TWO==================");   
            println!("::converting from fahrenheit to celcius");
            println!("::type the number of fahrenheit");
            println!("==============================================");
            fehrenhiets(user_input());
            break;
            },
            //check if user input value is more than two
            checker if checker > 2.0=>{
                println!("_________________________________");
                    println!("Check your input");
                    authenticate();
                    break;
            },
            //check if user input value is less than one
            checker if checker < 1.0 =>{
                println!("_________________________________");
                    println!("Check your input");
                    authenticate();
                    break;
            },
/*
==>check if the number is in range between 1 and 2 >>error::4
checker if checker == 1.0..1.9=>{
    println!("_________________________________");
        println!("Check your input");
        authenticate();
        break;
},
*/

//this error checker is different from the one from >>error::3 == line 46
           _=> {},
        }
    }
}