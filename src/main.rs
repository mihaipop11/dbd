use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(version, about)]
/// Device Bridge Daemon (dbd)
struct Cli {
    #[clap(short = 'a', long, forbid_empty_values = true, value_name = "DEV")]
    /// Device A
    device_a: String,

    #[clap(short = 'b', long, forbid_empty_values = true, value_name = "DEV")]
    /// Device B
    device_b: String,

    /// Turn debugging information on
    #[clap(short, long, action)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}

#[cfg(test)]
mod tests {
    use crate::Cli;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}