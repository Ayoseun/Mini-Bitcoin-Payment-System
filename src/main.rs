mod order;
mod product;
mod utils;

use order::place_order;
use product::show_products;
use inquire::Select;


fn main() {
    println!("-----------------------------------");
    println!("Welcome to mini-bitpay! by Ayoseun");
    println!("-----------------------------------");
    

    menu();
}

fn menu(){
    println!("Enter your choice: ");

    
    let options = vec!["Place order",
                                 "See Product List",
                                 "Exit"];

    let input = Select::new("Menu:", options.clone()).prompt();

    match input {
        Ok(input)=> {
            if input.eq(options[0]) {
                place_order();
            }

            else if input.eq(options[1]){
                show_products();
            }

            else if input.eq(options[2]) {
                return;
                
            }

        },

        Err(err) => {
            println!("Err while reading choice: {}", err )
        },


    }
}

