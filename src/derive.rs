use crate::expr::ColumnDefinition;
use crate::operator::Operator;

pub struct Property;

fn derive_attributes(op: &Operator) -> Vec<ColumnDefinition> {
    let children_attributes: Vec<Vec<ColumnDefinition>> = op.children().iter()
        .map(derive_attributes)
        .collect();

    op.calculate_attribute(children_attributes.as_slice())
}
