extern crate clap;
use clap::{App, AppSettings};

mod cmd;

fn main() {
    let app = App::new("Omics Tool Suite - VCF Utility")
        .version("1.0")
        .setting(AppSettings::GlobalVersion)
        .author("Jingcheng Yang <yjcyxky@163.com>")
        .about("A suite of programs for interacting with vcf file.");

    // You can add more subcommands on it.
    let subcommand = app.subcommand(cmd::filter::subcommand());

    let matches = subcommand.get_matches();

    if let Some(matches) = matches.subcommand_matches(cmd::filter::COMMAND_NAME) {
        cmd::filter::run(matches);
    }
}
