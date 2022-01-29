mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
        pub fn seat_at_table(){}
    }

    mod serving {
        fn taker_order(){}
        fn server_order(){}
        fn take_payment(){}
    }

}

pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
}

fn server_order() {}

mod back_of_house {

    pub enum Appetizer {
        Soup , 
        Salad, 
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String ,  
    }

    impl Breakfast {
        pub fn summmer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from((toast)),
                seasonal_fruit: String::from("peach")
            }
        }
    }
    fn fix_incorrect_order(){
        cook_order();
        super::server_order();
    }
    fn cook_order(){}
}
