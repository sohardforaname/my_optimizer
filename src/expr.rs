use crate::operator::Operator;

#[derive(Clone)]
pub struct Type {
    pub name: String
}

#[derive(Clone)]
pub struct Identifier {
    pub names: Vec<String>
}

#[derive(Clone)]
pub struct Signature {
    pub args: Vec<Type>,
    pub ret: Type
}

#[derive(Clone)]
pub enum Expr {
    ColumnDef(Identifier, Type),
    Literal(String, Type),
    Function(Signature, Box<Self>),
    Subquery(String, Box<Operator>),
    BinaryOp(String, Box<Self>, Box<Self>),
    UnaryOp(String, Box<Self>)
}

#[derive(Clone)]
pub struct ColumnDefinition {
    pub column_type: Type,
    pub is_not_null: bool,
    pub name: String
}

impl Expr {
    pub fn get_type(&self) -> &Type {
        match self {
            Expr::ColumnDef(_, col_type) => col_type,
            Expr::Literal(_, literal_type) =>  literal_type,
            Expr::Function(signature, _) => &signature.ret,
            _ => todo!()
        }
    }
}