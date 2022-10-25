mod front_of_house {
    pub mod hosting {
        
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
  

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //absolute path 
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //relative path to access deliver_oder() by use of super keyword
        super::deliver_order();
    }
    fn cook_order() {}

}