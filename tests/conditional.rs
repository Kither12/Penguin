mod common;
#[cfg(test)]
mod tests {
    use crate::{test_code_failed, test_code_ok};

    #[test]
    fn empty_nested_if() {
        test_code_ok!(
            "
                if true{
                    if true{
                        if false{

                        }
                        elif true{

                        }
                    }
                    else{

                    }
                }
                elif false{

                }
                else{}
            "
        )
    }
}
