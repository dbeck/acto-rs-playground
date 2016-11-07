extern crate acto_rs;
pub mod naked;

fn first() {
  use acto_rs::scheduler;
  let mut sched = scheduler::new();
  sched.start_with_threads(4);
  sched.stop();
}

fn main() {
  first();
  println!("Hello, world!");
  naked::run_naked();
}
