// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::token::{TokenType};
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Function(Option<Expression>, Box<str>, Option<Vec<Expression>>, Expression, Box<str>),
    If(Expression, Expression, Option<Expression>),
    While(Expression, Expression),
    For(Box<str>, Expression, Expression),
    Loop(Expression),
    Return(Expression),
    Defer(Expression),
    Let(Box<str>, Option<Box<str>>, Expression),
    Const(Box<str>, Option<Box<str>>, Expression),
    Expr(Expression),
    Block(Vec<Statement>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(Box<str>),
    Number(Box<str>),
    Str(Box<str>),
    Bool(bool),
    Paren(Box<Expression>),
    Block(Vec<Statement>),
    Array(Vec<Expression>),
    Tuple(Vec<Expression>),
    Assign(Box<Expression>, Box<Expression>),
    BinOp(Box<Expression>, TokenType, Box<Expression>),
    UnaryOp(TokenType, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Field(Box<Expression>, Box<str>),
    Scope(Box<Expression>, Box<str>),
    Closure(Vec<Box<str>>, Box<Expression>),
}

// ── Entry point ──────────────────────────────────────────────────────────────

pub fn print_ast(stmt: &Statement) {
    let mut out = String::new();
    fmt_statement(&mut out, stmt, 0);
    print!("{out}");
}

// ── Indent helper ─────────────────────────────────────────────────────────────

fn indent(buf: &mut String, depth: usize) {
    for _ in 0..depth {
        buf.push_str("  ");
    }
}

// ── Statements ────────────────────────────────────────────────────────────────

fn fmt_statement(buf: &mut String, stmt: &Statement, depth: usize) {
    match stmt {
        Statement::Block(stmts) => {
            indent(buf, depth); buf.push_str("Block\n");
            for s in stmts { fmt_statement(buf, s, depth + 1); }
        }
        Statement::Function(attach, name, args, body, ret) => {
            indent(buf, depth);
            let _ = writeln!(buf, "Fn '{name}' -> {ret}");
            if let Some(a) = attach {
                indent(buf, depth + 1); buf.push_str("attach:\n");
                fmt_expression(buf, a, depth + 2);
            }
            if let Some(params) = args {
                indent(buf, depth + 1); buf.push_str("params:\n");
                for p in params { fmt_expression(buf, p, depth + 2); }
            }
            indent(buf, depth + 1); buf.push_str("body:\n");
            fmt_expression(buf, body, depth + 2);
        }
        Statement::If(cond, then, else_) => {
            indent(buf, depth); buf.push_str("If\n");
            indent(buf, depth + 1); buf.push_str("cond:\n");
            fmt_expression(buf, cond, depth + 2);
            indent(buf, depth + 1); buf.push_str("then:\n");
            fmt_expression(buf, then, depth + 2);
            if let Some(e) = else_ {
                indent(buf, depth + 1); buf.push_str("else:\n");
                fmt_expression(buf, e, depth + 2);
            }
        }
        Statement::While(cond, body) => {
            indent(buf, depth); buf.push_str("While\n");
            indent(buf, depth + 1); buf.push_str("cond:\n");
            fmt_expression(buf, cond, depth + 2);
            indent(buf, depth + 1); buf.push_str("body:\n");
            fmt_expression(buf, body, depth + 2);
        }
        Statement::For(var, iter, body) => {
            indent(buf, depth);
            let _ = writeln!(buf, "For '{var}'");
            indent(buf, depth + 1); buf.push_str("iter:\n");
            fmt_expression(buf, iter, depth + 2);
            indent(buf, depth + 1); buf.push_str("body:\n");
            fmt_expression(buf, body, depth + 2);
        }
        Statement::Loop(body) => {
            indent(buf, depth); buf.push_str("Loop\n");
            fmt_expression(buf, body, depth + 1);
        }
        Statement::Return(val) => {
            indent(buf, depth); buf.push_str("Return\n");
            fmt_expression(buf, val, depth + 1);
        }
        Statement::Defer(val) => {
            indent(buf, depth); buf.push_str("Defer\n");
            fmt_expression(buf, val, depth + 1);
        }
        Statement::Let(name, ty, val) => {
            indent(buf, depth);
            let _ = writeln!(buf, "Let '{name}'{}", ty.as_deref().map(|t| format!(": {t}")).unwrap_or_default());
            fmt_expression(buf, val, depth + 1);
        }
        Statement::Const(name, ty, val) => {
            indent(buf, depth);
            let _ = writeln!(buf, "Const '{name}'{}", ty.as_deref().map(|t| format!(": {t}")).unwrap_or_default());
            fmt_expression(buf, val, depth + 1);
        }
        Statement::Expr(expr) => {
            fmt_expression(buf, expr, depth);
        }
    }
}

// ── Expressions ───────────────────────────────────────────────────────────────

fn fmt_expression(buf: &mut String, expr: &Expression, depth: usize) {
    match expr {
        Expression::Ident(s) => {
            indent(buf, depth); let _ = writeln!(buf, "Ident({s})");
        }
        Expression::Number(n) => {
            indent(buf, depth); let _ = writeln!(buf, "Number({n})");
        }
        Expression::Str(s) => {
            indent(buf, depth); let _ = writeln!(buf, "Str({s:?})");
        }
        Expression::Bool(b) => {
            indent(buf, depth); let _ = writeln!(buf, "Bool({b})");
        }
        Expression::Paren(inner) => {
            indent(buf, depth); buf.push_str("Paren\n");
            fmt_expression(buf, inner, depth + 1);
        }
        Expression::Block(stmts) => {
            indent(buf, depth); buf.push_str("Block\n");
            for s in stmts { fmt_statement(buf, s, depth + 1); }
        }
        Expression::Array(items) => {
            indent(buf, depth); let _ = writeln!(buf, "Array[{}]", items.len());
            for item in items { fmt_expression(buf, item, depth + 1); }
        }
        Expression::Tuple(items) => {
            indent(buf, depth); let _ = writeln!(buf, "Tuple({})", items.len());
            for item in items { fmt_expression(buf, item, depth + 1); }
        }
        Expression::Assign(lhs, rhs) => {
            indent(buf, depth); buf.push_str("Assign\n");
            fmt_expression(buf, lhs, depth + 1);
            fmt_expression(buf, rhs, depth + 1);
        }
        Expression::BinOp(lhs, op, rhs) => {
            indent(buf, depth); let _ = writeln!(buf, "BinOp({op})");
            fmt_expression(buf, lhs, depth + 1);
            fmt_expression(buf, rhs, depth + 1);
        }
        Expression::UnaryOp(op, operand) => {
            indent(buf, depth); let _ = writeln!(buf, "UnaryOp({op})");
            fmt_expression(buf, operand, depth + 1);
        }
        Expression::Call(callee, args) => {
            indent(buf, depth); let _ = writeln!(buf, "Call[{}]", args.len());
            indent(buf, depth + 1); buf.push_str("callee:\n");
            fmt_expression(buf, callee, depth + 2);
            if !args.is_empty() {
                indent(buf, depth + 1); buf.push_str("args:\n");
                for a in args { fmt_expression(buf, a, depth + 2); }
            }
        }
        Expression::Index(base, idx) => {
            indent(buf, depth); buf.push_str("Index\n");
            fmt_expression(buf, base, depth + 1);
            fmt_expression(buf, idx, depth + 1);
        }
        Expression::Field(base, field) => {
            indent(buf, depth); let _ = writeln!(buf, "Field .{field}");
            fmt_expression(buf, base, depth + 1);
        }
        Expression::Scope(base, seg) => {
            indent(buf, depth); let _ = writeln!(buf, "Scope ::{seg}");
            fmt_expression(buf, base, depth + 1);
        }
        Expression::Closure(params, body) => {
            indent(buf, depth);
            let _ = writeln!(buf, "Closure({})", params.join(", "));
            fmt_expression(buf, body, depth + 1);
        }
    }
}
