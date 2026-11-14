use super::*;
use serenity::json::to_value;
#[test]
fn routes_and_generic_views_never_echo_financial_input() {
    assert_eq!(classify("bắt đầu"), Route::Start);
    assert_eq!(classify("quyền"), Route::Access);
    assert_eq!(classify("chi 100k secret-marker"), Route::Finance);
    let value = to_value(views::message(views::welcome())).unwrap();
    assert!(!value["content"].as_str().unwrap().contains("secret-marker"));
    assert!(
        value["allowed_mentions"]["parse"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(views::welcome().contains("Administrator"));
}
