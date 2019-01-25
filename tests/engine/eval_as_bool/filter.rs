use crate::test_eval_as_bool_err;

#[test]
fn fail_on_invalid_output_type() {
    // Evaluates to String
    test_eval_as_bool_err!("`true` | LOWER");
}
