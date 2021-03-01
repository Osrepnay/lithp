use crate::parse::Expr;
use std::collections::HashMap;

pub fn run(exprs: Vec<Expr>) -> Result<(), String> {
    let mut builtin_funcs: HashMap<
        String,
        fn(&mut Environment, &Vec<Expr>) -> Result<Expr, String>,
    > = HashMap::new();
    builtin_funcs.insert(
        "=".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            if args.len() != 2 {
                return Err(format!(
                    "Wrong number of arguments; expected 2, got {}",
                    args.len()
                ));
            }
            let name = match &args[0] {
                Expr::Identifier(name) => name.clone(),
                _ => return Err("Expected identifier for argument 0, did not get identifier".to_owned()),
            };
            let value = eval_non_literal(env, &args[1].clone())?;
            env.vars.insert(name, value.clone());
            Ok(value)
        },
    );
    builtin_funcs.insert(
        "func".to_owned(),
        |_env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            if args.len() < 1 {
                return Err(format!(
                    "Too little arguments; expected 1 or more, got {}",
                    args.len()
                ));
            }
            for arg_idx in 0..args.len() - 1 {
                match args[arg_idx] {
                    Expr::Identifier(_) => (),
                    _ => {
                        return Err(format!(
                            "Expected identifier for argument {}, did not get identifier",
                            arg_idx
                        ))
                    }
                }
            }
            match args.last().unwrap() {
                Expr::FuncCall(_, _) => (),
                _ => {
                    return Err(format!(
                        "Expected function call for argument {}, did not get function call",
                        args.len() - 1
                    ))
                }
            }
            Ok(Expr::FuncCall("func".to_owned(), args.clone()))
        },
    );
    builtin_funcs.insert(
        "print".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            if args.len() != 1 {
                return Err(format!(
                    "Wrong number of arguments; expected 1, got {}",
                    args.len()
                ));
            }
            let value = eval_non_literal(env, &args[0])?;
            println!("{}", to_string(&value)?);
            Ok(Expr::StringLiteral(to_string(&value)?))
        },
    );
    builtin_funcs.insert(
        "*".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            let change_if_larger = |curr_largest: &mut Expr, poss_largest: &Expr| {
                match curr_largest {
                    Expr::DoubleLiteral(_) => (),
                    Expr::LongLiteral(_) => if let Expr::DoubleLiteral(_) = poss_largest {
                        *curr_largest = poss_largest.clone();
                    }
                    Expr::IntLiteral(_) => {
                        if let Expr::LongLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                        if let Expr::DoubleLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                    }
                    // Not possible
                    _ => (),
                }
            };
            let mut largest_value = Expr::IntLiteral(1);
            let mut product = 1.0;
            for arg in args {
                match eval_non_literal(env, &arg) {
                    Ok(Expr::IntLiteral(int)) => {
                        change_if_larger(&mut largest_value, &Expr::IntLiteral(1));
                        product *= int as f64;
                    }
                    Ok(Expr::LongLiteral(long)) => {
                        change_if_larger(&mut largest_value, &Expr::LongLiteral(1));
                        product *= long as f64;
                    }
                    Ok(Expr::DoubleLiteral(double)) => {
                        change_if_larger(&mut largest_value, &Expr::DoubleLiteral(1.0));
                        product *= double;
                    }
                    Ok(_) => return Err("Not a numeric value".to_owned()),
                    Err(err) => return Err(err),
                }
            }
            Ok(match largest_value {
                Expr::IntLiteral(_) => Expr::IntLiteral(product as i32),
                Expr::LongLiteral(_) => Expr::LongLiteral(product as i64),
                Expr::DoubleLiteral(_) => Expr::DoubleLiteral(product),
                _ => return Err("Not a numeric value".to_owned()),
            })
        },
    );
    builtin_funcs.insert(
        "/".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            let change_if_larger = |curr_largest: &mut Expr, poss_largest: &Expr| {
                match curr_largest {
                    Expr::DoubleLiteral(_) => (),
                    Expr::LongLiteral(_) => if let Expr::DoubleLiteral(_) = poss_largest {
                        *curr_largest = poss_largest.clone();
                    }
                    Expr::IntLiteral(_) => {
                        if let Expr::LongLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                        if let Expr::DoubleLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                    }
                    // Not possible
                    _ => (),
                }
            };
            let mut largest_value = Expr::IntLiteral(1);
            let mut quotient = match eval_non_literal(env, &args[0])? {
                Expr::DoubleLiteral(double) => double,
                Expr::LongLiteral(long) => long as f64,
                Expr::IntLiteral(int) => int as f64,
                _ => return Err("Not a numeric value".to_owned()),
            };
            for arg in &args[1..] {
                match eval_non_literal(env, &arg) {
                    Ok(Expr::IntLiteral(int)) => {
                        change_if_larger(&mut largest_value, &Expr::IntLiteral(1));
                        quotient /= int as f64;
                    }
                    Ok(Expr::LongLiteral(long)) => {
                        change_if_larger(&mut largest_value, &Expr::LongLiteral(1));
                        quotient /= long as f64;
                    }
                    Ok(Expr::DoubleLiteral(double)) => {
                        change_if_larger(&mut largest_value, &Expr::DoubleLiteral(1.0));
                        quotient /= double;
                    }
                    Ok(_) => return Err("Not a numeric value".to_owned()),
                    Err(err) => return Err(err),
                }
            }
            Ok(match largest_value {
                Expr::IntLiteral(_) => Expr::IntLiteral(quotient as i32),
                Expr::LongLiteral(_) => Expr::LongLiteral(quotient as i64),
                Expr::DoubleLiteral(_) => Expr::DoubleLiteral(quotient),
                _ => return Err("Not a numeric value".to_owned()),
            })
        },
    );
    builtin_funcs.insert(
        "+".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            let change_if_larger = |curr_largest: &mut Expr, poss_largest: &Expr| {
                match curr_largest {
                    Expr::DoubleLiteral(_) => (),
                    Expr::LongLiteral(_) => if let Expr::DoubleLiteral(_) = poss_largest {
                        *curr_largest = poss_largest.clone();
                    }
                    Expr::IntLiteral(_) => {
                        if let Expr::LongLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                        if let Expr::DoubleLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                    }
                    // Not possible
                    _ => (),
                }
            };
            let mut largest_value = Expr::IntLiteral(1);
            let mut sum = 0.0;
            for arg in args {
                match eval_non_literal(env, &arg) {
                    Ok(Expr::IntLiteral(int)) => {
                        change_if_larger(&mut largest_value, &Expr::IntLiteral(1));
                        sum += int as f64;
                    }
                    Ok(Expr::LongLiteral(long)) => {
                        change_if_larger(&mut largest_value, &Expr::LongLiteral(1));
                        sum += long as f64;
                    }
                    Ok(Expr::DoubleLiteral(double)) => {
                        change_if_larger(&mut largest_value, &Expr::DoubleLiteral(1.0));
                        sum += double;
                    }
                    Ok(_) => return Err("Not a numeric value".to_owned()),
                    Err(err) => return Err(err),
                }
            }
            Ok(match largest_value {
                Expr::IntLiteral(_) => Expr::IntLiteral(sum as i32),
                Expr::LongLiteral(_) => Expr::LongLiteral(sum as i64),
                Expr::DoubleLiteral(_) => Expr::DoubleLiteral(sum),
                _ => return Err("Not a numeric value".to_owned()),
            })
        },
    );
    builtin_funcs.insert(
        "-".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            let change_if_larger = |curr_largest: &mut Expr, poss_largest: &Expr| {
                match curr_largest {
                    Expr::DoubleLiteral(_) => (),
                    Expr::LongLiteral(_) => if let Expr::DoubleLiteral(_) = poss_largest {
                        *curr_largest = poss_largest.clone();
                    }
                    Expr::IntLiteral(_) => {
                        if let Expr::LongLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                        if let Expr::DoubleLiteral(_) = poss_largest {
                            *curr_largest = poss_largest.clone();
                        }
                    }
                    // Not possible
                    _ => (),
                }
            };
            let mut largest_value = Expr::IntLiteral(1);
            let mut difference = match eval_non_literal(env, &args[0])? {
                Expr::DoubleLiteral(double) => double,
                Expr::LongLiteral(long) => long as f64,
                Expr::IntLiteral(int) => int as f64,
                _ => return Err("Not a numeric value".to_owned()),
            };
            for arg in &args[1..] {
                match eval_non_literal(env, &arg) {
                    Ok(Expr::IntLiteral(int)) => {
                        change_if_larger(&mut largest_value, &Expr::IntLiteral(1));
                        difference -= int as f64;
                    }
                    Ok(Expr::LongLiteral(long)) => {
                        change_if_larger(&mut largest_value, &Expr::LongLiteral(1));
                        difference -= long as f64;
                    }
                    Ok(Expr::DoubleLiteral(double)) => {
                        change_if_larger(&mut largest_value, &Expr::DoubleLiteral(1.0));
                        difference -= double;
                    }
                    Ok(_) => return Err("Not a numeric value".to_owned()),
                    Err(err) => return Err(err),
                }
            }
            Ok(match largest_value {
                Expr::IntLiteral(_) => Expr::IntLiteral(difference as i32),
                Expr::LongLiteral(_) => Expr::LongLiteral(difference as i64),
                Expr::DoubleLiteral(_) => Expr::DoubleLiteral(difference),
                _ => return Err("Not a numeric value".to_owned()),
            })
        },
    );
    builtin_funcs.insert(
        "==".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            for arg_idx in 0..args.len() - 1 {
                let eq_left = eval_non_literal(env, &args[arg_idx])?;
                let eq_right = eval_non_literal(env, &args[arg_idx + 1])?;
                match eq_left {
                    Expr::IntLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if eq_right != eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right != eq_left as i64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right != eq_left as f64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::LongLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if eq_right as i64 != eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right != eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right != eq_left as f64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::DoubleLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if eq_right as f64 != eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right as f64 != eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right != eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::StringLiteral(eq_left) => if let Expr::StringLiteral(eq_right) = eq_right {
                        if eq_left != eq_right {
                            return Ok(Expr::BooleanLiteral(false));
                        }
                    } else {
                        return Err("Arguments are not the same type".to_owned());
                    }
                    // Hopefully not possible
                    _ => (),
                }
            }
            Ok(Expr::BooleanLiteral(true))
        }
    );
    builtin_funcs.insert(
        ">".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            for arg_idx in 0..args.len() - 1 {
                let eq_left = eval_non_literal(env, &args[arg_idx])?;
                let eq_right = eval_non_literal(env, &args[arg_idx + 1])?;
                match eq_left {
                    Expr::IntLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if eq_right >= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right >= eq_left as i64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right >= eq_left as f64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::LongLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if eq_right as i64 >= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right >= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right >= eq_left as f64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::DoubleLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if eq_right as f64 >= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right as f64 >= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right >= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    _ => return Err("Cannot compare these types".to_owned()),
                }
            }
            Ok(Expr::BooleanLiteral(true))
        }
    );
    builtin_funcs.insert(
        "<".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            for arg_idx in 0..args.len() - 1 {
                let eq_left = eval_non_literal(env, &args[arg_idx])?;
                let eq_right = eval_non_literal(env, &args[arg_idx + 1])?;
                match eq_left {
                    Expr::IntLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if eq_right <= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right <= eq_left as i64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right <= eq_left as f64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::LongLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if (eq_right as i64) <= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if eq_right <= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right <= eq_left as f64 {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::DoubleLiteral(eq_left) => {
                        match eq_right {
                            Expr::IntLiteral(eq_right) => if (eq_right as f64) <= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::LongLiteral(eq_right) => if (eq_right as f64) <= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            Expr::DoubleLiteral(eq_right) => if eq_right <= eq_left {
                                return Ok(Expr::BooleanLiteral(false));
                            }
                            _ => return Err("Arguments are not the same type".to_owned()),
                        }
                    }
                    Expr::StringLiteral(eq_left) => if let Expr::StringLiteral(eq_right) = eq_right {
                        if eq_left != eq_right {
                            return Ok(Expr::BooleanLiteral(false));
                        }
                    } else {
                        return Err("Arguments are not the same type".to_owned());
                    }
                    _ => return Err("Cannot compare these types".to_owned()),
                }
            }
            Ok(Expr::BooleanLiteral(true))
        }
    );
    builtin_funcs.insert(
        "ifElse".to_owned(),
        |env: &mut Environment, args: &Vec<Expr>| -> Result<Expr, String> {
            if args.len() != 3 {
                return Err(format!(
                    "Wrong number of arguments; expected 3, got {}",
                    args.len()
                ));
            }
            let exec_expr = eval_non_literal(env, &args[0])?;
            match exec_expr {
                Expr::BooleanLiteral(boolean) => {
                    if boolean {
                        Ok(args[1].clone())
                    } else {
                        Ok(args[2].clone())
                    }
                }
                _ => return Err("Expected boolean for argument 0, did not get boolean".to_owned()),
            }
        },
    );
    let mut env = Environment {
        vars: HashMap::new(),
        builtin_funcs,
    };
    for expr in exprs {
        match expr {
            Expr::FuncCall(name, args) => {
                let clone = env.clone();
                if let Some(function) = clone.builtin_funcs.get(&name) {
                    function(&mut env, &args)?
                } else {
                    return Err("Undeclared function".to_owned());
                }
            }
            _ => return Err("Unsupported operation".to_owned()),
        };
    }
    Ok(())
}

