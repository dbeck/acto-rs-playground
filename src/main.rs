extern crate acto_rs;
pub mod naked;

fn main() {
  naked::run_naked();
  naked::increase_my_bill();
  naked::trigger_me();
}
