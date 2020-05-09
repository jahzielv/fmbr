use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    // println!("Hello, world!");
    let args = Cli::from_args();
    println!("args: {} {}", args.pattern, args.path.display());
    let content = std::fs::read_to_string(&args.path).expect("didn't find that boi");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
