#[cfg(test)]
pub mod evaluator_test {
    use crate::{
        evaluator::evaluator::eval,
        lexer::lexer::Lexer,
        object::{env::Environment, object::Object},
        parser::parser::Parser,
    };

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        eval(parser.parse_program(), &mut Environment::new())
    }

    fn eval_integer_object(obj: Object, expected_value: i32) {
        assert_eq!(obj, Object::INTEGER(expected_value));
    }

    fn eval_boolean_object(obj: Object, expected_value: bool) {
        assert_eq!(obj, Object::BOOLEAN(expected_value));
    }

    #[test]
    fn eval_integer_expression() {
        let input_expctdvalue = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("0", 0),
            ("-0", 0),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_integer_object(test_eval(i), *v));
    }

    #[test]
    fn eval_boolean_expression() {
        let input_expctdvalue = vec![
            ("false", false),
            ("true", true),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_boolean_object(test_eval(i), *v));
    }

    #[test]
    fn eval_bang_operator() {
        let input_expctdvalue = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!0", true),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
            ("!!0", false),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_boolean_object(test_eval(i), *v));
    }

    #[test]
    fn eval_if_expression() {
        let input_expctdvalue = vec![
            ("if (true) { 10 }", Object::INTEGER(10)),
            ("if (false) { 10 }", Object::NULL),
            ("if (1) { 10 }", Object::INTEGER(10)),
            ("if (1 < 2) { 10 }", Object::INTEGER(10)),
            ("if (1 > 2) { 10 }", Object::NULL),
            ("if (1 > 2) { 10 } else { 20 }", Object::INTEGER(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::INTEGER(10)),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| assert_eq!(test_eval(i), *v));
    }

    #[test]
    fn eval_return_statement() {
        let input_expctdvalue = vec![
            ("9; 8;", 8),
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 6;", 10),
        ];

        input_expctdvalue
            .iter()
            .map(|(i, v)| (test_eval(i), v))
            .map(|(i, v)| {
                (
                    match i {
                        Object::RETURN(r) => *r,
                        obj => obj,
                    },
                    v,
                )
            })
            .for_each(|(i, v)| eval_integer_object(i, *v));
    }

    #[test]
    fn eval_error_handling() {
        let input_expctdvalue = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| assert_eq!(test_eval(i), Object::ERROR(v.to_string())));
    }

    #[test]
    fn eval_let_statement() {
        let input_expctdvalue = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_integer_object(test_eval(i), *v))
    }
}
