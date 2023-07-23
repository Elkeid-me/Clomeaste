use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

use crate::ast_def::ASTNode;

#[derive(Parser)]
#[grammar = "tml.pest"]
struct TMLParser;

pub fn parse_tml(document: &str) -> std::result::Result<ASTNode, Error<Rule>> {
    let tml_document = TMLParser::parse(Rule::document, document)?.next().unwrap();

    fn parse_ast_node(pair: Pair<Rule>) -> ASTNode {
        match pair.as_rule() {
            Rule::star_primitive => ASTNode::StarPrimitive,
            Rule::dollar_primitive => ASTNode::DollarPrimitive,
            Rule::backslash_primitive => ASTNode::BackslashPrimitive,
            Rule::percent_primitive => ASTNode::PercentPrimitive,
            Rule::tilde_primitive => ASTNode::TildePrimitive,
            Rule::sharp_primitive => ASTNode::SharpPrimitive,
            Rule::toc_primitive => ASTNode::TOC,

            Rule::direct_tex => ASTNode::DirectTeX(pair.into_inner().as_str()),
            Rule::shell_escape => {
                ASTNode::ShellEscape(pair.into_inner().map(|pair| pair.as_str()).collect())
            }

            Rule::inline_math => {
                ASTNode::InlineMath(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::bold => {
                ASTNode::Bold(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::italic => {
                ASTNode::Italic(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::highlight => {
                ASTNode::Highlight(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::delete_line => {
                ASTNode::DeleteLine(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::plain_text => ASTNode::PlainText(pair.as_str()),
            Rule::inline_code => ASTNode::InlineCode(pair.into_inner().as_str()),
            Rule::inline_math_code => ASTNode::InlineMathCode(pair.as_str()),
            Rule::display_math_code => ASTNode::DisplayMathCode(pair.as_str()),
            Rule::code_line => ASTNode::CodeLine(pair.as_str()),

            Rule::para => {
                ASTNode::Paragraph(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }

            Rule::doc_abstract => {
                ASTNode::Abstract(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::code_block => {
                let mut iter = pair.into_inner();
                let language = iter.next().unwrap().as_str();
                let lines = iter.map(|pair| parse_ast_node(pair)).collect();
                ASTNode::CodeBlock { language, lines }
            }
            Rule::list_item => {
                ASTNode::ListItem(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::display_math => {
                ASTNode::DisplayMath(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::display_math_2 => {
                ASTNode::DisplayMath2(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::enumerate => {
                ASTNode::Enumerate(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::itemize => {
                ASTNode::Itemize(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::front_matter => {
                ASTNode::FrontMatter(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::title_lvl_1 => {
                ASTNode::TitleLevel1(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::title_lvl_2 => {
                ASTNode::TitleLevel2(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::title_lvl_3 => {
                ASTNode::TitleLevel3(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::title_lvl_4 => {
                ASTNode::TitleLevel4(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::title_lvl_5 => {
                ASTNode::TitleLevel5(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::title_lvl_6 => {
                ASTNode::TitleLevel6(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            Rule::document => {
                ASTNode::Document(pair.into_inner().map(|pair| parse_ast_node(pair)).collect())
            }
            _ => ASTNode::InlineCode("Error"),
        }
    }

    Ok(parse_ast_node(tml_document))
}
