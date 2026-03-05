//! Source-to-bytecode compilation for TradeLang programs.
//!
//! This module drives lexing and parsing, performs semantic analysis and type
//! inference, resolves locals and builtins, and emits deterministic bytecode.

use std::collections::{HashMap, HashSet};

use crate::ast::{
    Ast, BinaryOp, Block, Expr, ExprKind, FunctionDecl, NodeId, Stmt, StmtKind, UnaryOp,
};
use crate::builtins::BuiltinId;
use crate::bytecode::{Constant, Instruction, LocalInfo, OpCode, Program};
use crate::diagnostic::{CompileError, Diagnostic, DiagnosticKind};
use crate::lexer;
use crate::parser;
use crate::span::Span;
use crate::types::{SlotKind, Type, Value};

const PREDEFINED_SERIES: [(&str, Type); 6] = [
    ("open", Type::SeriesF64),
    ("high", Type::SeriesF64),
    ("low", Type::SeriesF64),
    ("close", Type::SeriesF64),
    ("volume", Type::SeriesF64),
    ("time", Type::SeriesF64),
];

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CompiledProgram {
    pub program: Program,
    pub source: String,
}

pub fn compile(source: &str) -> Result<CompiledProgram, CompileError> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    Compiler::new(source, &ast).compile()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum InferredType {
    Concrete(Type),
    Na,
}

impl InferredType {
    fn concrete(self) -> Option<Type> {
        match self {
            Self::Concrete(ty) => Some(ty),
            Self::Na => None,
        }
    }

    fn allow_bool(self) -> bool {
        matches!(
            self,
            Self::Concrete(Type::Bool | Type::SeriesBool) | Self::Na
        )
    }

    fn is_numeric_like(self) -> bool {
        matches!(self, Self::Concrete(Type::F64 | Type::SeriesF64) | Self::Na)
    }
}

#[derive(Clone, Copy, Debug)]
struct AnalyzerSymbol {
    ty: InferredType,
}

#[derive(Clone, Copy, Debug)]
struct CompilerSymbol {
    slot: u16,
    ty: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FunctionSpecializationKey {
    function_id: NodeId,
    arg_types: Vec<InferredType>,
}

#[derive(Clone, Copy, Debug)]
struct FunctionParamBinding {
    ty: Type,
    kind: SlotKind,
}

#[derive(Clone, Debug)]
struct FunctionSpecialization {
    expr_types: HashMap<NodeId, InferredType>,
    user_function_calls: HashMap<NodeId, FunctionSpecializationKey>,
    return_type: InferredType,
    history_capacity: usize,
    param_bindings: Vec<FunctionParamBinding>,
}

#[derive(Default)]
struct Analysis {
    expr_types: HashMap<NodeId, InferredType>,
    user_function_calls: HashMap<NodeId, FunctionSpecializationKey>,
    resolved_let_slots: HashMap<NodeId, u16>,
    locals: Vec<LocalInfo>,
    history_capacity: usize,
    function_specializations: HashMap<FunctionSpecializationKey, FunctionSpecialization>,
}

struct Analyzer<'a> {
    diagnostics: Vec<Diagnostic>,
    scopes: Vec<HashMap<String, AnalyzerSymbol>>,
    analysis: Analysis,
    functions_by_name: HashMap<String, &'a FunctionDecl>,
    functions_by_id: HashMap<NodeId, &'a FunctionDecl>,
    active_specializations: HashSet<FunctionSpecializationKey>,
}

impl<'a> Analyzer<'a> {
    fn new(ast: &'a Ast) -> Self {
        let mut analyzer = Self {
            diagnostics: Vec::new(),
            scopes: vec![HashMap::new()],
            analysis: Analysis {
                history_capacity: 2,
                ..Analysis::default()
            },
            functions_by_name: HashMap::new(),
            functions_by_id: HashMap::new(),
            active_specializations: HashSet::new(),
        };

        for (name, ty) in PREDEFINED_SERIES {
            analyzer.define_symbol(name.to_string(), InferredType::Concrete(ty), true);
        }
        analyzer.collect_functions(ast);
        analyzer.validate_function_bodies();
        analyzer.validate_function_cycles();
        analyzer
    }

    fn analyze(mut self, ast: &Ast) -> Result<Analysis, CompileError> {
        for stmt in &ast.statements {
            self.analyze_stmt(stmt);
        }
        if self.diagnostics.is_empty() {
            Ok(self.analysis)
        } else {
            Err(CompileError::new(self.diagnostics))
        }
    }

