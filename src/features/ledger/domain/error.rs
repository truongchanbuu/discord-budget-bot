#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub(crate) enum JournalError {
    #[error("Không gửi OTP, mật khẩu, PIN/CVV hay thông tin đăng nhập ngân hàng.")]
    SensitiveInput,
    #[error("Bạn gửi quá nhiều lệnh. Vui lòng chờ một phút.")]
    RateLimited,
    #[error("Không thể gửi phản hồi Discord.")]
    Delivery,
    #[error("Dữ liệu chưa hợp lệ. Hãy kiểm tra số tiền, ngày và quỹ.")]
    InvalidInput,
    #[error("Bạn đã chi/nhận bao nhiêu? Bấm Sửa để nhập số tiền.")]
    AmountRequired,
    #[error("Không có quyền truy cập.")]
    Denied,
    #[error("Bản nháp đã thay đổi. Hãy dùng nút trên bản mới nhất.")]
    Stale,
    #[error("Bản nháp đã hết hạn. Hãy gửi lại tin nhắn.")]
    Expired,
    #[error("Bản nháp đã bị hủy.")]
    Cancelled,
    #[error("Quỹ không đủ tiền tại ngày giao dịch hoặc một ngày sau đó.")]
    InsufficientFunds,
    #[error("Số tiền vượt giới hạn hỗ trợ.")]
    Overflow,
    #[error("Quỹ đã có số dư khởi tạo.")]
    OpeningExists,
    #[error("Tên quỹ đã tồn tại.")]
    Duplicate,
    #[error("Chưa thể lưu/đọc sổ. Vui lòng thử lại với cùng bản nháp.")]
    Storage,
    #[error("Thao tác quá thời gian. Kiểm tra lại bản nháp trước khi thử lại.")]
    Timeout,
}
