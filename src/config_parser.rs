use std::fs;
use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "config.pest"]
struct CONFIGParser;
pub fn parser_config(unparsed_config_path: &str) -> HashMap<String, String> {
    let mut clomeaste_config: HashMap<String, String> = HashMap::from([
        ("backslash_primitive".to_string(), "\\textbackslash".to_string()),
        ("percent_primitive".to_string(), "\\%".to_string()),
        ("sharp_primitive".to_string(), "\\#".to_string()),
        ("tilde_primitive".to_string(), "\\textasciitilde".to_string()),
        ("toc_primitive".to_string(), "\\tableofcontents".to_string()),
        ("before_bold".to_string(), "\\textbf{".to_string()),
        ("after_bold".to_string(), "}".to_string()),
        ("before_italic".to_string(), "\\textit{".to_string()),
        ("after_italic".to_string(), "}".to_string()),
        ("before_highlight".to_string(), "".to_string()),
        ("after_highlight".to_string(), "".to_string()),
        ("before_delete_line".to_string(), "".to_string()),
        ("after_delete_line".to_string(), "".to_string()),
        ("before_inline_math".to_string(), "$".to_string()),
        ("after_inline_math".to_string(), "$".to_string()),
        ("before_inline_code".to_string(), "\\verb|".to_string()),
        ("after_inline_code".to_string(), "|".to_string()),
        ("before_title".to_string(), "\\title{".to_string()),
        ("after_title".to_string(), "}".to_string()),
        ("before_author".to_string(), "\\author{".to_string()),
        ("after_author".to_string(), "}".to_string()),
        ("before_date".to_string(), "\\date{".to_string()),
        ("after_date".to_string(), "}".to_string()),
        ("before_abstract".to_string(), "\\begin{abstract}".to_string()),
        ("after_abstract".to_string(), "\\end{abstract}".to_string()),
        ("before_enumerate".to_string(), "\\begin{enumerate}".to_string()),
        ("after_enumerate".to_string(), "\\end{enumerate}".to_string()),
        ("before_itemize".to_string(), "\\begin{itemize}".to_string()),
        ("after_itemize".to_string(), "\\end{itemize}".to_string()),
        ("before_display_math".to_string(), "\\[".to_string()),
        ("after_display_math".to_string(), "\\]".to_string()),
        ("before_display_math_2".to_string(), "".to_string()),
        ("after_display_math_2".to_string(), "".to_string()),
        ("code_block".to_string(), "minted".to_string()),
        ("before_title_level_1".to_string(), "\\section{".to_string()),
        ("after_title_level_1".to_string(), "}".to_string()),
        ("before_title_level_2".to_string(), "\\subsection{".to_string()),
        ("after_title_level_2".to_string(), "}".to_string()),
        ("before_title_level_3".to_string(), "\\subsubsection{".to_string()),
        ("after_title_level_3".to_string(), "}".to_string()),
        ("before_title_level_4".to_string(), "\\paragraph{".to_string()),
        ("after_title_level_4".to_string(), "}".to_string()),
        ("before_title_level_5".to_string(), "".to_string()),
        ("after_title_level_5".to_string(), "".to_string()),
        ("before_title_level_6".to_string(), "".to_string()),
        ("after_title_level_6".to_string(), "".to_string()),
        ("preamble".to_string(), "\\documentclass[a4paper]{ctexart}\n\\usepackage{minted, amsmath, amssymb}\n\\renewcommand{\\theFancyVerbLine}{\\ttfamily{\\arabic{FancyVerbLine}}}\n".to_string())
    ]);

    let unparsed_config = fs::read_to_string(unparsed_config_path);

    match unparsed_config {
        Ok(unparsed_config) => {
            let config = CONFIGParser::parse(Rule::clomeaste_config, &unparsed_config);
            match config {
                Ok(mut pair) => {
                    pair.next().unwrap().into_inner().for_each(|pair| {
                        let mut inside_iter = pair.into_inner();
                        let config_key = inside_iter.next().unwrap().as_str();
                        let config_value = inside_iter.next().unwrap().as_str();

                        clomeaste_config.insert(config_key.to_string(), config_value.to_string());
                    });
                    clomeaste_config
                }
                Err(_) => {
                    println!("解析配置文件时出现错误, 使用默认配置. ");
                    clomeaste_config
                }
            }
        }
        Err(_) => {
            println!("读取配置文件时出现错误, 使用默认配置. ");
            clomeaste_config
        }
    }
}
