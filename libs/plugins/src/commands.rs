
pub trait Command: Send + Sync {
  fn name(&self) -> &'static str;
  fn execute<'a>(&self, args: &[&'a str]) -> Box<dyn std::future::Future<Output = ()> + Send + 'a>;
}

impl NodiumView for NodiumViewConsole {
  fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
      // Register default commands
      self.register_command(HelpCommand);
      self.register_command(ExitCommand);

      // clear console and print welcome message as asci art
      self.handle_clear();

      // start the console loop as async task
      let rt = tokio::runtime::Runtime::new().unwrap();
      rt.block_on(self.run_loop());

      Ok(())
  }

  // ...
}

struct HelpCommand;
impl Command for HelpCommand {
  fn name(&self) -> &'static str {
      "help"
  }

  fn execute<'a>(&self, args: &[&'a str]) -> Box<dyn std::future::Future<Output = ()> + Send + 'a> {
      Box::new(async move {
          println!("Nodium help");
          println!("Commands:");
          println!("list - list all plugins");
          println!("version - show nodium version");
          println!("help - show this help");
          println!("reload - reload plugins");
          println!("clear - clear console");
          println!("exit - exit nodium");
      })
  }
}

struct ExitCommand;
impl Command for ExitCommand {
  fn name(&self) -> &'static str {
      "exit"
  }

  fn execute<'a>(&self, args: &[&'a str]) -> Box<dyn std::future::Future<Output = ()> + Send + 'a> {
      Box::new(async move {
          // Add exit logic here if needed
      })
  }
}
