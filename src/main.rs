use clap::{Parser, ArgEnum, ArgGroup};
use dbd::bridge::dbridge::DBridge;
use dbd::enp::char_dev::CharDevRW;

#[derive(Parser)]
#[clap(version, about)]
#[clap(group(
    ArgGroup::new("file-rule")
        .multiple(true)
        .arg("file")
        .conflicts_with_all(&["device-a", "device-b", "enp-type1", "enp-type2"])
))]
#[clap(group(
    ArgGroup::new("enp-1-rule")
        .arg("device-a")
        .requires_all(&["device-b", "enp-type1"])
))]
#[clap(group(
    ArgGroup::new("enp-2-rule")
        .arg("device-b")
        .requires_all(&["device-a", "enp-type2"])
))]
/// Device Bridge Daemon (dbd)
struct Cli {
    #[clap(short = 'a', long, forbid_empty_values = true, value_name = "DEV", value_parser)]
    /// Device A
    device_a: String,

    #[clap(long, arg_enum, value_parser)]
    enp_type1: EnpType,

    #[clap(short = 'b', long, forbid_empty_values = true, value_name = "DEV", value_parser)]
    /// Device B
    device_b: String,

    #[clap(long, arg_enum, value_parser)]
    enp_type2: Option<EnpType>,

    #[clap(long, arg_enum, value_parser)]
    file: Option<String>,

    /// Turn debugging information on
    #[clap(short, long, action)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum EnpType {
    File,
}

fn main() {
    let args = Cli::parse();

    let enp1 = CharDevRW::new(&args.device_a);
    let enp2 = CharDevRW::new(&args.device_b);

    #[allow(unused_variables)]
    let bridge = DBridge::new(enp1, enp2);
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
