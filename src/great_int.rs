use acto_rs::*;


struct SendGreatingsActor {
  last_sent: usize,
}

impl source::Source for SendGreatingsActor {

  type OutputValue = usize;
  type OutputError = String;

  fn process(&mut self,
             output: &mut Sender<Message<Self::OutputValue, Self::OutputError>>,
             _stop: &mut bool)
  {
    output.put(|value| *value = Some(Message::Value(self.last_sent)) );
    self.last_sent += 1;
  }
}

struct PrintGreatingSumActor {
  sum_received: usize,
}

impl sink::Sink for PrintGreatingSumActor {

  type InputValue = usize;
  type InputError = String;

  fn process(&mut self,
             input: &mut ChannelWrapper<Self::InputValue, Self::InputError>,
             _stop: &mut bool)
  {
    if let &mut ChannelWrapper::ConnectedReceiver(ref mut _channel_id,
                                                  ref mut receiver,
                                                  ref mut _sender_name) = input
    {
      for m in receiver.iter() {
        match m {
          Message::Value(val) => {
            self.sum_received += val;
            println!("Hello {}, welcome. Sum is {}", val, self.sum_received);
          }
          Message::Error(position, err) => {
            println!("Error: {:?} at position: {:?}",err, position);
          }
          _ => {}
        }
      }
    }
  }
}

pub fn great_five() {
  use acto_rs::connectable::Connectable;

  let mut sched = scheduler::new();
  sched.start();

  let greater_queue_size = 2_000;
  let (greater_task, mut greater_output) =
    source::new( "SendGreatings",
                 greater_queue_size,
                 Box::new(SendGreatingsActor{last_sent:0}));;

  let mut printer_task =
    sink::new( "PrintGreatingAndSum",
               Box::new(PrintGreatingSumActor{sum_received:0}));

  printer_task.connect(&mut greater_output).unwrap();

  let greater_id = sched.add_task(greater_task, SchedulingRule::OnExternalEvent).unwrap();
  let _printer_id = sched.add_task(printer_task, SchedulingRule::OnMessage);

  sched.notify(&greater_id).unwrap();

  sched.stop();
}
