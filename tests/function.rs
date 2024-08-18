mod common;
#[cfg(test)]
mod tests {
    use crate::{test_code_failed, test_code_ok};

    #[test]
    fn function() {
        test_code_ok!(
            "
                gimme a = () => {};
                a();
            "
        )
    }
    #[test]
    fn function_argument() {
        test_code_ok!(
            "
                gimme a = (a, b) => {};
                a(2, 3);
            "
        );
        test_code_ok!(
            "
                gimme a = (a, b) => {
                    gimme c = (a) => {};
                    c(2);
                };
                a(2, 3);
            "
        )
    }
    #[test]
    fn function_argument_not_match() {
        test_code_failed!(
            "
                gimme a = (a, b) => {};
                a(2);
            "
        );
        test_code_failed!(
            "
                gimme a = () => {};
                a(2);
            "
        )
    }
    #[test]
    fn function_argument_redeclare_should_fail() {
        test_code_failed!(
            "
                gimme a = (a) => {
                    gimme a = 0;
                };
                a(2);
            "
        );
    }
}
