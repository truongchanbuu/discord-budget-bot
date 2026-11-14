use super::super::domain::MemberRef;
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Route {
    Start,
    Access,
    Finance,
}
pub(crate) fn classify(input: &str) -> Route {
    match input.trim().to_lowercase().as_str() {
        "" | "bắt đầu" | "bat dau" => Route::Start,
        "quyền" | "quyen" => Route::Access,
        _ => Route::Finance,
    }
}
pub(crate) fn leading_mention(content: &str, bot: MemberRef, explicit: bool) -> Option<&str> {
    if !explicit {
        return None;
    }
    for prefix in [format!("<@{}>", bot.get()), format!("<@!{}>", bot.get())] {
        if let Some(rest) = content.strip_prefix(&prefix)
            && (rest.is_empty() || rest.starts_with(char::is_whitespace))
        {
            return Some(rest.trim());
        }
    }
    None
}
