use clap::builder::styling::{AnsiColor, Color, Style};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "OvO", version, author)]
#[command(about = "OvO - A lightweight JavaScript runtime.", long_about = None)]
#[command(styles=new_styles())]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  /// Create an empty OvO project.
  #[command()]
  New {
    /// Project name
    name: String,
  },

  /// Add a dependency
  #[command()]
  Add {
    /// Package name
    name: String,
  },

  /// Remove a dependency
  #[command()]
  Remove {
    /// Package name
    name: String,
  },

  /// Run a file
  #[command()]
  Run {
    /// Path to the file
    path: Option<String>,
  },

  /// Run tests
  #[command()]
  Test {},

  /// Compile a file
  #[command()]
  Compile {
    /// Path to the file
    path: Option<String>,
  },

  /// Serve an ES module
  ///
  /// The module should have at least a `fetch` function exported.
  #[command()]
  Serve {
    /// Path to the module
    path: Option<String>,
  },
}

fn main() {
  let cli = Cli::parse();
  match cli.command {
    Command::New { name } => todo!(),
    Command::Add { name } => todo!(),
    Command::Remove { name } => todo!(),
    Command::Run { path } => todo!(),
    Command::Test {} => todo!(),
    Command::Compile { path } => todo!(),
    Command::Serve { path } => todo!(),
  }
}

fn new_styles() -> clap::builder::Styles {
  clap::builder::Styles::styled()
    .literal(
      Style::new()
        .bold()
        .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
    )
    .usage(Style::new().bold())
    .header(Style::new().bold())
}
