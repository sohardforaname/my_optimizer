use crate::expr::ColumnDefinition;

pub struct Schema {
    pub name: String,
    pub columns: Vec<ColumnDefinition>
}
