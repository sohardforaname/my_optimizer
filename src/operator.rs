use crate::expr::{ColumnDefinition, Expr};

#[derive(Clone)]
pub enum Operator {
    Scan {
        relation: String,
        schema: Vec<ColumnDefinition>
    },
    Filter {
        child: Box<Self>,
        predicate: Expr
    },
    Project {
        child: Box<Self>,
        projections: Vec<(Expr, String)>
    },
    Join {
        join_type: String,
        condition: Expr,
        left: Box<Self>,
        right: Box<Self>
    },
    Agg {
        child: Box<Self>,
        group: Vec<Expr>,
        agg_expr: Vec<Expr>
    },
    OrderBy {
        child: Box<Self>,
        order_keys: Vec<(Expr, bool)>
    },
    Limit {
        child: Box<Self>,
        limit: i32,
        offset: i32
    }
}

impl Operator {
    pub fn children(&self) -> Vec<Operator> {
        match self {
            Operator::Scan { .. }=> vec![],
            Operator::Filter { child, .. } => vec![(*child.as_ref()).clone()],
            Operator::Project{ child, .. } => vec![(*child.as_ref()).clone()],
            _ => panic!("unresolved operator")
        }
    }

    pub fn calculate_attribute(&self, children_attribute: &[Vec<ColumnDefinition>])
        -> Vec<ColumnDefinition> {
        match self {
            Operator::Scan { schema, .. } => (*schema).clone(),
            Operator::Filter { .. } => children_attribute[0].clone(),
            Operator::Project { projections, .. } => projections.clone().iter()
                .map(|(col, alias)| ColumnDefinition {
                    column_type: (*col.get_type()).clone(),
                    is_not_null: false,
                    name: alias.clone()
                }).collect(),
            Operator::Join { .. } => {
                let mut attributes = children_attribute[0].clone();
                attributes.extend(children_attribute[1].clone());
                attributes
            }
            _ => panic!()
        }
    }
}