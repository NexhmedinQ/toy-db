use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser as _;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sql.pest"]
pub struct SQLParser;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Bool(bool),
    Int(u64),
    String(String)
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Bool,
    Int,
    String,
    Record(Vec<(String, Type)>),
}

pub type Record = Vec<(String, Expr)>;

#[derive(Debug, PartialEq)]
pub enum Command {
    Select(String),
    Insert(String, Record),
    Create(String, Type),
}

pub fn parse_sql(statement: &str) -> Result<Command, Error<Rule>> {
    let sql_statement = SQLParser::parse(Rule::sql_command, statement)?.next().unwrap();
    let sql_command = sql_statement.into_inner().next().unwrap();
    match sql_command.as_rule() {
        Rule::create => {
            let mut inner = sql_command.into_inner();
            Ok(Command::Create(inner.next().unwrap().as_str().to_string(), parse_type(inner.next().unwrap()).unwrap()))
        },
        //Rule::insert => {}
        Rule::select => Ok(Command::Select(sql_command.into_inner().next().unwrap().as_str().to_string())),
        _ => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: format!("Unexpected rule {:?}, expected valid SQL command", sql_command),
            },
            sql_command.as_span(),
        )),
    }
}

pub fn parse_type(pair: Pair<Rule>) -> Result<Type, Error<Rule>> {
    let sql_type = pair.into_inner().next().unwrap();
    match sql_type.as_rule() {
        Rule::type_identifier => match sql_type.as_str() {
            "Int" => Ok(Type::Int),
            "String" => Ok(Type::String),
            "Bool" => Ok(Type::Bool),
            invalid => Err(Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: format!("Invalid type {:?}", invalid),
                },
                sql_type.as_span(),
            )),
        }
        Rule::record_type => Ok(Type::Record(parse_record(sql_type.into_inner()))),
        _ => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: format!("Unexpected type {:?}, expected valid SQL type", pair),
            },
            sql_type.as_span(),
        )),
    }
}

pub fn parse_record<E>(mut pairs: Pairs<Rule>) -> Vec<(String, E)> {
    let mut vec = Vec::new();
    while let Some(col_name) = pairs.next() {
        vec.push((col_name.as_str().to_string(), pairs.next().))
    }
    pair.into_inner().map(|pair| (std::string::String::from("mfkwe"), Type::Bool)).collect()
}