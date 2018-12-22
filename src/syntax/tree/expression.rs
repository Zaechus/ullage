//! Syntax Expression
//!
//! A syntax expression represents the value of a given node in the
//! syntax tree.

use super::fn_builder::FunctionDeclarationBuilder;
use super::operators::{InfixOp, PrefixOp};
use super::types::TypeRef;

/// An identifier, with an optional type attached
#[derive(Debug, PartialEq)]
pub struct TypedId {
    /// The Type of this Identifier
    ///
    /// If a type was specified then this contains the type
    /// reference. If empty then the type should be inferred.
    pub typ: Option<TypeRef>,

    /// The Idnetifier Itself
    pub id: String,
}

impl TypedId {
    /// Create an Id with a Known Type
    ///
    /// Constructs a new idnetifier declaration where the identifier
    /// definitely has a known type.
    pub fn new(id: String, typ: TypeRef) -> Self {
        Self::from_parts(id, Some(typ))
    }

    /// Create an Id without a Known Type
    ///
    /// Constructs a new identifier declaraiton where the identifier
    /// does not have a type specified in the source. This is used
    /// where the type will be infered at a later date.
    pub fn new_without_type(id: String) -> Self {
        Self::from_parts(id, None)
    }

    /// Create an Id from Constituent Parts
    ///
    /// Used to construct a new identifier when a type has only
    /// optionally been specified.
    pub fn from_parts(id: String, typ: Option<TypeRef>) -> Self {
        TypedId { id: id, typ: typ }
    }
}

/// Literal / Constant Value
#[derive(Debug, PartialEq)]
pub enum Constant {
    /// A Numeric Value
    Number(i64),
    /// A Boolean Value
    Bool(bool),
    /// A Literal String
    String(String),
}

/// Represents an AST expression.
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Expression {
    Identifier(String),
    Literal(Constant),
    Prefix(PrefixOp, Box<Expression>),
    Infix(Box<Expression>, InfixOp, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    IfThenElse(Box<Expression>, Box<Expression>, Box<Expression>),
    Function(String, TypeRef, Vec<TypedId>, Box<Expression>),
    Loop(Box<Expression>, Box<Expression>),
    Sequence(Vec<Expression>),
    Print(Box<Expression>),
    Declaration(TypedId, bool, Box<Expression>),
}

impl Expression {
    /// # New Identifier Expression
    ///
    /// A reference to an identifier, either as a variable reference
    /// or declaration, part of a function definition or function
    /// call.
    pub fn identifier(s: String) -> Self {
        Expression::Identifier(s.into())
    }

    /// # New Numeric Constant
    ///
    /// A constant numeric value, either specified inline using a
    /// numeric literal or computed from other known compile-time
    /// constants.
    pub fn constant_num(n: i64) -> Self {
        Expression::Literal(Constant::Number(n))
    }

    /// # New String Constant
    ///
    /// A constant string value, either specified inline using a
    /// string literal or computed from other known compile-time
    /// constants.
    pub fn constant_string<T>(s: T) -> Self
    where
        T: Into<String>,
    {
        Expression::Literal(Constant::String(s.into()))
    }

    /// New Bool Constant
    ///
    /// A constant boolean value. Created from the literal 'true' or
    /// 'false'.
    pub fn constant_bool(b: bool) -> Self {
        Expression::Literal(Constant::Bool(b))
    }

    /// # New Prefix Operator Expression
    ///
    /// Represents the application of a prefix unary operator to
    /// another expression.
    pub fn prefix(op: PrefixOp, expr: Expression) -> Self {
        Expression::Prefix(op, Box::new(expr))
    }

    /// # New Infix Operator Expression
    ///
    /// Represents the application of an infix binary operator to two
    /// expression operands.
    pub fn infix(lhs: Expression, op: InfixOp, rhs: Expression) -> Self {
        Expression::Infix(Box::new(lhs), op, Box::new(rhs))
    }

    /// # New Function Call Expression
    ///
    /// Represents calling a given function with a numer of arguments.
    pub fn call(callee: Expression, args: Vec<Expression>) -> Self {
        Expression::Call(Box::new(callee), args)
    }

    /// # New Index Expression
    ///
    /// Represents indexing one expression by another. This could be
    /// an array lookup, or slice operation.
    pub fn index(lhs: Expression, index: Expression) -> Self {
        Expression::Index(Box::new(lhs), Box::new(index))
    }

    /// # New If Then Else Expression
    ///
    /// Represents either a single conditional expression, or a
    /// ternary expression.
    pub fn if_then_else(iff: Expression, then: Expression, els: Expression) -> Self {
        Expression::IfThenElse(Box::new(iff), Box::new(then), Box::new(els))
    }

    /// # New Function Definition
    ///
    /// Create a function delcaration builder. This can be used to
    /// create a function expression.
    pub fn function(id: String) -> FunctionDeclarationBuilder {
        FunctionDeclarationBuilder::new(id)
    }

    /// # New Loop Expression
    ///
    /// Represents the repeated evaluation of an expression until a
    /// condition changes.
    pub fn loop_while(condition: Expression, body: Vec<Expression>) -> Self {
        let body = Expression::sequence(body);
        Expression::Loop(Box::new(condition), Box::new(body))
    }

    /// # New Variable Declaration
    ///
    /// Represents the declaration of a local variable.
    pub fn declaration(var: TypedId, is_mut: bool, expr: Expression) -> Self {
        Expression::Declaration(var, is_mut, Box::new(expr))
    }

    /// # New Sequence Expression
    ///
    /// Represents a sequence of expressions evaluated one after the
    /// other.
    pub fn sequence(exprs: Vec<Expression>) -> Self {
        Expression::Sequence(exprs)
    }

    /// # Print Expression
    ///
    /// Evaluates an inner expression, prints it to standard output,
    /// and then returns the inner expression's value.
    pub fn print(expr: Expression) -> Self {
        Expression::Print(Box::new(expr))
    }
}