use super::*;

#[test]
fn offline_does_not_read_discord_fields() {
    for value in [None, Some(OsString::from("offline"))] {
        let mut names = Vec::new();
        let result = RuntimeConfig::load(&mut |name| {
            names.push(name.to_owned());
            value.clone()
        });
        assert!(matches!(result, Ok(RuntimeConfig::Offline)));
        assert_eq!(names, ["FUNDGUARD_MODE"]);
    }
}

#[test]
fn invalid_modes_fail_without_fallback() {
    for value in ["", "DISCORD", " discord", "offline ", "invalid-mode-marker"] {
        let error = RuntimeConfig::load(&mut |_| Some(value.into())).unwrap_err();
        assert!(error.to_string().contains("FUNDGUARD_MODE"));
        assert!(!format!("{error:?}").contains("invalid-mode-marker"));
    }
}

#[cfg(unix)]
#[test]
fn nontext_mode_fails() {
    use std::os::unix::ffi::OsStringExt;
    assert!(RuntimeConfig::load(&mut |_| Some(OsString::from_vec(vec![255]))).is_err());
}
