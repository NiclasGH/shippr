use crate::Result;
use std::io;

/// Asks the user for an input and returns the given answer
pub(crate) fn user_confirmation(prompt: &str) -> Result<bool> {
    internal_user_confirmation(prompt, io::stdin().lock())
}

/// Testable internal interface
fn internal_user_confirmation<R>(prompt: &str, mut reader: R) -> Result<bool>
where
    R: io::BufRead,
{
    let approvals = ["y", "yes"];

    println!("{prompt}");

    let mut input = String::new();
    reader.read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    Ok(approvals.contains(&input.as_str()))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use rstest::rstest;

    type TestResult = std::result::Result<(), Box<dyn Error>>;

    #[rstest]
    #[case(b"Y")]
    #[case(b"y")]
    #[case(b"Yes")]
    #[case(b"yes")]
    #[case(b"YeS")]
    #[case(b"YES")]
    fn accept_true(#[case] input: &[u8]) -> TestResult {
        // when
        let result = internal_user_confirmation("Unimportant prompt", input)?;

        // then
        assert!(result);

        Ok(())
    }

    #[rstest]
    #[case(b"N")]
    #[case(b"n")]
    #[case(b"no")]
    #[case(b"No")]
    #[case(b"okay")]
    #[case(b"ok")]
    fn deny_false(#[case] input: &[u8]) -> TestResult {
        // when
        let result = internal_user_confirmation("Unimportant prompt", input)?;

        // then
        assert!(!result);

        Ok(())
    }
}
