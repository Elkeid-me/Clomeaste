use std::{env, fs, io::Result, process};

mod ast_def;
mod config_parser;
mod document_parser;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("参数数目错误.");
        process::exit(1);
    }
    let config = config_parser::parser_config("clomeaste_config.clcfg");
    let unparsed_tml = fs::read_to_string(&args[1])?;
    let document = document_parser::parse_tml(&unparsed_tml).expect("解析文档时出现错误.");
    let mut file = fs::File::create(&args[2])?;
    document.dump(&mut file, &config);
    Ok(())
}
