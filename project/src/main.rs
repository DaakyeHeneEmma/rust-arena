
#[derive(Clone,Debug)]

struct Set(Box<i32>, Box<i32>);

#[derive(Debug,Clone)]
struct Car {
    model: String,
    name: String
}

 macro_rules! cars {
     ($car:ident) => {
         fn $car(){
            println!( "carz {}", stringify!($car));
         }
     };
 } 

//  pub fn person(a::i32)-> Result<i32>{

//  }

 cars!(year);
 cars!(month);
 cars!(day);

fn main(){
    year();
    month();
    day();

    // let cordinates = Set(Box::new(1), Box::new(2));
    // println!("a set of numbers {:?} ",cordinates);

    // let new_car = Car{model: String::from("toyota"), name: String::from("corolla")};
    // println!("the car is {:?} ", new_car);
    // println!("the car is {} ", new_car.name);
    
}
