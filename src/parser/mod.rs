//use crate::Rule;


use pest::Parser;

#[derive(Parser)]
#[grammar = "./parser/grammar/ident.pest"]
struct IdentParser;

pub fn test(){
    let pairs = IdentParser::parse(Rule::ident_list, "a1 b2").unwrap_or_else(|e| panic!("{}", e));

}