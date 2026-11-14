use super::*;
use std::collections::BTreeMap;

fn load(
    category: Option<&str>,
    channels: Option<&str>,
    database: bool,
) -> Result<DiscordConfig, ApplicationError> {
    let mut values: BTreeMap<&str, OsString> = [
        ("FUNDGUARD_DISCORD_TOKEN", "synthetic-token"),
        ("FUNDGUARD_DISCORD_GUILD_ID", "12"),
        ("FUNDGUARD_DISCORD_OWNER_ID", "34"),
        ("FUNDGUARD_OWNER_REF", "test-owner"),
        ("FUNDGUARD_OWNER_STATE", "enabled"),
    ]
    .into_iter()
    .map(|(k, v)| (k, v.into()))
    .collect();
    if let Some(value) = category {
        values.insert("FUNDGUARD_FINANCE_CATEGORY_ID", value.into());
    }
    if let Some(value) = channels {
        values.insert("FUNDGUARD_FINANCE_ONBOARDING_CHANNEL_IDS", value.into());
    }
    if database {
        values.insert(
            "FUNDGUARD_FINANCE_DATABASE_URL",
            "postgresql://localhost/fundguard_test".into(),
        );
    }
    DiscordConfig::load(&mut |name| values.get(name).cloned())
}

#[test]
fn absent_configuration_preserves_existing_mode() {
    assert!(load(None, None, false).unwrap().spaces.is_none());
}

#[test]
fn independent_overrides_require_storage() {
    assert!(load(Some("20"), Some("21"), false).is_err());
}

#[test]
fn canonical_ids_are_bounded_unique_and_redacted() {
    for invalid in [
        "",
        "0",
        "01",
        "+20",
        " 20",
        "18446744073709551616",
        "secret-marker",
    ] {
        let error = load(Some(invalid), Some("21"), true).unwrap_err();
        assert!(!format!("{error:?}").contains("secret-marker"));
    }
    for invalid in [
        "",
        "0",
        "01",
        "21,21",
        "21,",
        "21,+22",
        "21,22,23,24,25,26,27,28,29",
    ] {
        assert!(load(Some("20"), Some(invalid), true).is_err());
    }
    for count in 1..=8 {
        let channels = (21..21 + count)
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let config = load(Some("20"), Some(&channels), true).unwrap();
        let spaces = config.spaces.unwrap();
        assert_eq!(spaces.category.unwrap().get(), 20);
        assert_eq!(spaces.onboarding.len(), count as usize);
        assert!(!format!("{spaces:?}").contains("21"));
    }
}

#[test]
fn storage_enables_defaults_and_each_override_is_independent() {
    let defaults = load(None, None, true).unwrap().spaces.unwrap();
    assert!(defaults.category.is_none());
    assert!(defaults.onboarding.is_empty());
    let category = load(Some("20"), None, true).unwrap().spaces.unwrap();
    assert_eq!(category.category.unwrap().get(), 20);
    assert!(category.onboarding.is_empty());
    let channels = load(None, Some("21"), true).unwrap().spaces.unwrap();
    assert!(channels.category.is_none());
    assert_eq!(channels.onboarding[0].get(), 21);
    for pair in [(Some("20"), None), (None, Some("21"))] {
        assert!(load(pair.0, pair.1, false).is_err());
    }
}
