use acto_rs::*;


struct SendGreetingsActor {
  last_sent: usize,
}

impl source::Source for SendGreetingsActor {

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

struct PrintGreetingSumActor {
  sum_received: usize,
}

impl sink::Sink for PrintGreetingSumActor {

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

pub fn greet_five() {
  use acto_rs::connectable::Connectable;

  let mut sched = scheduler::new();
  sched.start();

  let greeter_queue_size = 2_000;
  let (greeter_task, mut greeter_output) =
    source::new( "SendGreetings",
                 greeter_queue_size,
                 Box::new(SendGreetingsActor{last_sent:0}));;

  let mut printer_task =
    sink::new( "PrintGreetingAndSum",
               Box::new(PrintGreetingSumActor{sum_received:0}));

  printer_task.connect(&mut greeter_output).unwrap();

  let greeter_id = sched.add_task(greeter_task, SchedulingRule::OnExternalEvent).unwrap();
  let _printer_id = sched.add_task(printer_task, SchedulingRule::OnMessage);

  sched.notify(&greeter_id).unwrap();

  sched.stop();
}
