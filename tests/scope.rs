mod common;
#[cfg(test)]
mod tests {
    use crate::{test_code_failed, test_code_ok};

    #[test]
    fn empty_scope() {
        test_code_ok!(
            "
                {}
                if true {}
                if false {} else {}
                if false {} elif false {} else {}
                while false {}    
            "
        )
    }
    #[test]
    fn scope() {
        test_code_ok!(
            "
                gimme a = 0;
                {
                    gimme a = 1;
                }
                a = 0;
            "
        )
    }
    #[test]
    fn scope_redeclare() {
        test_code_failed!(
            "
                gimme a = 0;
                {
                    gimme a = 1;
                }
                gimme a = 1;
            "
        );
        test_code_failed!(
            "
                gimme a = 0;
                gimme a = () => {};
            "
        );
    }
}
