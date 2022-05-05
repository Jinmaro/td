
use clap::Parser;

/// Command Line Tool to manage your To Do.
#[derive(Parser)]
#[clap(
    name = "td",
    author = "Takizawa Riki",
    version = "0.1.0",
    about = "Command Line Tool to manage your To Do."
)]

struct Options {
    #[clap(short = 'a', long = "add", value_name = "NEW_TO_DO", help = "add NEW_TO_DO to your To Do list.")]
    add: Option<String>,

    #[clap(short = 'f', long = "finish", value_name = "FINISHED_TO_DO", help = "put a check mark to FINISHED_TO_DO of your To Do list.")]
    fin: Option<String>,

    #[clap(short = 'd', long = "delete", value_name = "DELETED_TO_DO", help = "delete DELETED_TO_DO from your To Do list.")]
    del: Option<String>,

    #[clap(short = 'e', long = "edit", value_name = "EDITED_TO_DO", help = "change EDITED_TO_DO of your To Do list to NEW_TO_DO.")]
    edi: Option<String>,

    todo: String,
}

fn main() {
    let options = Options::parse();

}
