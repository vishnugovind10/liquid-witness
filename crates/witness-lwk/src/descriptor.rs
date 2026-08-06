use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatchOnlyDescriptor(String);

#[derive(Debug, Error)]
pub enum DescriptorError {
    #[error("descriptor is empty")]
    Empty,
    #[error("descriptor appears to contain signer material")]
    SignerMaterial,
}

impl WatchOnlyDescriptor {
    pub fn parse(value: impl Into<String>) -> Result<Self, DescriptorError> {
        let value = value.into();
        let lower = value.to_ascii_lowercase();
        if value.trim().is_empty() {
            return Err(DescriptorError::Empty);
        }
        if lower.contains("xprv") || lower.contains("tprv") || lower.contains("seed") {
            return Err(DescriptorError::SignerMaterial);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn redacted_scope(&self) -> String {
        let prefix: String = self.0.chars().take(18).collect();
        format!("{prefix}...")
    }
}

#[cfg(test)]
mod tests {
    use super::{DescriptorError, WatchOnlyDescriptor};

    #[test]
    fn rejects_empty_descriptor() {
        assert!(matches!(
            WatchOnlyDescriptor::parse(" "),
            Err(DescriptorError::Empty)
        ));
    }

    #[test]
    fn rejects_private_key_material_markers() {
        assert!(matches!(
            WatchOnlyDescriptor::parse("ct(slip77(tprv8Zgx...),elwpk(...))"),
            Err(DescriptorError::SignerMaterial)
        ));
    }
}
