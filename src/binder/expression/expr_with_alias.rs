// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.

use super::*;

/// A bound expression with alias
#[derive(PartialEq, Clone)]
pub struct BoundExprWithAlias {
    pub expr: Box<BoundExpr>,
    pub alias: String,
}

impl std::fmt::Debug for BoundExprWithAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (alias to {})", self.expr, self.alias)
    }
}

/// An alias reference to a bound expression
#[derive(PartialEq, Clone)]
pub struct BoundAlias {
    pub alias: String,
}

impl std::fmt::Debug for BoundAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alias)
    }
}

impl Binder {
    /// Bind an alias to a bound expression
    pub fn bind_alias(&mut self, expr: BoundExpr, ident: Ident) -> BoundExpr {
        let alias = ident.value;
        self.context.aliases.push(alias.clone());
        BoundExpr::ExprWithAlias(BoundExprWithAlias {
            expr: Box::new(expr),
            alias,
        })
    }
}
