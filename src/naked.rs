use acto_rs::*;
use std::{thread, time};

// a very simple task with a counter only
struct NakedTask {
  // to help the Task trait impl
  name : String,
  // state
  count : usize,
}

impl Task for NakedTask {
  // execute() runs 3 times and after it sets the stop flag
  // which tells the scheduler, not to execute this task anymore
  fn execute(&mut self, stop: &mut bool) {
    self.count += 1;
    println!("- {} #{}", self.name, self.count);
    if self.count == 3 {
      // three is enough
      *stop = true;
    }
  }

  fn name(&self) -> &String { &self.name }

  // zero / None values, since NakedTask has
  // no input or output channels
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
  // - create a scheduler
  // - add a recurring task
  // - stop the scheduler after 4 seconds
  let mut sched = scheduler::new();
  sched.start();
  sched.add_task(
    Box::new(NakedTask{name:String::from("RunningNaked"), count:0}),
    SchedulingRule::Periodic(PeriodLengthInUsec(1_000_000))).unwrap();
  thread::sleep(time::Duration::from_secs(4));
  sched.stop();
}

pub fn increase_my_bill() {
  let mut sched = scheduler::new();
  sched.start();
  sched.add_task(
    Box::new(NakedTask{name:String::from("IncreaseBill"), count:0}),
    SchedulingRule::Loop).unwrap();
  thread::sleep(time::Duration::from_secs(1));
  sched.stop();
}

pub fn trigger_me() {
  let mut sched = scheduler::new();
  sched.start();
  let task_id = sched.add_task(
    Box::new(NakedTask{name:String::from("TriggerMe"), count:0}),
    SchedulingRule::OnExternalEvent).unwrap();
  // notify(..) wakes up the task identified by task_id
  sched.notify(&task_id).unwrap();
  sched.stop();
}
