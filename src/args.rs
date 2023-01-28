use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
        /// Installation directory
        #[clap(short, long, default_value = "xlabs")]
        pub directory: String,

         /// Download launcher assets
        #[clap(short, long)]
        pub launcher: bool,
}

pub fn get() -> Args {
    let mut args = Args::parse();
    args.directory = args.directory.replace('"', "");
    args
}