use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::frontend::ast::{
    ColumnAssignmentEnum, Declaration, Expr, Operator, Parameter, Statement, TypeConstruct,
};

use super::{
    environment::{
        EnvironmentCell, WrenchFunction, env_add,
        env_expand_scope, env_get, env_new, env_shrink_scope, env_to_closure, env_update,
    },
    library::{wrench_import, wrench_print, wrench_table_add_row},
    pipes::evaluate_pipes,
    table::{Row, Table, TableCell, TableCellType},
};

// Represents the value of an evaluated expression in the Wrench language
#[derive(Clone, Debug)]
pub enum ExpressionValue {
    Number(i32),
    Double(f64),
    String(String),
    Bool(bool),
    Table(Rc<RefCell<Table>>),
    Row(Row),
    Array(Vec<ExpressionValue>),
    Null,
}

//Represents the value of a statement in the Wrench language. Either the statement returns something or nothing
#[derive(Debug)]
pub enum StatementValue {
    None,
    Return(ExpressionValue),
}

/*
 * This file deals with evaluating the AST
 */

pub fn interpret(input: Statement) {
    let mut env = env_new();
    env_expand_scope(&mut env);
    evaluate_statement(input, &mut env);
}

//Evaluate S in Stmt
fn evaluate_statement(statement: Statement, env: &mut Vec<Vec<EnvironmentCell>>) -> StatementValue {
    //Matches D
    match statement {
        Statement::Declaration(declaration) => {
            evaluate_declaration(declaration, env);
            StatementValue::None
        }
        //Matches e
        Statement::Expr(expression) => {
            evaluate_expression(*expression, env);
            StatementValue::None
        }
        //Matches x = e
        Statement::VariableAssignment(variable, expression) => {
            let evaluated_value = evaluate_expression(*expression, env);
            env_update(env, &variable, evaluated_value);
            StatementValue::None
        }
        //Matches S1;S2
        Statement::Compound(s1, s2) => {
            let s1v = evaluate_statement(*s1, env);

            if let StatementValue::Return(_) = s1v {
                return s1v;
            }

            let s2v: StatementValue = evaluate_statement(*s2, env);

            match s2v {
                StatementValue::Return(_) => s2v,
                StatementValue::None => StatementValue::None,
            }
        }
        //Matches skip
        Statement::Skip => StatementValue::None,
        //Matches return e
        Statement::Return(expression) => {
            let return_value = evaluate_expression(*expression, env);
            StatementValue::Return(return_value)
        }
        //Matches if (e) then {S1} else {S2}
        Statement::If(e1, s1, s2) => {
            let condition = evaluate_expression(*e1, env);
            match condition {
                ExpressionValue::Bool(true) => evaluate_statement(*s1, env),
                ExpressionValue::Bool(false) => evaluate_statement(*s2, env),
                _ => {
                    panic!("Interpretation error: Condition is not a boolean")
                }
            }
        }
        //Matches for (T x in e) {S}
        Statement::For(parameter, expression, body) => {
            let iterator = evaluate_expression(*expression, env);
            let Parameter::Parameter(_, n) = parameter;
            match iterator {
                ExpressionValue::Table(table) => {
                    let table = table.borrow();
                    for row in table.iter() {
                        env_expand_scope(env);
                        env_add(
                            env,
                            EnvironmentCell::Variable(n.clone(), ExpressionValue::Row(row.clone())),
                        );
                        let statement_value = evaluate_statement(*body.clone(), env);
                        match statement_value {
                            StatementValue::Return(value) => {
                                env_shrink_scope(env);
                                return StatementValue::Return(value);
                            }
                            StatementValue::None => {}
                        }
                        env_shrink_scope(env);
                    }
                    StatementValue::None
                }
                ExpressionValue::Array(array) => {
                    for element in array {
                        env_expand_scope(env);
                        env_add(env, EnvironmentCell::Variable(n.clone(), element));
                        let statement_value = evaluate_statement(*body.clone(), env);
                        match statement_value {
                            StatementValue::Return(value) => {
                                env_shrink_scope(env);
                                return StatementValue::Return(value);
                            }
                            StatementValue::None => {}
                        }
                        env_shrink_scope(env);
                    }
                    StatementValue::None
                }
                _ => {
                    panic!("Interpretation error: For loop iterator is not a table")
                }
            }
        }
        //Matches while(e){S}
        Statement::While(e, body) => {
            loop {
                let condition = evaluate_expression(*e.clone(), env);
                env_expand_scope(env);
                match condition {
                    ExpressionValue::Bool(true) => {
                        let statement_value = evaluate_statement(*body.clone(), env);
                        match statement_value {
                            StatementValue::Return(value) => {
                                env_shrink_scope(env);
                                return StatementValue::Return(value);
                            }
                            StatementValue::None => {}
                        }
                    }
                    ExpressionValue::Bool(false) => {
                        env_shrink_scope(env);
                        break;
                    }
                    _ => {
                        panic!("Interpretation error: Condition is not a boolean")
                    }
                }
                env_shrink_scope(env);
            }
            StatementValue::None
        }
    }
}