    fn collect_functions(&mut self, ast: &'a Ast) {
        for function in &ast.functions {
            if BuiltinId::from_name(&function.name).is_some() {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    format!("function name `{}` collides with a builtin", function.name),
                    function.span,
                ));
                continue;
            }
            if self.functions_by_name.contains_key(&function.name) {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    format!("duplicate function `{}`", function.name),
                    function.span,
                ));
                continue;
            }

            let mut seen = HashSet::new();
            for param in &function.params {
                if !seen.insert(param.name.as_str()) {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!(
                            "duplicate parameter `{}` in function `{}`",
                            param.name, function.name
                        ),
                        param.span,
                    ));
                }
            }

            self.functions_by_name
                .insert(function.name.clone(), function);
            self.functions_by_id.insert(function.id, function);
        }
    }

    fn validate_function_cycles(&mut self) {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        let mut reported = HashSet::new();
        let functions: Vec<&FunctionDecl> = self.functions_by_id.values().copied().collect();
        for function in functions {
            self.visit_function_cycle(function, &mut visiting, &mut visited, &mut reported);
        }
    }

    fn validate_function_bodies(&mut self) {
        let functions: Vec<&FunctionDecl> = self.functions_by_id.values().copied().collect();
        for function in functions {
            let params: HashSet<&str> = function
                .params
                .iter()
                .map(|param| param.name.as_str())
                .collect();
            self.validate_function_expr(&function.body, &params);
        }
    }

    fn validate_function_expr(&mut self, expr: &Expr, params: &HashSet<&str>) {
        match &expr.kind {
            ExprKind::Ident(name) => {
                if !params.contains(name.as_str()) && !is_predefined_series_name(name) {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!(
                            "function bodies may only reference parameters or predefined series; found `{name}`"
                        ),
                        expr.span,
                    ));
                }
            }
            ExprKind::Unary { expr, .. } => self.validate_function_expr(expr, params),
            ExprKind::Binary { left, right, .. } => {
                self.validate_function_expr(left, params);
                self.validate_function_expr(right, params);
            }
            ExprKind::Call { callee, args } => {
                match BuiltinId::from_name(callee) {
                    Some(BuiltinId::Plot) => {
                        self.diagnostics.push(Diagnostic::new(
                            DiagnosticKind::Type,
                            "function bodies may not call `plot`",
                            expr.span,
                        ));
                    }
                    Some(BuiltinId::Sma | BuiltinId::Ema | BuiltinId::Rsi) => {
                        if args.len() != 2 {
                            self.diagnostics.push(Diagnostic::new(
                                DiagnosticKind::Type,
                                format!("{callee} expects exactly two arguments"),
                                expr.span,
                            ));
                        }
                    }
                    Some(
                        BuiltinId::Open
                        | BuiltinId::High
                        | BuiltinId::Low
                        | BuiltinId::Close
                        | BuiltinId::Volume
                        | BuiltinId::Time,
                    ) => {
                        self.diagnostics.push(Diagnostic::new(
                            DiagnosticKind::Type,
                            "market data builtins are identifiers, not callable functions",
                            expr.span,
                        ));
                    }
                    None => match self.functions_by_name.get(callee).copied() {
                        Some(target) if target.params.len() != args.len() => {
                            self.diagnostics.push(Diagnostic::new(
                                DiagnosticKind::Type,
                                format!(
                                    "function `{callee}` expects {} argument(s), found {}",
                                    target.params.len(),
                                    args.len()
                                ),
                                expr.span,
                            ));
                        }
                        Some(_) => {}
                        None => {
                            self.diagnostics.push(Diagnostic::new(
                                DiagnosticKind::Type,
                                format!("unknown function `{callee}`"),
                                expr.span,
                            ));
                        }
                    },
                }
                for arg in args {
                    self.validate_function_expr(arg, params);
                }
            }
            ExprKind::Index { target, index } => {
                self.validate_function_expr(target, params);
                self.validate_function_expr(index, params);
            }
            ExprKind::Number(_) | ExprKind::Bool(_) | ExprKind::Na => {}
        }
    }

    fn visit_function_cycle(
        &mut self,
        function: &'a FunctionDecl,
        visiting: &mut HashSet<NodeId>,
        visited: &mut HashSet<NodeId>,
        reported: &mut HashSet<NodeId>,
    ) {
        if visited.contains(&function.id) {
            return;
        }
        visiting.insert(function.id);
        let callees: Vec<NodeId> = called_user_functions(&function.body, &self.functions_by_name)
            .into_iter()
            .filter_map(|callee| {
                self.functions_by_name
                    .get(callee)
                    .map(|function| function.id)
            })
            .collect();
        for callee_id in callees {
            let Some(target) = self.functions_by_id.get(&callee_id).copied() else {
                continue;
            };
            if visiting.contains(&target.id) {
                if reported.insert(function.id) {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        "recursive and cyclic function definitions are not allowed",
                        function.span,
                    ));
                }
                continue;
            }
            self.visit_function_cycle(target, visiting, visited, reported);
        }
        visiting.remove(&function.id);
        visited.insert(function.id);
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let { name, expr } => {
                let expr_ty = self.analyze_expr(expr);
                let concrete = match expr_ty {
                    InferredType::Concrete(ty) => ty,
                    InferredType::Na => Type::F64,
                };
                if self.scopes.last().unwrap().contains_key(name) {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("duplicate binding `{name}` in the same scope"),
                        stmt.span,
                    ));
                    return;
                }
                let slot =
                    self.define_symbol(name.clone(), InferredType::Concrete(concrete), false);
                self.analysis.resolved_let_slots.insert(stmt.id, slot);
            }
            StmtKind::If {
                condition,
                then_block,
                else_block,
            } => {
                let ty = self.analyze_expr(condition);
                if !ty.allow_bool() {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        "if condition must be bool, series<bool>, or na",
                        condition.span,
                    ));
                }
                self.push_scope();
                self.analyze_block(then_block);
                self.pop_scope();
                self.push_scope();
                self.analyze_block(else_block);
                self.pop_scope();
            }
            StmtKind::Expr(expr) => {
                self.analyze_expr(expr);
            }
        }
    }

    fn analyze_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> InferredType {
        let inferred = match &expr.kind {
            ExprKind::Number(_) => InferredType::Concrete(Type::F64),
            ExprKind::Bool(_) => InferredType::Concrete(Type::Bool),
            ExprKind::Na => InferredType::Na,
            ExprKind::Ident(name) => {
                let Some(symbol) = self.lookup_symbol(name) else {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("unknown identifier `{name}`"),
                        expr.span,
                    ));
                    return InferredType::Concrete(Type::F64);
                };
                symbol.ty
            }
            ExprKind::Unary { op, expr: inner } => self.analyze_unary(*op, inner),
            ExprKind::Binary { op, left, right } => self.analyze_binary(*op, left, right),
            ExprKind::Call { callee, args } => self.analyze_call(expr, callee, args),
            ExprKind::Index { target, index } => self.analyze_index(target, index, expr.span),
        };
        self.analysis.expr_types.insert(expr.id, inferred);
        inferred
    }

    fn analyze_unary(&mut self, op: UnaryOp, inner: &Expr) -> InferredType {
        let inner_ty = self.analyze_expr(inner);
        infer_unary(op, inner_ty, inner.span, &mut self.diagnostics)
    }

    fn analyze_binary(&mut self, op: BinaryOp, left: &Expr, right: &Expr) -> InferredType {
        let left_ty = self.analyze_expr(left);
        let right_ty = self.analyze_expr(right);
        infer_binary(
            op,
            left_ty,
            right_ty,
            left.span.merge(right.span),
            &mut self.diagnostics,
        )
    }

    fn analyze_call(&mut self, expr: &Expr, callee: &str, args: &[Expr]) -> InferredType {
        if let Some(builtin) = BuiltinId::from_name(callee) {
            return self.analyze_builtin_call(builtin, callee, args, expr.span, false);
        }

        let arg_types: Vec<InferredType> = args.iter().map(|arg| self.analyze_expr(arg)).collect();
        let Some(function) = self.functions_by_name.get(callee).copied() else {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                format!("unknown function `{callee}`"),
                expr.span,
            ));
            return InferredType::Concrete(Type::F64);
        };

        if args.len() != function.params.len() {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                format!(
                    "function `{callee}` expects {} argument(s), found {}",
                    function.params.len(),
                    args.len()
                ),
                expr.span,
            ));
            return InferredType::Concrete(Type::F64);
        }

        let key = FunctionSpecializationKey {
            function_id: function.id,
            arg_types,
        };
        self.analysis
            .user_function_calls
            .insert(expr.id, key.clone());
        let return_type = self.ensure_function_specialization(&key, expr.span);
        if let Some(spec) = self.analysis.function_specializations.get(&key) {
            self.analysis.history_capacity =
                self.analysis.history_capacity.max(spec.history_capacity);
        }
        return_type
    }

    fn analyze_builtin_call(
        &mut self,
        builtin: BuiltinId,
        callee: &str,
        args: &[Expr],
        span: Span,
        in_function_body: bool,
    ) -> InferredType {
        match builtin {
            BuiltinId::Plot => {
                if in_function_body {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        "function bodies may not call `plot`",
                        span,
                    ));
                }
                if args.len() != 1 {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        "plot expects exactly one argument",
                        span,
                    ));
                    return InferredType::Concrete(Type::Void);
                }
                let arg_ty = self.analyze_expr(&args[0]);
                if !matches!(
                    arg_ty,
                    InferredType::Concrete(Type::F64 | Type::SeriesF64) | InferredType::Na
                ) {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        "plot expects a numeric or series numeric value",
                        args[0].span,
                    ));
                }
                InferredType::Concrete(Type::Void)
            }
            BuiltinId::Sma | BuiltinId::Ema | BuiltinId::Rsi => {
                if args.len() != 2 {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} expects exactly two arguments"),
                        span,
                    ));
                    return InferredType::Concrete(Type::SeriesF64);
                }
                let series_ty = self.analyze_expr(&args[0]);
                if !matches!(series_ty, InferredType::Concrete(Type::SeriesF64)) {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} requires series<float> as the first argument"),
                        args[0].span,
                    ));
                }
                match literal_window(&args[1]) {
                    Some(window) if window > 0 => {
                        self.analysis.history_capacity =
                            self.analysis.history_capacity.max(window + 1);
                    }
                    Some(_) => self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} length must be greater than zero"),
                        args[1].span,
                    )),
                    None => self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} length must be a non-negative integer literal"),
                        args[1].span,
                    )),
                }
                InferredType::Concrete(Type::SeriesF64)
            }
            BuiltinId::Open
            | BuiltinId::High
            | BuiltinId::Low
            | BuiltinId::Close
            | BuiltinId::Volume
            | BuiltinId::Time => {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "market data builtins are identifiers, not callable functions",
                    span,
                ));
                for arg in args {
                    self.analyze_expr(arg);
                }
                InferredType::Concrete(Type::SeriesF64)
            }
        }
    }

    fn analyze_index(&mut self, target: &Expr, index: &Expr, span: Span) -> InferredType {
        let target_ty = self.analyze_expr(target);
        let Some(offset) = literal_window(index) else {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                "series indexing requires a non-negative integer literal",
                index.span,
            ));
            return InferredType::Concrete(Type::F64);
        };
        self.analysis.history_capacity = self.analysis.history_capacity.max(offset + 1);
        match target_ty {
            InferredType::Concrete(Type::SeriesF64) => InferredType::Concrete(Type::F64),
            InferredType::Concrete(Type::SeriesBool) => InferredType::Concrete(Type::Bool),
            InferredType::Na => InferredType::Na,
            _ => {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "only series values can be indexed",
                    span,
                ));
                InferredType::Concrete(Type::F64)
            }
        }
    }

    fn ensure_function_specialization(
        &mut self,
        key: &FunctionSpecializationKey,
        span: Span,
    ) -> InferredType {
        if let Some(spec) = self.analysis.function_specializations.get(key) {
            return spec.return_type;
        }
        if !self.active_specializations.insert(key.clone()) {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                "recursive and cyclic function definitions are not allowed",
                span,
            ));
            return InferredType::Concrete(Type::F64);
        }

        let return_type = match self.functions_by_id.get(&key.function_id).copied() {
            Some(function) => {
                let spec = FunctionAnalyzer::new(self, function, key.arg_types.clone()).analyze();
                let return_type = spec.return_type;
                self.analysis
                    .function_specializations
                    .insert(key.clone(), spec);
                return_type
            }
            None => {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "unknown function specialization target",
                    span,
                ));
                InferredType::Concrete(Type::F64)
            }
        };

        self.active_specializations.remove(key);
        return_type
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_symbol(&mut self, name: String, ty: InferredType, hidden: bool) -> u16 {
        let slot = self.analysis.locals.len() as u16;
        let concrete = match ty {
            InferredType::Concrete(ty) => ty,
            InferredType::Na => Type::SeriesF64,
        };
        let kind = if concrete.is_series() {
            SlotKind::Series
        } else {
            SlotKind::Scalar
        };
        self.analysis.locals.push(LocalInfo {
            name: if hidden { None } else { Some(name.clone()) },
            ty: concrete,
            kind,
            hidden,
        });
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name, AnalyzerSymbol { ty });
        slot
    }

    fn lookup_symbol(&self, name: &str) -> Option<AnalyzerSymbol> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }
}

