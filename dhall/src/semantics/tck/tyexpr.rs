use crate::error::{TypeError, TypeMessage};
use crate::semantics::{rc, NameEnv, NzEnv, TyEnv, Value};
use crate::syntax::{ExprKind, Span, V};
use crate::Normalized;
use crate::{NormalizedExpr, ToExprOptions};

pub(crate) type Type = Value;

/// Stores an alpha-normalized variable.
#[derive(Debug, Clone, Copy)]
pub struct AlphaVar {
    idx: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum TyExprKind {
    Var(AlphaVar),
    // Forbidden ExprKind variants: Var, Import, Embed
    Expr(ExprKind<TyExpr, Normalized>),
}

// An expression with inferred types at every node and resolved variables.
#[derive(Clone)]
pub(crate) struct TyExpr {
    kind: Box<TyExprKind>,
    ty: Option<Type>,
    span: Span,
}

impl AlphaVar {
    pub(crate) fn new(idx: usize) -> Self {
        AlphaVar { idx }
    }
    pub(crate) fn idx(&self) -> usize {
        self.idx
    }
}

impl TyExpr {
    pub fn new(kind: TyExprKind, ty: Option<Type>, span: Span) -> Self {
        TyExpr {
            kind: Box::new(kind),
            ty,
            span,
        }
    }

    pub fn kind(&self) -> &TyExprKind {
        &*self.kind
    }
    pub fn span(&self) -> Span {
        self.span.clone()
    }
    pub fn get_type(&self) -> Result<Type, TypeError> {
        match &self.ty {
            Some(t) => Ok(t.clone()),
            None => Err(TypeError::new(TypeMessage::Sort)),
        }
    }

    /// Converts a value back to the corresponding AST expression.
    pub fn to_expr(&self, opts: ToExprOptions) -> NormalizedExpr {
        tyexpr_to_expr(self, opts, &mut NameEnv::new())
    }
    pub fn to_expr_tyenv(&self, env: &TyEnv) -> NormalizedExpr {
        let opts = ToExprOptions {
            normalize: true,
            alpha: false,
        };
        let mut env = env.as_nameenv().clone();
        tyexpr_to_expr(self, opts, &mut env)
    }

    /// Eval the TyExpr. It will actually get evaluated only as needed on demand.
    pub fn eval(&self, env: &NzEnv) -> Value {
        Value::new_thunk(env, self.clone())
    }
    /// Eval a closed TyExpr (i.e. without free variables). It will actually get evaluated only as
    /// needed on demand.
    pub fn eval_closed_expr(&self) -> Value {
        self.eval(&NzEnv::new())
    }
    /// Eval a closed TyExpr fully and recursively;
    pub fn rec_eval_closed_expr(&self) -> Value {
        let mut val = self.eval_closed_expr();
        val.normalize_mut();
        val
    }
}

fn tyexpr_to_expr<'a>(
    tyexpr: &'a TyExpr,
    opts: ToExprOptions,
    env: &mut NameEnv,
) -> NormalizedExpr {
    rc(match tyexpr.kind() {
        TyExprKind::Var(v) if opts.alpha => {
            ExprKind::Var(V("_".into(), v.idx()))
        }
        TyExprKind::Var(v) => ExprKind::Var(env.label_var(v)),
        TyExprKind::Expr(e) => {
            let e = e.map_ref_maybe_binder(|l, tye| {
                if let Some(l) = l {
                    env.insert_mut(l);
                }
                let e = tyexpr_to_expr(tye, opts, env);
                if let Some(_) = l {
                    env.remove_mut();
                }
                e
            });

            match e {
                ExprKind::Lam(_, t, e) if opts.alpha => {
                    ExprKind::Lam("_".into(), t, e)
                }
                ExprKind::Pi(_, t, e) if opts.alpha => {
                    ExprKind::Pi("_".into(), t, e)
                }
                e => e,
            }
        }
    })
}

impl std::fmt::Debug for TyExpr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = fmt.debug_struct("TyExpr");
        x.field("kind", self.kind());
        if let Some(ty) = self.ty.as_ref() {
            x.field("type", &ty);
        } else {
            x.field("type", &None::<()>);
        }
        x.finish()
    }
}
