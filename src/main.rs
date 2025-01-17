use clap::{
    builder::{styling, Styles},
    Args, Parser,
};

#[derive(Parser, Debug)]
#[command(
    version,
    next_display_order = None,
    help_template = "\
{before-help}{name} {version}

{usage-heading} {usage}

{all-args}{after-help}",
    styles = Styles::styled()
        .header(styling::AnsiColor::Yellow.on_default())
        .usage(styling::AnsiColor::Yellow.on_default())
        .literal(styling::AnsiColor::Green.on_default())
)]
enum Command {
    #[command(name = "-a")]
    /// Adds the todo item
    Add(AddOptions),
    #[command(name = "-r")]
    /// Removes the todo item
    Remove(RemoveOptions),
    #[command(name = "-d")]
    /// Marks the todo item as done
    Done(DoneOptions),
}

#[derive(Args, Clone, Debug)]
struct AddOptions {
    /// Id of the todo item
    item: String,
}

#[derive(Args, Clone, Debug)]
struct RemoveOptions {
    /// Id of the todo item
    id: isize,
}

#[derive(Args, Clone, Debug)]
struct DoneOptions {
    /// Id of the todo item
    id: isize,
}

fn main() {
    let _ = match Command::parse() {
        Command::Add(options) => println!("Adds {}", options.item),
        Command::Remove(options) => println!("Removes {}", options.id),
        Command::Done(options) => println!("Marks it as done {}", options.id),
    };
}
