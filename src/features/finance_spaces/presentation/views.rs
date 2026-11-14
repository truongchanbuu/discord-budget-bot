use serenity::builder::{CreateAllowedMentions, CreateMessage};
pub(super) const PRIVACY: &str = "Chỉ chủ kênh được ghi/sửa và quản lý quyền. Người xem được đọc lịch sử và tin nhắn sau này. Chủ máy chủ/Administrator vẫn có thể xem. Thu hồi quyền không xóa thông tin đã đọc hoặc sao chép; không có mật khẩu kênh.";
pub(super) fn message(text: impl Into<String>) -> CreateMessage {
    CreateMessage::new()
        .content(text)
        .allowed_mentions(
            CreateAllowedMentions::new()
                .empty_users()
                .empty_roles()
                .everyone(false),
        )
        .flags(serenity::model::channel::MessageFlags::SUPPRESS_EMBEDS)
}
pub(super) fn welcome() -> String {
    format!(
        "Kênh tài chính sẵn sàng. Journal cũ chỉ được giữ cho tài khoản đã liên kết; thành viên mới bắt đầu với số dư 0. Báo cáo do chủ yêu cầu có thể hiển thị cả số liệu cũ cho người xem.\nDùng @bot trợ giúp · @bot nhận lương 16tr · @bot chi 100k ăn uống · @bot quyền\n{PRIVACY}"
    )
}
