use crate::assets::*;
use lang_c::ast::*;

pub fn compile(code: &str) -> CompilationStatus {
    let code = code
        .split('\n')
        .filter(|line| !line.starts_with('#'))
        .filter(|line| !line.trim().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
        .replace('$', "");
    match lang_c::driver::parse_preprocessed(&lang_c::driver::Config::default(), code) {
        Result::Ok(program) => {
            let mut loops = Vec::new();
            for object in program.unit.0 {
                if let ExternalDeclaration::FunctionDefinition(node) = object.node {
                    if let DeclaratorKind::Identifier(id) = node.node.declarator.node.kind.node {
                        if id.node.name == "main" {
                            // Process main function code
                            if let Statement::Compound(nodes) = node.node.statement.node {
                                for node in nodes {
                                    if let BlockItem::Statement(statement) = node.node {
                                        if let Statement::While(statement) = statement.node {
                                            if let Statement::Compound(statements) =
                                                statement.node.statement.node
                                            {
                                                loops.push(Loop::new(
                                                    statements
                                                        .into_iter()
                                                        .filter_map(|statement| {
                                                            if let BlockItem::Statement(statement) =
                                                                statement.node
                                                            {
                                                                Some(statement.node)
                                                            } else {
                                                                None
                                                            }
                                                        })
                                                        .collect(),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            CompilationStatus::Success(loops)
        }
        Err(_) => CompilationStatus::Failure,
    }
}

#[derive(Clone, Debug)]
pub enum CompilationStatus {
    None,
    Success(Vec<Loop>),
    Failure,
}

#[derive(Clone, Debug)]
pub struct Loop {
    statements: Vec<Statement>,
}

impl Loop {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    #[allow(clippy::collapsible_match)]
    pub fn iteration<F: FnMut(&str) -> bool>(&self, mut api_layer: F) -> u32 {
        for statement in &self.statements {
            let breaks = execute(&mut api_layer, statement);
            if breaks > 0 {
                return breaks;
            }
        }
        0
    }
}

fn evaluate<F: FnMut(&str) -> bool>(api_layer: &mut F, expression: &Expression) -> bool {
    match expression {
        Expression::Call(node) => {
            if let Expression::Identifier(name) = &node.node.callee.node {
                api_layer(&name.node.name)
            } else {
                println!(
                    "Warning: unimplemented function expression: {:#?}",
                    node.node.callee.node
                );
                false
            }
        }
        Expression::BinaryOperator(node) => {
            if node.node.operator.node == BinaryOperator::LogicalAnd {
                evaluate(api_layer, &node.node.lhs.node) && evaluate(api_layer, &node.node.rhs.node)
            } else {
                println!(
                    "Warning: unimplemented binary operator: {:#?}",
                    node.node.operator.node
                );
                false
            }
        }
        _ => {
            println!("Warning: unimplemented expression: {:#?}", expression);
            false
        }
    }
}

fn execute<F: FnMut(&str) -> bool>(api_layer: &mut F, statement: &Statement) -> u32 {
    match statement {
        Statement::Expression(expr) => {
            if let Some(expr) = expr {
                if let Expression::Call(node) = &expr.node {
                    if let Expression::Identifier(name) = &node.node.callee.node {
                        api_layer(&name.node.name);
                    }
                }
            }
            0
        }
        Statement::Break => 1,
        Statement::Return(_) => u32::MAX,
        Statement::If(statement) => {
            if evaluate(api_layer, &statement.node.condition.node) {
                return execute(api_layer, &statement.node.then_statement.node);
            }
            0
        }
        Statement::Goto(_) => todo!(),
        _ => 0,
    }
}
