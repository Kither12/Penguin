mod common;
#[cfg(test)]
mod tests {
    use crate::{test_code_failed, test_code_ok};

    #[test]
    fn while_loop() {
        test_code_ok!(
            "
                gimme a = 0;
                while a < 10{
                    a += 1;
                }
            "
        )
    }
}
