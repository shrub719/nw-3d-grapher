fn test_1_1_a() -> bool {
    print!("1.1.a");

    use crate::expr::parser::*;

    let expr = Expr::new("3", true).unwrap();
    
    expr.rpn.into_iter().collect::<Vec<_>>() == vec!(Token::Const(3.0))
}

const TESTS: [fn() -> bool; 1] = [
    test_1_1_a
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
