// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.

use super::*;
use crate::binder::BoundExpr;
use crate::binder::BoundExpr::*;
use crate::parser::BinaryOperator::*;
use crate::parser::UnaryOperator;
use crate::types::DataTypeKind as Ty;
use crate::types::DataValue::*;

/// Arithemtic expression simplification rule prunes the useless constant in the binary expressions.
///
/// For example,
/// `select 1 * a, b / 1, c + 0, d - 0 from t;`
/// The query will be converted to:
/// `select a, b, c, d from t;`
pub struct ArithExprSimplification;

impl Rewriter for ArithExprSimplification {
    fn rewrite_expr(&mut self, expr: &mut BoundExpr) {
        // TODO: support more data types.
        let new = match &expr {
            BinaryOp(op) => match (&op.op, &*op.left_expr, &*op.right_expr) {
                // x + 0, 0 + x
                (Plus, Constant(Int32(0)), other) => other.clone(),
                (Plus, other, Constant(Int32(0))) => other.clone(),
                (Plus, Constant(Float64(f)), other) if *f == 0.0 => other.clone(),
                (Plus, other, Constant(Float64(f))) if *f == 0.0 => other.clone(),
                // x - 0
                (Minus, other, Constant(Int32(0))) => other.clone(),
                (Minus, other, Constant(Float64(f))) if *f == 0.0 => other.clone(),
                // x * 0, 0 * x
                (Multiply, Constant(Int32(0)), _) => Constant(Int32(0)),
                (Multiply, _, Constant(Int32(0))) => Constant(Int32(0)),
                (Multiply, Constant(Float64(f)), _) if *f == 0.0 => Constant(Float64(0.0)),
                (Multiply, _, Constant(Float64(f))) if *f == 0.0 => Constant(Float64(0.0)),
                // x * 1, 1 * x
                (Multiply, Constant(Int32(1)), other) => other.clone(),
                (Multiply, other, Constant(Int32(1))) => other.clone(),
                (Multiply, Constant(Float64(f)), other) if *f == 1.0 => other.clone(),
                (Multiply, other, Constant(Float64(f))) if *f == 1.0 => other.clone(),
                // x / 1
                (Divide, other, Constant(Int32(1))) => other.clone(),
                (Divide, other, Constant(Float64(f))) if *f == 1.0 => other.clone(),

                _ => return,
            },
            UnaryOp(op) => match (&op.op, &*op.expr) {
                (UnaryOperator::Plus, other) => other.clone(),
                _ => return,
            },
            TypeCast(op) => match (&op.ty, &*op.expr) {
                (Ty::Boolean, k @ Constant(Bool(_))) => k.clone(),
                (Ty::Int(_), k @ Constant(Int32(_))) => k.clone(),
                (Ty::BigInt(_), k @ Constant(Int64(_))) => k.clone(),
                (Ty::Double, k @ Constant(Float64(_))) => k.clone(),
                (Ty::String, k @ Constant(String(_))) => k.clone(),
                _ => return,
            },
            _ => return,
        };
        *expr = new;
    }
}
