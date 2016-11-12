extern crate acto_rs;
pub mod naked;
pub mod great_int;

fn main() {
  // naked actors
  naked::run_naked();
  naked::increase_my_bill();
  naked::trigger_me();

  // great integers
  great_int::great_five();
}
