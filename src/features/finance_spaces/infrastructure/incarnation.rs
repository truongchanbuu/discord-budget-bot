use super::super::domain::SpaceError;
use chrono::{DateTime, SecondsFormat, Utc};

pub(super) fn encode(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Nanos, true)
}

pub(super) fn decode(value: &str) -> Result<DateTime<Utc>, SpaceError> {
    let instant = DateTime::parse_from_rfc3339(value)
        .map_err(|_| SpaceError::Storage)?
        .with_timezone(&Utc);
    if value.len() != 30 || encode(instant) != value {
        return Err(SpaceError::Storage);
    }
    Ok(instant)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    #[test]
    fn nanosecond_identity_survives_encoding_and_distinguishes_adjacent_instants() {
        let first = "2026-11-14T00:00:00.123456789Z"
            .parse::<DateTime<Utc>>()
            .unwrap();
        let next = first + chrono::Duration::nanoseconds(1);
        assert_eq!(decode(&encode(first)).unwrap(), first);
        assert_ne!(encode(first), encode(next));
        assert_eq!(decode(&encode(next)).unwrap(), next);
    }

    #[test]
    fn equivalent_offsets_encode_to_one_canonical_utc_representation() {
        let local = DateTime::parse_from_rfc3339("2026-11-14T07:00:00.123456789+07:00")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(encode(local), "2026-11-14T00:00:00.123456789Z");
        let whole = "2026-11-14T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
        assert_eq!(encode(whole), "2026-11-14T00:00:00.000000000Z");
    }

    #[test]
    fn malformed_or_noncanonical_storage_is_rejected_without_retaining_input() {
        for value in [
            "secret-marker",
            "2026-02-30T00:00:00.000000000Z",
            "2026-11-14T00:00:00Z",
            "2026-11-14T00:00:00.1234567890Z",
            "2026-11-14T07:00:00.123456789+07:00",
        ] {
            let error = decode(value).unwrap_err();
            assert!(!format!("{error:?} {error}").contains(value));
        }
    }
}
