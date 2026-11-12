use fundguard::features::{
    financial_core::{CoreError, OwnerRef, ReferenceKind},
    identity::{AccessError, DiscordAccountId, TrustedActor},
};

#[test]
fn canonical_accounts_are_exact_and_bounded() {
    let one = DiscordAccountId::parse("1").unwrap();
    let other = DiscordAccountId::parse("42").unwrap();
    let maximum = DiscordAccountId::parse("18446744073709551615").unwrap();
    assert_eq!(one, DiscordAccountId::parse("1").unwrap());
    assert_ne!(one, other);
    assert_ne!(maximum, other);
    let actor = TrustedActor::new(other);
    assert_eq!(actor, TrustedActor::new(other));
    assert_ne!(actor, TrustedActor::new(one));
}

#[test]
fn invalid_account_forms_are_rejected_without_normalization() {
    for input in [
        "",
        " ",
        "\t\n",
        "\u{2003}",
        "0",
        "00",
        "01",
        "+1",
        "-1",
        " 42",
        "42 ",
        "４２",
        "٤٢",
        "1.0",
        "1e3",
        "1,000",
        "<@42>",
        "name",
        "18446744073709551616",
        "123456789012345678901",
    ] {
        assert_eq!(
            DiscordAccountId::parse(input),
            Err(AccessError::InvalidAccountId)
        );
    }
}

#[test]
fn existing_owner_validation_and_byte_equality_remain_authoritative() {
    for input in ["", " \t\n", "\u{2003}"] {
        assert_eq!(
            OwnerRef::new(input.into()),
            Err(CoreError::InvalidReference(ReferenceKind::Owner))
        );
    }
    let owner = OwnerRef::new(" Owner-A ".into()).unwrap();
    assert_eq!(owner.as_str(), " Owner-A ");
    assert_ne!(owner, OwnerRef::new("Owner-A".into()).unwrap());
    assert_ne!(owner, OwnerRef::new(" owner-a ".into()).unwrap());
}
