use lang_c::ast::{
    BinaryOperator, Constant, Expression as CExpression, FloatFormat, IntegerBase, IntegerSize,
    UnaryOperator,
};
use proc_macro2::{Ident, Literal as RustLiteral};
use quote::format_ident;
use std::{
    num::{ParseFloatError, ParseIntError},
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Debug, Clone)]
pub enum Literal {
    Int32(i32),
    Int64(i64),
    UnsignedInt32(u32),
    UnsignedInt64(u64),
    Float32(f32),
    Float64(f64),
}

impl Literal {
    pub fn value(&self) -> RustLiteral {
        match *self {
            Literal::Int32(val) => RustLiteral::i32_unsuffixed(val),
            Literal::Int64(val) => RustLiteral::i64_unsuffixed(val),
            Literal::UnsignedInt32(val) => RustLiteral::u32_unsuffixed(val),
            Literal::UnsignedInt64(val) => RustLiteral::u64_unsuffixed(val),
            Literal::Float32(val) => RustLiteral::f32_unsuffixed(val),
            Literal::Float64(val) => RustLiteral::f64_unsuffixed(val),
        }
    }

    pub fn ty(&self) -> Ident {
        match self {
            // Most constants are not defined to be unsigned while they should be
            Literal::Int32(val) if !val.is_negative() => format_ident!("u32"),
            Literal::Int32(_) => format_ident!("i32"),
            Literal::Int64(_) => format_ident!("i64"),
            Literal::UnsignedInt32(_) => format_ident!("u32"),
            Literal::UnsignedInt64(_) => format_ident!("u64"),
            Literal::Float32(_) => format_ident!("f32"),
            Literal::Float64(_) => format_ident!("f64"),
        }
    }
}

impl From<&Constant> for Literal {
    fn from(constant: &Constant) -> Self {
        match constant {
            Constant::Integer(int) => {
                let num = &int.number;
                let radix = match int.base {
                    IntegerBase::Decimal => 10,
                    IntegerBase::Hexadecimal => 16,
                    IntegerBase::Octal => 8,
                    IntegerBase::Binary => 2,
                };

                let value = || -> Result<_, ParseIntError> {
                    let value = match (int.suffix.unsigned, int.suffix.size != IntegerSize::Int) {
                        (false, false) => Literal::Int32(i32::from_str_radix(num, radix)?),
                        (false, true) => Literal::Int64(i64::from_str_radix(num, radix)?),
                        (true, false) => Literal::UnsignedInt32(u32::from_str_radix(num, radix)?),
                        (true, true) => Literal::UnsignedInt64(u64::from_str_radix(num, radix)?),
                    };

                    Ok(value)
                };

                match value() {
                    Ok(value) => value,
                    Err(err) => panic!("Failed to parse integer value: {:?} from {:?}", err, int),
                }
            }
            Constant::Float(float) => {
                let value = || -> Result<_, ParseFloatError> {
                    let value = match float.suffix.format != FloatFormat::Float {
                        false => Literal::Float32(float.number.parse()?),
                        true => Literal::Float64(float.number.parse()?),
                    };

                    Ok(value)
                };

                match value() {
                    Ok(value) => value,
                    Err(err) => panic!("Failed to parse float value: {:?} from {:?}", err, float),
                }
            }
            unsupported => panic!("Unsupported constant: {:?}", unsupported),
        }
    }
}

impl Add for Literal {
    type Output = Literal;

    fn add(self, rhs: Self) -> Self::Output {
        #[allow(clippy::match_single_binding)]
        match (self, rhs) {
            unsupported => panic!("Unsupported Add: {:?}", unsupported),
        }
    }
}

impl Sub for Literal {
    type Output = Literal;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::UnsignedInt32(lhs), Literal::Int32(rhs)) => {
                Literal::UnsignedInt32(lhs.sub(rhs as u32))
            }
            unsupported => panic!("Unsupported Sub: {:?}", unsupported),
        }
    }
}

impl Mul for Literal {
    type Output = Literal;

    fn mul(self, rhs: Self) -> Self::Output {
        #[allow(clippy::match_single_binding)]
        match (self, rhs) {
            unsupported => panic!("Unsupported Mul: {:?}", unsupported),
        }
    }
}

impl Div for Literal {
    type Output = Literal;

    fn div(self, rhs: Self) -> Self::Output {
        #[allow(clippy::match_single_binding)]
        match (self, rhs) {
            unsupported => panic!("Unsupported Div: {:?}", unsupported),
        }
    }
}

impl Not for Literal {
    type Output = Literal;

    fn not(self) -> Self::Output {
        match self {
            Literal::UnsignedInt32(operand) => Literal::UnsignedInt32(!operand),
            Literal::UnsignedInt64(operand) => Literal::UnsignedInt64(!operand),
            unsupported => panic!("Unsupported Not: {:?}", unsupported),
        }
    }
}

impl Neg for Literal {
    type Output = Literal;

    fn neg(self) -> Self::Output {
        match self {
            Literal::Int32(operand) => Literal::Int32(-operand),
            unsupported => panic!("Unsupported Neg: {:?}", unsupported),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

impl Expression {
    pub fn eval_to_literal(&self) -> Literal {
        match self {
            Expression::Literal(literal) => literal.clone(),
            Expression::Unary { operator, operand } => {
                let operand = operand.eval_to_literal();

                match operator {
                    UnaryOperator::Complement => !operand,
                    UnaryOperator::Minus => -operand,
                    unsupported => panic!("Unsupported unary operator: {:?}", unsupported),
                }
            }
            Expression::Binary { operator, lhs, rhs } => {
                let lhs = lhs.eval_to_literal();
                let rhs = rhs.eval_to_literal();

                match operator {
                    BinaryOperator::Plus => lhs + rhs,
                    BinaryOperator::Minus => lhs - rhs,
                    BinaryOperator::Multiply => lhs * rhs,
                    BinaryOperator::Divide => lhs / rhs,
                    unsupported => panic!("Unsupported binary operator: {:?}", unsupported),
                }
            }
        }
    }
}

impl From<&CExpression> for Expression {
    fn from(expression: &CExpression) -> Self {
        match expression {
            CExpression::Constant(constant) => Expression::Literal(Literal::from(&constant.node)),
            CExpression::UnaryOperator(unary) => Expression::Unary {
                operator: unary.node.operator.node.clone(),
                operand: Box::new(Expression::from(&unary.node.operand.node)),
            },
            CExpression::BinaryOperator(binary) => Expression::Binary {
                operator: binary.node.operator.node.clone(),
                lhs: Box::new(Expression::from(&binary.node.lhs.node)),
                rhs: Box::new(Expression::from(&binary.node.rhs.node)),
            },
            unsupported => panic!("Unsupported expression: {:?}", unsupported),
        }
    }
}