struct FunctionAnalyzer<'a, 'b> {
    parent: &'b mut Analyzer<'a>,
    function: &'a FunctionDecl,
    scopes: Vec<HashMap<String, AnalyzerSymbol>>,
    expr_types: HashMap<NodeId, InferredType>,
    user_function_calls: HashMap<NodeId, FunctionSpecializationKey>,
    history_capacity: usize,
    param_bindings: Vec<FunctionParamBinding>,
}

impl<'a, 'b> FunctionAnalyzer<'a, 'b> {
    fn new(
        parent: &'b mut Analyzer<'a>,
        function: &'a FunctionDecl,
        arg_types: Vec<InferredType>,
    ) -> Self {
        let mut root = HashMap::new();
        for (name, ty) in PREDEFINED_SERIES {
            root.insert(
                name.to_string(),
                AnalyzerSymbol {
                    ty: InferredType::Concrete(ty),
                },
            );
        }

        let mut param_bindings = Vec::with_capacity(function.params.len());
        for (param, arg_ty) in function.params.iter().zip(arg_types) {
            root.insert(param.name.clone(), AnalyzerSymbol { ty: arg_ty });
            param_bindings.push(param_binding(arg_ty));
        }

        Self {
            parent,
            function,
            scopes: vec![root],
            expr_types: HashMap::new(),
            user_function_calls: HashMap::new(),
            history_capacity: 2,
            param_bindings,
        }
    }

