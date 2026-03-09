use crate::*;

fn test_tokenisation(expr: &str, expected: Vec<expr::parser::Token>) -> bool {
    use expr::parser::*;

    let expr = Expr::new(expr, true).unwrap();

    expr.rpn.into_iter().collect::<Vec<_>>() == expected
}

fn test_evaluation(expr: &str, expected: f32) -> bool {
    use expr::parser::*;

    let expr = Expr::new(expr, true).unwrap();
    let result = expr.eval(0.0, 0.0, 0.0).unwrap();

    result == expected
}

fn test_evaluation_approx(expr: &str, expected: f32) -> bool {
    use expr::parser::*;

    let expr = Expr::new(expr, true).unwrap();
    let result = expr.eval(0.0, 0.0, 0.0).unwrap();

    (result * 100.0).floor() == (expected * 100.0).floor()
}

fn t11a() -> bool {
    print!("1.1.a");

    use expr::parser::Token::*;
    test_tokenisation(
        "3",
        vec!(Const(3.0))
    )
}

fn t11b() -> bool {
    print!("1.1.b");

    use expr::parser::Token::*;
    test_tokenisation(
        "1 1 +",
        vec!(Const(1.0), Const(1.0), Add)
    )
}

fn t11c() -> bool {
    print!("1.1.c");

    use expr::parser::Token::*;
    test_tokenisation(
        "x",
        vec!(X)
    )
}

fn t11d() -> bool {
    print!("1.1.d");

    use expr::parser::Token::*;
    test_tokenisation(
        "0 sin",
        vec!(Const(0.0), Sin)
    )
}

fn t22a() -> bool {
    print!("2.2.a");
    test_evaluation("1 1 +", 2.0)
}

fn t22b() -> bool {
    print!("2.2.b");
    test_evaluation_approx("3.14159 sin", 0.0)
}

// TODO: all the other functions...

const TESTS: &[fn() -> bool] = &[
    t11a, t11b, t11c, t11d,
    t22a, t22b
];

pub fn run_all() {
    println!("running tests\n");

    for test in TESTS {
        let passed = test();
        let result = if passed { "passed" } else { "failed" };
        println!(": {}", result);
    }

    println!("\ntesting complete");
}
