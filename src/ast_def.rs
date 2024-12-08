use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::Command;

pub enum ASTNode<'a> {
    StarPrimitive,
    DollarPrimitive,
    BackslashPrimitive,
    PercentPrimitive,
    TildePrimitive,
    SharpPrimitive,
    TOC,

    Bold(Vec<ASTNode<'a>>),
    Italic(Vec<ASTNode<'a>>),
    Highlight(Vec<ASTNode<'a>>),
    DeleteLine(Vec<ASTNode<'a>>),
    InlineMath(Vec<ASTNode<'a>>),
    InlineMathCode(&'a str),
    DisplayMathCode(&'a str),
    InlineCode(&'a str),
    PlainText(&'a str),
    CodeLine(&'a str),

    FrontMatter(Vec<ASTNode<'a>>),
    Paragraph(Vec<ASTNode<'a>>),
    Abstract(Vec<ASTNode<'a>>),
    CodeBlock { language: &'a str, lines: Vec<ASTNode<'a>> },
    DisplayMath(Vec<ASTNode<'a>>),
    DisplayMath2(Vec<ASTNode<'a>>),
    Enumerate(Vec<ASTNode<'a>>),
    Itemize(Vec<ASTNode<'a>>),
    DirectTeX(&'a str),
    ShellEscape(Vec<&'a str>),
    TitleLevel1(Vec<ASTNode<'a>>),
    TitleLevel2(Vec<ASTNode<'a>>),
    TitleLevel3(Vec<ASTNode<'a>>),
    TitleLevel4(Vec<ASTNode<'a>>),
    TitleLevel5(Vec<ASTNode<'a>>),
    TitleLevel6(Vec<ASTNode<'a>>),
    ListItem(Vec<ASTNode<'a>>),
    Document(Vec<ASTNode<'a>>),
}

impl ASTNode<'_> {
    pub fn dump(&self, file: &mut fs::File, config: &HashMap<String, String>) {
        match self {
            ASTNode::StarPrimitive => {
                write!(file, "*");
            }
            ASTNode::DollarPrimitive => {
                write!(file, "\\$\\ ");
            }
            ASTNode::BackslashPrimitive => {
                write!(file, "{}\\ ", config["backslash_primitive"]);
            }
            ASTNode::PercentPrimitive => {
                write!(file, "{}\\ ", config["percent_primitive"]);
            }
            ASTNode::TildePrimitive => {
                write!(file, "{}\\ ", config["tilde_primitive"]);
            }
            ASTNode::SharpPrimitive => {
                write!(file, "{}\\ ", config["sharp_primitive"]);
            }
            ASTNode::TOC => {
                write!(file, "{}", config["toc_primitive"]);
            }
            ASTNode::Bold(inside) => {
                write!(file, "{}", config["before_bold"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_bold"]);
            }
            ASTNode::Italic(inside) => {
                write!(file, "{}", config["before_italic"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_italic"]);
            }
            ASTNode::Highlight(inside) => {
                write!(file, "{}", config["before_highlight"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_highlight"]);
            }
            ASTNode::DeleteLine(inside) => {
                write!(file, "{}", config["before_delete_line"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_delete_line"]);
            }
            ASTNode::InlineMath(inside) => {
                write!(file, "{}", config["before_inline_math"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_inline_math"]);
            }
            ASTNode::InlineMathCode(s) => {
                write!(file, "{}", s);
            }
            ASTNode::DisplayMathCode(s) => {
                write!(file, "{}", s);
            }
            ASTNode::InlineCode(s) => {
                write!(
                    file,
                    "{}{}{}",
                    config["before_inline_code"], s, config["after_inline_code"]
                );
            }
            ASTNode::PlainText(s) => {
                write!(file, "{}", s);
            }
            ASTNode::CodeLine(s) => {
                write!(file, "{}", s);
            }

            ASTNode::FrontMatter(inside) => {
                write!(file, "{}", config["before_title"]);
                inside[0].dump(file, config);
                writeln!(file, "{}", config["after_title"]);
                write!(file, "{}", config["before_author"]);
                inside[1].dump(file, config);
                writeln!(file, "{}", config["after_author"]);
                write!(file, "{}", config["before_date"]);
                inside[2].dump(file, config);
                writeln!(file, "{}", config["after_date"]);
            }
            ASTNode::Paragraph(inside) => {
                inside.iter().for_each(|i| i.dump(file, config));
            }
            ASTNode::Abstract(inside) => {
                writeln!(file, "{}", config["before_abstract"]);
                inside.iter().for_each(|i| {
                    i.dump(file, config);
                    write!(file, "\n\n");
                });
                write!(file, "{}", config["after_abstract"]);
            }
            ASTNode::CodeBlock { language, lines } => match &config["code_block"] as &str {
                "lstlsting" => {
                    writeln!(file, "\\begin{{lstlisting}}");
                    lines.iter().for_each(|i| {
                        i.dump(file, config);
                        writeln!(file);
                    });
                    write!(file, "\\end{{lstlisting}}");
                }
                "verbatim" => {
                    writeln!(file, "\\begin{{verbatim}}");
                    lines.iter().for_each(|i| {
                        i.dump(file, config);
                        writeln!(file);
                    });
                    write!(file, "\\end{{verbatim}}");
                }
                _ => {
                    writeln!(file, "\\begin{{minted}}[linenos, frame = single]{{{}}}", language);
                    lines.iter().for_each(|i| {
                        i.dump(file, config);
                        writeln!(file);
                    });
                    write!(file, "\\end{{minted}}");
                }
            },
            ASTNode::DisplayMath(inside) => {
                writeln!(file, "{}", config["before_display_math"]);
                inside.iter().for_each(|i| {
                    i.dump(file, config);
                    writeln!(file);
                });
                write!(file, "{}", config["after_display_math"]);
            }
            ASTNode::DisplayMath2(inside) => {
                writeln!(file, "{}", config["before_display_math_2"]);
                inside.iter().for_each(|i| {
                    i.dump(file, config);
                    writeln!(file);
                });
                write!(file, "{}", config["after_display_math_2"]);
            }
            ASTNode::Enumerate(inside) => {
                writeln!(file, "{}", config["before_enumerate"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_enumerate"]);
            }
            ASTNode::Itemize(inside) => {
                writeln!(file, "{}", config["before_itemize"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_itemize"]);
            }
            ASTNode::DirectTeX(s) => {
                write!(file, "{}", s);
            }
            ASTNode::ShellEscape(s) => {
                match Command::new(s[0]).args(&s[1..]).output() {
                    Ok(output) => file.write_all(String::from_utf8(output.stdout).unwrap().trim().as_bytes()),
                    Err(_) => file.write_all("Running Error".as_bytes()),
                };
            }
            ASTNode::TitleLevel1(inside) => {
                write!(file, "{}", config["before_title_level_1"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_title_level_1"]);
            }
            ASTNode::TitleLevel2(inside) => {
                write!(file, "{}", config["before_title_level_2"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_title_level_2"]);
            }
            ASTNode::TitleLevel3(inside) => {
                write!(file, "{}", config["before_title_level_3"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_title_level_3"]);
            }
            ASTNode::TitleLevel4(inside) => {
                write!(file, "{}", config["before_title_level_4"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_title_level_4"]);
            }
            ASTNode::TitleLevel5(inside) => {
                write!(file, "{}", config["before_title_level_5"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_title_level_5"]);
            }
            ASTNode::TitleLevel6(inside) => {
                write!(file, "{}", config["before_title_level_6"]);
                inside.iter().for_each(|i| i.dump(file, config));
                write!(file, "{}", config["after_title_level_6"]);
            }
            ASTNode::ListItem(inside) => {
                write!(file, "\\item ");
                inside.iter().for_each(|i| {
                    i.dump(file, config);
                    write!(file, "\n\n");
                });
            }
            ASTNode::Document(inside) => {
                write!(file, "{}", config["preamble"]);
                let mut iter = inside.iter();
                let block_1 = iter.next().unwrap();
                match block_1 {
                    ASTNode::FrontMatter(_) => {
                        block_1.dump(file, config);
                        writeln!(file, "\\begin{{document}}");
                        writeln!(file, "\\maketitle");
                        iter.for_each(|i| {
                            i.dump(file, config);
                            write!(file, "\n\n");
                        });
                    }
                    _ => {
                        writeln!(file, "\\begin{{document}}");
                        block_1.dump(file, config);
                        write!(file, "\n\n");
                        iter.for_each(|i| {
                            i.dump(file, config);
                            write!(file, "\n\n");
                        });
                    }
                }
                writeln!(file, "\\end{{document}}");
            }
        }
    }
}