    fn analyze(mut self) -> FunctionSpecialization {
        let return_type = self.analyze_expr(&self.function.body);
        if matches!(return_type, InferredType::Concrete(Type::Void)) {
            self.parent.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                format!("function `{}` must not return void", self.function.name),
                self.function.body.span,
            ));
        }
        FunctionSpecialization {
            expr_types: self.expr_types,
            user_function_calls: self.user_function_calls,
            return_type,
            history_capacity: self.history_capacity,
            param_bindings: self.param_bindings,
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> InferredType {
        let inferred = match &expr.kind {
            ExprKind::Number(_) => InferredType::Concrete(Type::F64),
            ExprKind::Bool(_) => InferredType::Concrete(Type::Bool),
            ExprKind::Na => InferredType::Na,
            ExprKind::Ident(name) => match self.lookup_symbol(name) {
                Some(symbol) => symbol.ty,
                None => {
                    self.parent.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!(
                            "function bodies may only reference parameters or predefined series; found `{name}`"
                        ),
                        expr.span,
                    ));
                    InferredType::Concrete(Type::F64)
                }
            },
            ExprKind::Unary { op, expr: inner } => {
                let inner_ty = self.analyze_expr(inner);
                infer_unary(*op, inner_ty, inner.span, &mut self.parent.diagnostics)
            }
            ExprKind::Binary { op, left, right } => {
                let left_ty = self.analyze_expr(left);
                let right_ty = self.analyze_expr(right);
                infer_binary(
                    *op,
                    left_ty,
                    right_ty,
                    left.span.merge(right.span),
                    &mut self.parent.diagnostics,
                )
            }
            ExprKind::Call { callee, args } => self.analyze_call(expr, callee, args),
            ExprKind::Index { target, index } => self.analyze_index(target, index, expr.span),
        };
        self.expr_types.insert(expr.id, inferred);
        inferred
    }

    fn analyze_call(&mut self, expr: &Expr, callee: &str, args: &[Expr]) -> InferredType {
        if let Some(builtin) = BuiltinId::from_name(callee) {
            return self.analyze_builtin_call(builtin, callee, args, expr.span);
        }

        let arg_types: Vec<InferredType> = args.iter().map(|arg| self.analyze_expr(arg)).collect();
        let Some(function) = self.parent.functions_by_name.get(callee).copied() else {
            self.parent.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                format!("unknown function `{callee}`"),
                expr.span,
            ));
            return InferredType::Concrete(Type::F64);
        };

        if args.len() != function.params.len() {
            self.parent.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                format!(
                    "function `{callee}` expects {} argument(s), found {}",
                    function.params.len(),
                    args.len()
                ),
                expr.span,
            ));
            return InferredType::Concrete(Type::F64);
        }

        let key = FunctionSpecializationKey {
            function_id: function.id,
            arg_types,
        };
        self.user_function_calls.insert(expr.id, key.clone());
        let return_type = self.parent.ensure_function_specialization(&key, expr.span);
        if let Some(spec) = self.parent.analysis.function_specializations.get(&key) {
            self.history_capacity = self.history_capacity.max(spec.history_capacity);
        }
        return_type
    }

    fn analyze_builtin_call(
        &mut self,
        builtin: BuiltinId,
        callee: &str,
        args: &[Expr],
        span: Span,
    ) -> InferredType {
        match builtin {
            BuiltinId::Plot => {
                self.parent.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "function bodies may not call `plot`",
                    span,
                ));
                if args.len() != 1 {
                    self.parent.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        "plot expects exactly one argument",
                        span,
                    ));
                    return InferredType::Concrete(Type::Void);
                }
                let arg_ty = self.analyze_expr(&args[0]);
                if !matches!(
                    arg_ty,
                    InferredType::Concrete(Type::F64 | Type::SeriesF64) | InferredType::Na
                ) {
                    self.parent.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        "plot expects a numeric or series numeric value",
                        args[0].span,
                    ));
                }
                InferredType::Concrete(Type::Void)
            }
            BuiltinId::Sma | BuiltinId::Ema | BuiltinId::Rsi => {
                if args.len() != 2 {
                    self.parent.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} expects exactly two arguments"),
                        span,
                    ));
                    return InferredType::Concrete(Type::SeriesF64);
                }
                let series_ty = self.analyze_expr(&args[0]);
                if !matches!(series_ty, InferredType::Concrete(Type::SeriesF64)) {
                    self.parent.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} requires series<float> as the first argument"),
                        args[0].span,
                    ));
                }
                match literal_window(&args[1]) {
                    Some(window) if window > 0 => {
                        self.history_capacity = self.history_capacity.max(window + 1);
                    }
                    Some(_) => self.parent.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} length must be greater than zero"),
                        args[1].span,
                    )),
                    None => self.parent.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Type,
                        format!("{callee} length must be a non-negative integer literal"),
                        args[1].span,
                    )),
                }
                InferredType::Concrete(Type::SeriesF64)
            }
            BuiltinId::Open
            | BuiltinId::High
            | BuiltinId::Low
            | BuiltinId::Close
            | BuiltinId::Volume
            | BuiltinId::Time => {
                self.parent.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "market data builtins are identifiers, not callable functions",
                    span,
                ));
                for arg in args {
                    self.analyze_expr(arg);
                }
                InferredType::Concrete(Type::SeriesF64)
            }
        }
    }

    fn analyze_index(&mut self, target: &Expr, index: &Expr, span: Span) -> InferredType {
        let target_ty = self.analyze_expr(target);
        let Some(offset) = literal_window(index) else {
            self.parent.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Type,
                "series indexing requires a non-negative integer literal",
                index.span,
            ));
            return InferredType::Concrete(Type::F64);
        };
        self.history_capacity = self.history_capacity.max(offset + 1);
        match target_ty {
            InferredType::Concrete(Type::SeriesF64) => InferredType::Concrete(Type::F64),
            InferredType::Concrete(Type::SeriesBool) => InferredType::Concrete(Type::Bool),
            InferredType::Na => InferredType::Na,
            _ => {
                self.parent.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "only series values can be indexed",
                    span,
                ));
                InferredType::Concrete(Type::F64)
            }
        }
    }

    fn lookup_symbol(&self, name: &str) -> Option<AnalyzerSymbol> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }
}

