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

pub use crate::front_of_house::hosting;

mod customer {
    pub fn eating_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

pub fn eating_at_restaurant() {
    use crate::back_of_house::Breakfast;
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
