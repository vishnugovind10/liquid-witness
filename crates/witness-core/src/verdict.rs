use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Verdict {
    Verified,
    Mismatch,
    Incomplete,
    Demo,
}

impl Verdict {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Verdict::Verified => ExitCode(0),
            Verdict::Mismatch => ExitCode(1),
            Verdict::Incomplete => ExitCode(2),
            Verdict::Demo => ExitCode(3),
        }
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verdict::Verified => f.write_str("VERIFIED"),
            Verdict::Mismatch => f.write_str("MISMATCH"),
            Verdict::Incomplete => f.write_str("INCOMPLETE"),
            Verdict::Demo => f.write_str("DEMO"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExitCode(pub i32);

#[cfg(test)]
mod tests {
    use super::Verdict;

    #[test]
    fn verdict_exit_codes_are_audit_pipeline_stable() {
        assert_eq!(Verdict::Verified.exit_code().0, 0);
        assert_eq!(Verdict::Mismatch.exit_code().0, 1);
        assert_eq!(Verdict::Incomplete.exit_code().0, 2);
        assert_eq!(Verdict::Demo.exit_code().0, 3);
    }
}
