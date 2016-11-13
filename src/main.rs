extern crate acto_rs;
pub mod naked;
pub mod greet_int;

fn main() {
  // naked actors
  naked::run_naked();
  naked::increase_my_bill();
  naked::trigger_me();

  // greet integers
  greet_int::greet_five();
}