fn called_user_functions<'a>(
    expr: &'a Expr,
    functions_by_name: &'a HashMap<String, &'a FunctionDecl>,
) -> Vec<&'a str> {
    let mut calls = Vec::new();
    collect_called_user_functions(expr, functions_by_name, &mut calls);
    calls
}

fn collect_called_user_functions<'a>(
    expr: &'a Expr,
    functions_by_name: &'a HashMap<String, &'a FunctionDecl>,
    calls: &mut Vec<&'a str>,
) {
    match &expr.kind {
        ExprKind::Unary { expr, .. } => {
            collect_called_user_functions(expr, functions_by_name, calls)
        }
        ExprKind::Binary { left, right, .. } => {
            collect_called_user_functions(left, functions_by_name, calls);
            collect_called_user_functions(right, functions_by_name, calls);
        }
        ExprKind::Call { callee, args } => {
            if functions_by_name.contains_key(callee) {
                calls.push(callee.as_str());
            }
            for arg in args {
                collect_called_user_functions(arg, functions_by_name, calls);
            }
        }
        ExprKind::Index { target, index } => {
            collect_called_user_functions(target, functions_by_name, calls);
            collect_called_user_functions(index, functions_by_name, calls);
        }
        ExprKind::Number(_) | ExprKind::Bool(_) | ExprKind::Na | ExprKind::Ident(_) => {}
    }
}

