use crate::*;

fn test_tokenisation(expr: &str, expected: Vec<expr::parser::Token>) -> bool {
    use expr::parser::*;

    let expr = Expr::new(expr, true).unwrap();

    expr.rpn.into_iter().collect::<Vec<_>>() == expected
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

const TESTS: &[fn() -> bool] = &[
    t11a, t11b, t11c, t11d
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
