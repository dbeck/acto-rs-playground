extern crate acto_rs;
use acto_rs::*;
use std::{thread, time};

struct NakedTask {
  // to help the Task trait impl
  name : String,
  // state
  count : usize,
}

impl Task for NakedTask {
  fn execute(&mut self, stop: &mut bool) {
    self.count += 1;
    println!("Running naked #{}", self.count);
    if self.count == 3 {
      // three is enough
      *stop = true;
    }
  }

  fn name(&self) -> &String { &self.name }
  fn input_count(&self) -> usize { 0 }
  fn output_count(&self) -> usize { 0 }

  fn input_id(&self, _ch_id: ReceiverChannelId)
    -> Option<(ChannelId, SenderName)> { None }

  fn input_channel_pos(&self, _ch_id: ReceiverChannelId)
    -> ChannelPosition { ChannelPosition(0) }

  fn output_channel_pos(&self, _ch_id: SenderChannelId)
    -> ChannelPosition { ChannelPosition(0) }
}

pub fn run_naked() {
  let mut sched = scheduler::new();
  sched.start();
  sched.add_task(
    Box::new(NakedTask{name:String::from("RunningNaked"), count:0}),
    SchedulingRule::Periodic(PeriodLengthInUsec(1_000_000))).unwrap();
  thread::sleep(time::Duration::from_secs(4));
  sched.stop();
}
