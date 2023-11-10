use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    #[arg(short, default_value_t = ("".to_string()))]
    equation: String
}
fn main() {
    let args = Args::parse();

    if args.equation == "".to_string() {

    }

}