fn infer_unary(
    op: UnaryOp,
    inner_ty: InferredType,
    span: Span,
    diagnostics: &mut Vec<Diagnostic>,
) -> InferredType {
    match op {
        UnaryOp::Neg => {
            if inner_ty.is_numeric_like() {
                match inner_ty {
                    InferredType::Concrete(Type::SeriesF64) => {
                        InferredType::Concrete(Type::SeriesF64)
                    }
                    InferredType::Na => InferredType::Na,
                    _ => InferredType::Concrete(Type::F64),
                }
            } else {
                diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "unary `-` requires numeric input",
                    span,
                ));
                InferredType::Concrete(Type::F64)
            }
        }
        UnaryOp::Not => {
            if inner_ty.allow_bool() {
                match inner_ty {
                    InferredType::Concrete(Type::SeriesBool) => {
                        InferredType::Concrete(Type::SeriesBool)
                    }
                    InferredType::Na => InferredType::Na,
                    _ => InferredType::Concrete(Type::Bool),
                }
            } else {
                diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "unary `!` requires bool input",
                    span,
                ));
                InferredType::Concrete(Type::Bool)
            }
        }
    }
}

fn infer_binary(
    op: BinaryOp,
    left_ty: InferredType,
    right_ty: InferredType,
    span: Span,
    diagnostics: &mut Vec<Diagnostic>,
) -> InferredType {
    match op {
        BinaryOp::And | BinaryOp::Or => {
            if !(left_ty.allow_bool() && right_ty.allow_bool()) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "logical operators require bool, series<bool>, or na operands",
                    span,
                ));
            }
            if matches!(
                (left_ty, right_ty),
                (InferredType::Concrete(Type::SeriesBool), _)
                    | (_, InferredType::Concrete(Type::SeriesBool))
            ) {
                InferredType::Concrete(Type::SeriesBool)
            } else if matches!((left_ty, right_ty), (InferredType::Na, InferredType::Na)) {
                InferredType::Na
            } else {
                InferredType::Concrete(Type::Bool)
            }
        }
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
            if !(left_ty.is_numeric_like() && right_ty.is_numeric_like()) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "arithmetic operators require numeric operands",
                    span,
                ));
            }
            if matches!(
                (left_ty, right_ty),
                (InferredType::Concrete(Type::SeriesF64), _)
                    | (_, InferredType::Concrete(Type::SeriesF64))
            ) {
                InferredType::Concrete(Type::SeriesF64)
            } else if matches!((left_ty, right_ty), (InferredType::Na, InferredType::Na)) {
                InferredType::Na
            } else {
                InferredType::Concrete(Type::F64)
            }
        }
        BinaryOp::Eq | BinaryOp::Ne => match (left_ty, right_ty) {
            (InferredType::Concrete(Type::SeriesBool), _)
            | (_, InferredType::Concrete(Type::SeriesBool))
            | (InferredType::Concrete(Type::SeriesF64), _)
            | (_, InferredType::Concrete(Type::SeriesF64)) => {
                InferredType::Concrete(Type::SeriesBool)
            }
            (InferredType::Na, InferredType::Na) => InferredType::Na,
            _ => InferredType::Concrete(Type::Bool),
        },
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            if !(left_ty.is_numeric_like() && right_ty.is_numeric_like()) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Type,
                    "comparison operators require numeric operands",
                    span,
                ));
            }
            if matches!(
                (left_ty, right_ty),
                (InferredType::Concrete(Type::SeriesF64), _)
                    | (_, InferredType::Concrete(Type::SeriesF64))
            ) {
                InferredType::Concrete(Type::SeriesBool)
            } else if matches!((left_ty, right_ty), (InferredType::Na, InferredType::Na)) {
                InferredType::Na
            } else {
                InferredType::Concrete(Type::Bool)
            }
        }
    }
}

fn literal_window(expr: &Expr) -> Option<usize> {
    match expr.kind {
        ExprKind::Number(value) if value >= 0.0 && value.fract() == 0.0 => Some(value as usize),
        _ => None,
    }
}

fn param_binding(arg_ty: InferredType) -> FunctionParamBinding {
    match arg_ty {
        InferredType::Concrete(ty) => FunctionParamBinding {
            ty,
            kind: if ty.is_series() {
                SlotKind::Series
            } else {
                SlotKind::Scalar
            },
        },
        InferredType::Na => FunctionParamBinding {
            ty: Type::SeriesF64,
            kind: SlotKind::Series,
        },
    }
}

fn is_predefined_series_name(name: &str) -> bool {
    PREDEFINED_SERIES
        .iter()
        .any(|(predefined, _)| *predefined == name)
}

struct Compiler<'a> {
    source: &'a str,
    ast: &'a Ast,
    analysis: Analysis,
    program: Program,
    diagnostics: Vec<Diagnostic>,
    builtin_call_count: u16,
    scopes: Vec<HashMap<String, CompilerSymbol>>,
    functions_by_id: HashMap<NodeId, &'a FunctionDecl>,
}

impl<'a> Compiler<'a> {
    fn new(source: &'a str, ast: &'a Ast) -> Self {
        let functions_by_id = ast
            .functions
            .iter()
            .map(|function| (function.id, function))
            .collect();
        Self {
            source,
            ast,
            analysis: Analysis::default(),
            program: Program::default(),
            diagnostics: Vec::new(),
            builtin_call_count: 0,
            scopes: Vec::new(),
            functions_by_id,
        }
    }

    fn compile(mut self) -> Result<CompiledProgram, CompileError> {
        self.analysis = Analyzer::new(self.ast).analyze(self.ast)?;
        self.program.locals = self.analysis.locals.clone();
        self.program.history_capacity = self.analysis.history_capacity.max(2);
        self.rebuild_scopes();
        let expr_types = self.analysis.expr_types.clone();
        let user_calls = self.analysis.user_function_calls.clone();
        for stmt in &self.ast.statements {
            self.emit_stmt(stmt, &expr_types, &user_calls);
        }
        self.program
            .instructions
            .push(Instruction::new(OpCode::Return));
        if self.diagnostics.is_empty() {
            Ok(CompiledProgram {
                program: self.program,
                source: self.source.to_string(),
            })
        } else {
            Err(CompileError::new(self.diagnostics))
        }
    }