fn eval_non_literal(env: &mut Environment, expr: &Expr) -> Result<Expr, String> {
    match expr {
        Expr::Identifier(name) => {
            let env_clone = env.clone();
            if let Some(var) = env_clone.vars.get(name) {
                eval_non_literal(env, var)
            } else {
                Err("Undeclared variable".to_owned())
            }
        }
        Expr::FuncCall(name, args) => {
            if *name == "func".to_owned() {
                return Ok(expr.clone());
            }
            let env_clone = env.clone();
            if let Some(func) = env_clone.vars.get(name) {
                let mut env_shadow = env.clone();
                let func_expr = match func {
                    Expr::FuncCall(name, func_args) => {
                        if *name != "func".to_owned() {
                            return Err(format!("{} is not a function", name));
                        }
                        if func_args.len() - 1 != args.len() {
                            return Err(format!(
                                "Wrong number of arguments; expected {}, got {}",
                                func_args.len() - 1,
                                args.len()
                            ));
                        }
                        for arg_idx in 0..args.len() {
                            match &func_args[arg_idx] {
                                Expr::Identifier(name) => {
                                    env_shadow.vars.insert(name.clone(), eval_non_literal(env, &args[arg_idx])?);
                                }
                                _ => (),
                            }
                        }
                        func_args.last().unwrap()
                    }
                    _ => return Err("Not a function".to_owned()),
                };
                eval_non_literal(&mut env_shadow, func_expr)
            } else if let Some(func) = env_clone.builtin_funcs.get(name) {
                let expr = func(env, args)?;
                eval_non_literal(env, &expr)
            } else {
                Err("Undeclared function".to_owned())
            }
        }
        _ => Ok(expr.clone()),
    }
}

fn to_string(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::StringLiteral(string) => Ok(string.clone()),
        Expr::IntLiteral(int) => Ok(int.to_string()),
        Expr::LongLiteral(long) => Ok(long.to_string()),
        Expr::DoubleLiteral(double) => Ok(double.to_string()),
        Expr::BooleanLiteral(boolean) => Ok(boolean.to_string()),
        _ => Err("Cannot convert identifier or function to string".to_owned()),
    }
}

#[derive(Clone)]
pub struct Environment {
    vars: HashMap<String, Expr>,
    builtin_funcs: HashMap<String, fn(&mut Environment, &Vec<Expr>) -> Result<Expr, String>>,
}
