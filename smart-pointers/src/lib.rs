mod box_examples;
mod deref_traits;
mod references;

pub fn run() {
    box_examples::box1();
    box_examples::box2();

    deref_traits::traits1();
    deref_traits::traits2();
    deref_traits::traits3();
    deref_traits::traits4_unrelated();
    deref_traits::traits5();

    deref_traits::cleanup1();
    deref_traits::cleanup2();

    references::example1();
    references::example2();
    references::example3();
    references::example4_unrelated();
    references::example5();
}
