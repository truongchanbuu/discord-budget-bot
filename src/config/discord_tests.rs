use super::*;
use std::collections::BTreeMap;

fn settings() -> BTreeMap<&'static str, OsString> {
    [
        ("FUNDGUARD_DISCORD_TOKEN", "synthetic-token-marker"),
        ("FUNDGUARD_DISCORD_GUILD_ID", "12"),
        ("FUNDGUARD_DISCORD_OWNER_ID", "34"),
        ("FUNDGUARD_OWNER_REF", " owner-marker "),
        ("FUNDGUARD_OWNER_STATE", "enabled"),
    ]
    .into_iter()
    .map(|(key, value)| (key, value.into()))
    .collect()
}

#[test]
fn every_required_setting_is_validated_without_disclosure() {
    for name in settings().keys().copied() {
        for value in [None, Some(OsString::new())] {
            let mut values = settings();
            if let Some(value) = value {
                values.insert(name, value);
            } else {
                values.remove(name);
            }
            let error = DiscordConfig::load(&mut |key| values.get(key).cloned()).unwrap_err();
            assert!(error.to_string().contains(name));
            assert!(!format!("{error} {error:#?}").contains("marker"));
        }
    }
}

#[test]
fn field_grammars_and_nonblank_owner_bytes_are_preserved() {
    let mut values = settings();
    for (key, invalid) in [
        ("FUNDGUARD_DISCORD_TOKEN", " token "),
        ("FUNDGUARD_DISCORD_GUILD_ID", "01"),
        ("FUNDGUARD_DISCORD_GUILD_ID", "0"),
        ("FUNDGUARD_DISCORD_GUILD_ID", "18446744073709551616"),
        ("FUNDGUARD_DISCORD_GUILD_ID", "+12"),
        ("FUNDGUARD_DISCORD_GUILD_ID", " 12"),
        ("FUNDGUARD_DISCORD_OWNER_ID", "34 "),
        ("FUNDGUARD_DISCORD_OWNER_ID", "00"),
        ("FUNDGUARD_OWNER_STATE", " enabled"),
        ("FUNDGUARD_OWNER_STATE", "ENABLED"),
        ("FUNDGUARD_OWNER_REF", "\u{2003}"),
    ] {
        let saved = values.insert(key, invalid.into());
        assert!(
            DiscordConfig::load(&mut |name| values.get(name).cloned()).is_err(),
            "{key}"
        );
        values.insert(key, saved.unwrap());
    }
    values.insert("FUNDGUARD_DISCORD_GUILD_ID", u64::MAX.to_string().into());
    let config = DiscordConfig::load(&mut |key| values.get(key).cloned()).unwrap();
    assert_eq!(config.guild.get(), u64::MAX);
    let actor =
        crate::features::identity::TrustedActor::new(DiscordAccountId::parse("34").unwrap());
    assert_eq!(
        crate::features::identity::resolve_owner(Some(&actor), Some(&config.binding)).unwrap(),
        &OwnerRef::new(" owner-marker ".into()).unwrap()
    );
    assert!(!format!("{config:?} {config:#?}").contains("marker"));
    assert!(!format!("{:?}", config.token).contains("marker"));
    values.insert("FUNDGUARD_OWNER_STATE", "disabled".into());
    let config = DiscordConfig::load(&mut |key| values.get(key).cloned()).unwrap();
    assert!(crate::features::identity::resolve_owner(Some(&actor), Some(&config.binding)).is_err());
}

#[cfg(unix)]
#[test]
fn nontext_required_fields_fail() {
    use std::os::unix::ffi::OsStringExt;
    for name in settings().keys().copied() {
        let mut values = settings();
        values.insert(name, OsString::from_vec(vec![255]));
        assert!(DiscordConfig::load(&mut |key| values.get(key).cloned()).is_err());
    }
}

#[test]
fn optional_database_url_is_validated_and_redacted() {
    let mut values = settings();
    assert!(
        DiscordConfig::load(&mut |key| values.get(key).cloned())
            .unwrap()
            .database_url
            .is_none()
    );
    values.insert(
        "FUNDGUARD_FINANCE_DATABASE_URL",
        "postgresql://synthetic:secret-marker@localhost/fundguard_test".into(),
    );
    let config = DiscordConfig::load(&mut |key| values.get(key).cloned()).unwrap();
    assert!(config.database_url.is_some());
    assert!(!format!("{config:#?} {:?}", config.database_url).contains("secret-marker"));
    for invalid in ["", " postgres://local/db", "postgres://local/db "] {
        values.insert("FUNDGUARD_FINANCE_DATABASE_URL", invalid.into());
        let error = DiscordConfig::load(&mut |key| values.get(key).cloned()).unwrap_err();
        assert!(error.to_string().contains("FUNDGUARD_FINANCE_DATABASE_URL"));
        assert!(!format!("{error:?}").contains("postgres://"));
    }
}
