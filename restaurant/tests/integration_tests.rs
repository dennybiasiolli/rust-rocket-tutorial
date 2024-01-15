use restaurant;

#[test]
fn it_calls_eat_at_restaurant() {
    restaurant::eat_at_restaurant();
}

#[test]
fn it_calls_add_to_waitlist() {
    restaurant::hosting::add_to_waitlist();
}
