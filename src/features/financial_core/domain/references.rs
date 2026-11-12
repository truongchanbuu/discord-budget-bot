use super::{CoreError, ReferenceKind};
use std::fmt;

fn checked_reference(value: String, kind: ReferenceKind) -> Result<String, CoreError> {
    if value.trim().is_empty() {
        return Err(CoreError::InvalidReference(kind));
    }
    Ok(value)
}

#[derive(Clone, PartialEq, Eq)]
pub struct OwnerRef(String);

impl OwnerRef {
    pub fn new(value: String) -> Result<Self, CoreError> {
        checked_reference(value, ReferenceKind::Owner).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for OwnerRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OwnerRef([redacted])")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TargetRef(String);

impl TargetRef {
    pub fn new(value: String) -> Result<Self, CoreError> {
        checked_reference(value, ReferenceKind::Target).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for TargetRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetRef([redacted])")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct OriginRef(String);

impl OriginRef {
    pub fn new(value: String) -> Result<Self, CoreError> {
        checked_reference(value, ReferenceKind::Origin).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for OriginRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OriginRef([redacted])")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(value: String) -> Result<Self, CoreError> {
        checked_reference(value, ReferenceKind::Idempotency).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for IdempotencyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IdempotencyKey([redacted])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_role_rejects_blank_references() {
        for input in ["", " \t\n", "\u{2003}"] {
            assert_eq!(
                OwnerRef::new(input.into()).err(),
                Some(CoreError::InvalidReference(ReferenceKind::Owner))
            );
            assert_eq!(
                TargetRef::new(input.into()).err(),
                Some(CoreError::InvalidReference(ReferenceKind::Target))
            );
            assert_eq!(
                OriginRef::new(input.into()).err(),
                Some(CoreError::InvalidReference(ReferenceKind::Origin))
            );
            assert_eq!(
                IdempotencyKey::new(input.into()).err(),
                Some(CoreError::InvalidReference(ReferenceKind::Idempotency))
            );
        }
    }

    #[test]
    fn references_preserve_bytes_and_redact_debug() {
        let input = " Owner-sensitive-marker ";
        let owner = OwnerRef::new(input.into()).unwrap();
        let target = TargetRef::new(input.into()).unwrap();
        let origin = OriginRef::new(input.into()).unwrap();
        let key = IdempotencyKey::new(input.into()).unwrap();
        for value in [
            owner.as_str(),
            target.as_str(),
            origin.as_str(),
            key.as_str(),
        ] {
            assert_eq!(value, input);
        }
        for debug in [
            format!("{owner:?}"),
            format!("{target:#?}"),
            format!("{origin:?}"),
            format!("{key:#?}"),
        ] {
            assert!(!debug.contains(input));
        }
        assert_ne!(
            owner,
            OwnerRef::new("owner-sensitive-marker".into()).unwrap()
        );
    }
}
