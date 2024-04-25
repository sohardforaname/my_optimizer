use std::ops::Deref;
use sqlparser::parser::Parser;
use sqlparser::ast::{SetExpr, Statement};
use sqlparser::dialect::GenericDialect;
use crate::expr::{Identifier, Type};
use crate::expr::Expr::ColumnDef;
use crate::operator::Operator;
use crate::operator::Operator::{Project, Scan};

pub fn parse(sql: &str) -> Operator {
    let dialect = GenericDialect {};
    let statements = Parser::parse_sql(&dialect, sql).unwrap();
    assert_eq!(statements.len(), 1);
    for statement in statements.iter() {
        if let Statement::Query(query) = statement {
            if let SetExpr::Select(select, .. ) = query.body.deref() {
                return Project {
                    child: Box::from(Scan { relation: "".to_string(), schema: vec![] }),
                    projections: select.projection.iter()
                        .map(|col| (
                            ColumnDef(Identifier { names: vec![] }, Type { name: "".to_string() }),
                            col.to_string()
                        )).collect(),
                }
            }
        }
    }
    panic!()
}

#[test]
fn test_parse_select() {
    let op = parse("select * from t");
    if let Scan { relation, .. } = op {
        assert_eq!(relation, "")
    } else if let Project { projections, .. } = op {
        assert_eq!(projections[0].1, "*".to_string());
    }
}