//Evaluate D in Decl
fn evaluate_declaration(declaration: Declaration, env: &mut Vec<Vec<EnvironmentCell>>) {
    match declaration {
        //Matches var T x = e
        Declaration::Variable(_, var_name, value) => {
            let evaluated_value = evaluate_expression(*value, env);
            env_add(env, EnvironmentCell::Variable(var_name, evaluated_value));
        }
        //Matches const T x = e
        Declaration::Constant(_, var_name, value) => {
            let evaluated_value = evaluate_expression(*value, env);
            env_add(env, EnvironmentCell::Variable(var_name, evaluated_value));
        }
        //Matches function T x (T x) {S}
        Declaration::Function(func_type, func_name, parameters, body) => {
            let closure = env_to_closure(&env.clone());
            env_add(
                env,
                EnvironmentCell::Function(WrenchFunction::new(
                    func_type,
                    func_name,
                    parameters,
                    Box::new(*body),
                    closure,
                )),
            );
        }
    }
}

//Evaluate e in Expr
pub fn evaluate_expression(
    expression: Expr,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> ExpressionValue {
    match expression {
        //Matches null
        Expr::Null => ExpressionValue::Null,
        //Matches n
        Expr::Number(n) => ExpressionValue::Number(n),
        //Matches d
        Expr::Double(d) => ExpressionValue::Double(d),
        //Matches b
        Expr::Bool(b) => ExpressionValue::Bool(b),
        //Matches s
        Expr::StringLiteral(s) => ExpressionValue::String(s),
        //Matches e1 o e2
        Expr::Operation(e1, op, e2) => {
            let left = evaluate_expression(*e1, env);
            let right = evaluate_expression(*e2, env);
            evaluate_operation(left, op, right)
        }

        //Matches x
        Expr::Identifier(ref name) => match env_get(env, name) {
            EnvironmentCell::Variable(_, ref value) => value.clone(),
            EnvironmentCell::Function(..) => {
                panic!("Interpretation error: Function identifier not allowed as expression")
            }
        },
        //Matches x(e)
        Expr::FunctionCall(name, expressions) => {
            let mut args: Vec<ExpressionValue> = Vec::with_capacity(expressions.len());
            for expression in expressions {
                args.push(evaluate_expression(*expression, env));
            }
            evaluate_function_call(name, args, env)
        }
        //Matches row(T x = e)
        Expr::Row(column_assignment) => {
            let mut row: Vec<(String, TableCell)> = Vec::new();
            for assignment in column_assignment {
                match assignment {
                    ColumnAssignmentEnum::ColumnAssignment(_, name, expression) => {
                        let evaluated_value = evaluate_expression(*expression, env);
                        match evaluated_value {
                            ExpressionValue::Number(n) => {
                                row.push((name.clone(), TableCell::Int(n)));
                            }
                            ExpressionValue::String(s) => {
                                row.push((name.clone(), TableCell::String(s)));
                            }
                            ExpressionValue::Bool(b) => {
                                row.push((name.clone(), TableCell::Bool(b)));
                            }
                            ExpressionValue::Double(d) => {
                                row.push((name.clone(), TableCell::Double(d)));
                            }
                            _ => {
                                panic!("Interpretation error: Unsupported type in row assignment")
                            }
                        }
                    }
                }
            }
            ExpressionValue::Row(Row::new(row))
        }
        //Matches table(T x)
        Expr::Table(params) => {
            let mut structure: HashMap<String, TableCellType> = HashMap::new();
            for param in params {
                match param {
                    Parameter::Parameter(t, name) => match t {
                        TypeConstruct::Bool => {
                            structure.insert(name.clone(), TableCellType::Bool);
                        }
                        TypeConstruct::Int => {
                            structure.insert(name.clone(), TableCellType::Int);
                        }
                        TypeConstruct::String => {
                            structure.insert(name.clone(), TableCellType::String);
                        }
                        TypeConstruct::Double => {
                            structure.insert(name.clone(), TableCellType::Double);
                        }
                        _ => {
                            panic!("Interpretation error: Unsupported type in table declaration")
                        }
                    },
                }
            }
            ExpressionValue::Table(Rc::new(RefCell::new(Table::new(structure))))
        }
        //Matches e1 pipe x(e2)
        Expr::Pipe(expression, function_name, args) => {
            let args: Vec<Expr> = args.into_iter().map(|b| *b).collect();
            evaluate_pipes(expression, function_name, args, env)
        }
        //Matches !e
        Expr::Not(expr) => {
            let evaluated_value = evaluate_expression(*expr, env);
            match evaluated_value {
                ExpressionValue::Bool(b) => ExpressionValue::Bool(!b),
                _ => {
                    panic!(
                        "Interpretation error: Not operator can only be applied to boolean values"
                    )
                }
            }
        }
        //Matches e.x
        Expr::ColumnIndexing(expr, column) => {
            let evaluated_value = evaluate_expression(*expr, env);
            match evaluated_value {
                ExpressionValue::Row(row) => row.get(&column),
                ExpressionValue::Table(table) => table.borrow().get_column(&column),
                _ => {
                    panic!(
                        "Interpretation error: Column indexing can only be applied to rows or tables"
                    )
                }
            }
        }
        //Matches [e]
        Expr::Array(elements) => {
            let mut evaluated_elements: Vec<ExpressionValue> = Vec::new();
            for element in elements {
                evaluated_elements.push(evaluate_expression(*element, env));
            }
            ExpressionValue::Array(evaluated_elements)
        }
        //Matches e1[e2]
        Expr::Indexing(expr, index) => {
            let evaluated_value = evaluate_expression(*expr, env);
            match evaluated_value {
                ExpressionValue::Array(array) => {
                    let int_index = match evaluate_expression(*index, env) {
                        ExpressionValue::Number(n) => n as usize,
                        _ => {
                            panic!("Interpretation error: Index must be a integer")
                        }
                    };
                    if int_index < array.len() {
                        array[int_index].clone()
                    } else {
                        panic!("Interpretation error: Index out of bounds");
                    }
                }
                _ => {
                    panic!("Interpretation error: Indexing can only be applied to arrays")
                }
            }
        }
    }
}

//Evaluate o in Op
pub fn evaluate_function_call(
    name: String,
    args: Vec<ExpressionValue>,
    env: &[Vec<EnvironmentCell>],
) -> ExpressionValue {
    match name.as_str() {
        "print" => wrench_print(args),
        "import" => wrench_import(args),
        "table_add_row" => wrench_table_add_row(args),
        _ => {
            let function = env_get(env, &name);
            if let EnvironmentCell::Function(wrench_function) = function {
                let mut fun_env = wrench_function.get_closure_as_env();
                for (param, arg) in wrench_function.parameters.iter().zip(args.into_iter()) {
                    let Parameter::Parameter(_, param_name) = param;
                    env_add(
                        &mut fun_env,
                        EnvironmentCell::Variable(param_name.clone(), arg),
                    );
                }
                env_add(
                    &mut fun_env,
                    EnvironmentCell::Function(wrench_function.clone()),
                );

                let statement_value =
                    evaluate_statement(*wrench_function.body.clone(), &mut fun_env);
                match statement_value {
                    StatementValue::Return(value) => value,
                    StatementValue::None => ExpressionValue::Null,
                }
            } else {
                panic!(
                    "Interpretation error: Identifier '{:?}' is not a function",
                    name
                );
            }
        }
    }
}

pub fn evaluate_custom_function_call(
    function: &WrenchFunction,
    args: Vec<ExpressionValue>,
) -> ExpressionValue {
    let mut fun_env = function.get_closure_as_env();
    for (param, arg) in function.parameters.iter().zip(args.into_iter()) {
        let Parameter::Parameter(_, param_name) = param;
        env_add(
            &mut fun_env,
            EnvironmentCell::Variable(param_name.clone(), arg),
        );
    }
    env_add(&mut fun_env, EnvironmentCell::Function(function.clone()));

    let statement_value = evaluate_statement(*function.body.clone(), &mut fun_env);
    match statement_value {
        StatementValue::Return(value) => value,
        StatementValue::None => ExpressionValue::Null,
    }
}

fn evaluate_operation(
    left: ExpressionValue,
    operator: Operator,
    right: ExpressionValue,
) -> ExpressionValue {
    match operator {
        Operator::Addition => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Number(l + r);
            } else if let (ExpressionValue::String(l), ExpressionValue::String(r)) = (&left, &right)
            {
                return ExpressionValue::String(format!("{}{}", l, r));
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Double(l + r);
            }
        }
        Operator::Subtraction => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Number(l - r);
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Double(l - r);
            }
        }
        Operator::Or => {
            if let (ExpressionValue::Bool(l), ExpressionValue::Bool(r)) = (&left, &right) {
                return ExpressionValue::Bool(*l || *r);
            }
        }
        Operator::LessThan => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Bool(l < r);
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Bool(l < r);
            }
        }
        Operator::LessThanOrEqual => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Bool(l <= r);
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Bool(l <= r);
            }
        }
        Operator::Multiplication => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Number(l * r);
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Double(l * r);
            }
        }
        Operator::Modulo => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Number(l % r);
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Double(l % r);
            }
        }
        Operator::Equals => {
            if let (ExpressionValue::Bool(l), ExpressionValue::Bool(r)) = (&left, &right) {
                return ExpressionValue::Bool(l == r);
            } else if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right)
            {
                return ExpressionValue::Bool(l == r);
            } else if let (ExpressionValue::String(l), ExpressionValue::String(r)) = (&left, &right)
            {
                return ExpressionValue::Bool(l == r);
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Bool(l == r);
            }
        }
        Operator::Division => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Number(l / r);
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Double(l / r);
            }
        }
        Operator::Exponent => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Number(l.pow(*r as u32));
            } else if let (ExpressionValue::Double(l), ExpressionValue::Double(r)) = (&left, &right)
            {
                return ExpressionValue::Double(l.powf(*r));
            }
        }
    }
    panic!(
        "Interpretation error: Unsupported operation for {:?} {:?} {:?}",
        &left, &operator, &right,
    );
}
