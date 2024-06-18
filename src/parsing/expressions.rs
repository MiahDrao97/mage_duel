use crate::game_zones::types::{DamageType, Dice};

use super::{symbol_table::SymbolTable, tokens::{Token, Tokens}};
use std::{ops::Add, rc::Rc};

#[derive(Debug)]
pub enum ParseExpressionError {
    MismatchedOperands,
    InvalidOperator,
    OperandTypesNotSupported
}

pub trait Expression {
    fn get_type(&self) -> ExpressionType;
    fn evaluate(&self, symbol_table: &SymbolTable) -> ExpressionResult;
}

pub struct BinaryOperation {
    left: Box<dyn Expression>,
    operator: Tokens,
    right: Box<dyn Expression>,
}

pub struct UnaryOperation {
    operator: Tokens,
    right: Box<dyn Expression>
}

enum AdditiveOperation {
    Add,
    Subtract,
    Concatenate,
    ConcatenateUnique,
    Except
}

pub struct AdditiveExpression {
    op: BinaryOperation,
}

impl AdditiveExpression {
    
}

impl TryFrom<BinaryOperation> for AdditiveExpression {
    type Error = ParseExpressionError;

    fn try_from(value: BinaryOperation) -> Result<Self, Self::Error> {
        let operator: &str;
        if let Tokens::Symbol(ref symbol) = value.operator {
            operator = symbol.as_str();
            if operator != "+" && operator != "+!" && operator != "-" {
                return Err(ParseExpressionError::InvalidOperator);
            }
        } else {
            return Err(ParseExpressionError::InvalidOperator);
        }

        let lhs: ExpressionType;
        let rhs: ExpressionType;
        let is_list: bool;
        let mut lh_is_list = false;
        let mut rh_is_list = false;

        if let ExpressionType::List(list) = value.left.get_type() {
            lhs = *list;
            lh_is_list = true;
        } else {
            lhs = value.left.get_type();
        }

        if let ExpressionType::List(list) = value.right.get_type() {
            rhs = *list;
            rh_is_list = true;
        } else {
            rhs = value.right.get_type();
        }
        is_list = lh_is_list || rh_is_list;

        if !lh_is_list && rh_is_list {
            // this is like 7 - [ 7 ] or 7 + [ 7 ], which makes no sense
            return Err(ParseExpressionError::MismatchedOperands);
        }

        if !is_list && operator == "+!" {
            // +! is only a list operator
            return Err(ParseExpressionError::OperandTypesNotSupported);
        }

        if lhs == ExpressionType::Integer && rhs == ExpressionType::Integer { }
        else if is_list && lhs == rhs { /* either concatenating two lists or a single item to a list */ }
        else {
            // on the error path, kids
            if lhs != rhs {
                return Err(ParseExpressionError::MismatchedOperands);
            }
            return Err(ParseExpressionError::OperandTypesNotSupported);
        }

        Ok(AdditiveExpression { op: value })
    }
}

impl Expression for AdditiveExpression {
    fn evaluate(&self, symbol_table: &SymbolTable) -> ExpressionResult {
        let lhs = self.op.left.evaluate(symbol_table);
        let rhs = self.op.right.evaluate(symbol_table);

        if let ExpressionResult::Integer(l) = lhs {
            if let ExpressionResult::Integer(r) = rhs {
                return ExpressionResult::Integer(l + r);
            }
            panic!("Right-hand side did not evaluate to integer expression.")
        } else if let ExpressionResult::List(list) = lhs {
            if let ExpressionResult::List(other) = rhs {
                return ExpressionResult::List([ list, other ].concat().into());
            }
            else {
                return ExpressionResult::List([ list, [ rhs ].into() ].concat().into());
            }
        }
        // something is super messed up
        panic!("Left-hand side did not evaluate to integer or list expression.");
    }

    fn get_type(&self) -> ExpressionType {
        // TODO: List type
        ExpressionType::Integer
    }
}

pub struct UnaryExpression {
    op: UnaryOperation
}

impl TryFrom<UnaryOperation> for UnaryExpression {
    type Error = ParseExpressionError;

    fn try_from(value: UnaryOperation) -> Result<Self, Self::Error> {
        let op: &str;
        if let Tokens::Symbol(ref op_token) = value.operator {
            op = op_token.as_str();
        } else {
            return Err(ParseExpressionError::InvalidOperator);
        }
        if op != "-" && op != "~" && op != "^" {
            return Err(ParseExpressionError::InvalidOperator);
        }
        if (op == "-" || op == "^") && value.right.get_type() != ExpressionType::Integer {
            return Err(ParseExpressionError::OperandTypesNotSupported);
        }
        if op == "~" && value.right.get_type() != ExpressionType::Boolean {
            return Err(ParseExpressionError::OperandTypesNotSupported);
        }

        Ok(UnaryExpression { op: value })
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self, symbol_table: &SymbolTable) -> ExpressionResult {
        todo!();
    }

    fn get_type(&self) -> ExpressionType {
        todo!();
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExpressionType {
    Integer,
    Boolean,
    DamageType,
    Dice,
    List(Box<ExpressionType>),
    // TODO: Players and stuff
}

#[derive(Debug, Clone)]
pub enum ExpressionResult {
    Integer(i32),
    Boolean(bool),
    DamageType(DamageType),
    Dice(Dice),
    List(Rc<[ExpressionResult]>),
    // TODO: Players and stuff
}