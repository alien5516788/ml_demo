pub mod regression;
pub mod reinforcement_learning;
pub mod l1_dynamic_programming;


fn main() {
    // regression::regression::run();
    l1_dynamic_programming::value_iteration::run()
}