    fn emit_stmt(
        &mut self,
        stmt: &Stmt,
        expr_types: &HashMap<NodeId, InferredType>,
        user_calls: &HashMap<NodeId, FunctionSpecializationKey>,
    ) {
        match &stmt.kind {
            StmtKind::Let { name, expr } => {
                self.emit_expr(expr, expr_types, user_calls);
                let slot = self.analysis.resolved_let_slots[&stmt.id];
                self.emit(
                    Instruction::new(OpCode::StoreLocal)
                        .with_a(slot)
                        .with_span(stmt.span),
                );
                let local = &self.program.locals[slot as usize];
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), CompilerSymbol { slot, ty: local.ty });
            }
            StmtKind::If {
                condition,
                then_block,
                else_block,
            } => {
                self.emit_expr(condition, expr_types, user_calls);
                let jump_if_false = self.emit_placeholder(OpCode::JumpIfFalse, condition.span);
                self.push_scope();
                self.emit_block(then_block, expr_types, user_calls);
                self.pop_scope();
                let jump_over_else = self.emit_placeholder(OpCode::Jump, stmt.span);
                self.patch_jump(jump_if_false, self.program.instructions.len());
                self.push_scope();
                self.emit_block(else_block, expr_types, user_calls);
                self.pop_scope();
                self.patch_jump(jump_over_else, self.program.instructions.len());
            }
            StmtKind::Expr(expr) => {
                self.emit_expr(expr, expr_types, user_calls);
                if expr_types.get(&expr.id).and_then(|ty| ty.concrete()) != Some(Type::Void) {
                    self.emit(Instruction::new(OpCode::Pop).with_span(stmt.span));
                }
            }
        }
    }

    fn emit_block(
        &mut self,
        block: &Block,
        expr_types: &HashMap<NodeId, InferredType>,
        user_calls: &HashMap<NodeId, FunctionSpecializationKey>,
    ) {
        for stmt in &block.statements {
            self.emit_stmt(stmt, expr_types, user_calls);
        }
    }

    fn emit_expr(
        &mut self,
        expr: &Expr,
        expr_types: &HashMap<NodeId, InferredType>,
        user_calls: &HashMap<NodeId, FunctionSpecializationKey>,
    ) {
        match &expr.kind {
            ExprKind::Number(value) => {
                let index = self.push_constant(Value::F64(*value));
                self.emit(
                    Instruction::new(OpCode::LoadConst)
                        .with_a(index)
                        .with_span(expr.span),
                );
            }
            ExprKind::Bool(value) => {
                let index = self.push_constant(Value::Bool(*value));
                self.emit(
                    Instruction::new(OpCode::LoadConst)
                        .with_a(index)
                        .with_span(expr.span),
                );
            }
            ExprKind::Na => {
                let index = self.push_constant(Value::NA);
                self.emit(
                    Instruction::new(OpCode::LoadConst)
                        .with_a(index)
                        .with_span(expr.span),
                );
            }
            ExprKind::Ident(name) => match self.lookup_symbol(name) {
                Some(symbol) => self.emit(
                    Instruction::new(OpCode::LoadLocal)
                        .with_a(symbol.slot)
                        .with_span(expr.span),
                ),
                None => self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Compile,
                    format!("unknown identifier `{name}` during emission"),
                    expr.span,
                )),
            },
            ExprKind::Unary { op, expr: inner } => {
                self.emit_expr(inner, expr_types, user_calls);
                let opcode = match op {
                    UnaryOp::Neg => OpCode::Neg,
                    UnaryOp::Not => OpCode::Not,
                };
                self.emit(Instruction::new(opcode).with_span(expr.span));
            }
            ExprKind::Binary { op, left, right } => {
                self.emit_expr(left, expr_types, user_calls);
                self.emit_expr(right, expr_types, user_calls);
                let opcode = match op {
                    BinaryOp::And => OpCode::And,
                    BinaryOp::Or => OpCode::Or,
                    BinaryOp::Add => OpCode::Add,
                    BinaryOp::Sub => OpCode::Sub,
                    BinaryOp::Mul => OpCode::Mul,
                    BinaryOp::Div => OpCode::Div,
                    BinaryOp::Eq => OpCode::Eq,
                    BinaryOp::Ne => OpCode::Ne,
                    BinaryOp::Lt => OpCode::Lt,
                    BinaryOp::Le => OpCode::Le,
                    BinaryOp::Gt => OpCode::Gt,
                    BinaryOp::Ge => OpCode::Ge,
                };
                self.emit(Instruction::new(opcode).with_span(expr.span));
            }
            ExprKind::Call { callee, args } => {
                self.emit_call(expr, callee, args, expr_types, user_calls);
            }
            ExprKind::Index { target, index } => {
                self.emit_series_ref(target, expr_types, user_calls);
                let offset = literal_window(index).unwrap_or_default() as u16;
                self.emit(
                    Instruction::new(OpCode::SeriesGet)
                        .with_a(offset)
                        .with_span(expr.span),
                );
            }
        }
    }

    fn emit_call(
        &mut self,
        expr: &Expr,
        callee: &str,
        args: &[Expr],
        expr_types: &HashMap<NodeId, InferredType>,
        user_calls: &HashMap<NodeId, FunctionSpecializationKey>,
    ) {
        if let Some(key) = user_calls.get(&expr.id) {
            let Some(function) = self.functions_by_id.get(&key.function_id).copied() else {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Compile,
                    format!("unknown function `{callee}` during emission"),
                    expr.span,
                ));
                return;
            };
            let Some(spec) = self.analysis.function_specializations.get(key).cloned() else {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Compile,
                    format!("missing specialization for function `{callee}`"),
                    expr.span,
                ));
                return;
            };

            let mut scope = HashMap::new();
            for ((param, arg), binding) in function
                .params
                .iter()
                .zip(args.iter())
                .zip(spec.param_bindings.iter())
            {
                self.emit_expr(arg, expr_types, user_calls);
                let slot = self.allocate_hidden_slot(binding.ty, binding.kind);
                self.emit(
                    Instruction::new(OpCode::StoreLocal)
                        .with_a(slot)
                        .with_span(arg.span),
                );
                scope.insert(
                    param.name.clone(),
                    CompilerSymbol {
                        slot,
                        ty: binding.ty,
                    },
                );
            }
            self.scopes.push(scope);
            self.emit_expr(&function.body, &spec.expr_types, &spec.user_function_calls);
            self.pop_scope();
            return;
        }

        let Some(builtin) = BuiltinId::from_name(callee) else {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Compile,
                format!("unknown builtin `{callee}`"),
                expr.span,
            ));
            return;
        };

        let callsite = self.next_callsite();
        match builtin {
            BuiltinId::Plot => {
                self.emit_expr(&args[0], expr_types, user_calls);
                self.emit(
                    Instruction::new(OpCode::CallBuiltin)
                        .with_a(builtin as u16)
                        .with_b(1)
                        .with_c(0)
                        .with_span(expr.span),
                );
                self.program.plot_count = 1;
            }
            BuiltinId::Sma | BuiltinId::Ema | BuiltinId::Rsi => {
                self.emit_series_ref(&args[0], expr_types, user_calls);
                self.emit_expr(&args[1], expr_types, user_calls);
                self.emit(
                    Instruction::new(OpCode::CallBuiltin)
                        .with_a(builtin as u16)
                        .with_b(args.len() as u16)
                        .with_c(callsite)
                        .with_span(expr.span),
                );
            }
            _ => {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Compile,
                    format!("builtin `{callee}` is not callable in v0.1"),
                    expr.span,
                ));
            }
        }
    }

    fn emit_series_ref(
        &mut self,
        expr: &Expr,
        expr_types: &HashMap<NodeId, InferredType>,
        user_calls: &HashMap<NodeId, FunctionSpecializationKey>,
    ) {
        match &expr.kind {
            ExprKind::Ident(name) => match self.lookup_symbol(name) {
                Some(symbol) if symbol.ty.is_series() => {
                    self.emit(
                        Instruction::new(OpCode::LoadSeries)
                            .with_a(symbol.slot)
                            .with_span(expr.span),
                    );
                }
                _ => self.emit_materialized_series_ref(expr, expr_types, user_calls),
            },
            _ => self.emit_materialized_series_ref(expr, expr_types, user_calls),
        }
    }

    fn emit_materialized_series_ref(
        &mut self,
        expr: &Expr,
        expr_types: &HashMap<NodeId, InferredType>,
        user_calls: &HashMap<NodeId, FunctionSpecializationKey>,
    ) {
        let ty = match expr_types.get(&expr.id).copied() {
            Some(InferredType::Concrete(Type::Bool | Type::SeriesBool)) => Type::SeriesBool,
            _ => Type::SeriesF64,
        };
        let temp_slot = self.allocate_hidden_slot(ty, SlotKind::Series);
        self.emit_expr(expr, expr_types, user_calls);
        self.emit(
            Instruction::new(OpCode::StoreLocal)
                .with_a(temp_slot)
                .with_span(expr.span),
        );
        self.emit(
            Instruction::new(OpCode::LoadSeries)
                .with_a(temp_slot)
                .with_span(expr.span),
        );
    }

    fn emit(&mut self, instruction: Instruction) {
        self.program.instructions.push(instruction);
    }

    fn emit_placeholder(&mut self, opcode: OpCode, span: Span) -> usize {
        let index = self.program.instructions.len();
        self.program
            .instructions
            .push(Instruction::new(opcode).with_span(span));
        index
    }

    fn patch_jump(&mut self, index: usize, target: usize) {
        self.program.instructions[index].a = target as u16;
    }

    fn push_constant(&mut self, value: Value) -> u16 {
        let index = self.program.constants.len() as u16;
        self.program.constants.push(Constant::Value(value));
        index
    }

    fn allocate_hidden_slot(&mut self, ty: Type, kind: SlotKind) -> u16 {
        let slot = self.program.locals.len() as u16;
        self.program.locals.push(LocalInfo {
            name: None,
            ty,
            kind,
            hidden: true,
        });
        slot
    }

    fn next_callsite(&mut self) -> u16 {
        let callsite = self.builtin_call_count;
        self.builtin_call_count += 1;
        callsite
    }

    fn rebuild_scopes(&mut self) {
        let mut root = HashMap::new();
        for (slot, (name, ty)) in PREDEFINED_SERIES.into_iter().enumerate() {
            root.insert(
                name.to_string(),
                CompilerSymbol {
                    slot: slot as u16,
                    ty,
                },
            );
        }
        self.scopes = vec![root];
    }

    fn lookup_symbol(&self, name: &str) -> Option<CompilerSymbol> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
