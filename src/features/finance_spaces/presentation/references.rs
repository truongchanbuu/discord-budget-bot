use super::super::domain::{SpaceError, valid_token};
pub(super) struct Reference<'a> {
    pub(super) action: &'a str,
    pub(super) id: &'a str,
    pub(super) revision: i64,
}
pub(super) fn parse(value: &str) -> Result<Reference<'_>, SpaceError> {
    let parts: Vec<_> = value.split(':').collect();
    if value.len() > 100
        || parts.len() != 4
        || parts[0] != "fgspace"
        || ![
            "manage",
            "add",
            "revoke",
            "select_add",
            "select_revoke",
            "confirm",
            "cancel",
            "recover",
        ]
        .contains(&parts[1])
    {
        return Err(SpaceError::Invalid);
    }
    if parts[3].is_empty() || !parts[3].bytes().all(|b| b.is_ascii_digit()) {
        return Err(SpaceError::Invalid);
    }
    let revision = parts[3].parse::<i64>().map_err(|_| SpaceError::Invalid)?;
    if revision < 0 {
        return Err(SpaceError::Invalid);
    }
    if ["confirm", "cancel"].contains(&parts[1]) {
        if !valid_token(parts[2]) {
            return Err(SpaceError::Invalid);
        }
    } else if parts[2].is_empty()
        || !parts[2].bytes().all(|b| b.is_ascii_digit())
        || parts[2].starts_with('0')
    {
        return Err(SpaceError::Invalid);
    }
    Ok(Reference {
        action: parts[1],
        id: parts[2],
        revision,
    })
}
