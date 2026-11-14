# Finance DM Contract

Finance is enabled only with FUNDGUARD_FINANCE_DATABASE_URL in explicit Discord mode. This optional
redacted setting is ignored offline. Existing guild /fundguard readiness stays unchanged. No .env
file auto-loading. DB connect/migrate bounded15s; pool<=4, acquire/action<=5s, send<=2s.

Only owner-authenticated DM messages and finance-prefixed components/modals are handled. Ordinary
server messages, bots, unknown actors and disabled owner never access financial data. No MESSAGE_CONTENT
privilege. Every action rechecks policy/owner/channel/revision/expiry. Denial: “Không có quyền truy cập.”

Examples (whole VND, case-insensitive Vietnamese/ASCII aliases):
- “tôi vừa donate 100k cho vtuber A” → expense, Giải trí, Chi tiêu, current local date, original note.
- “nhận lương 15tr” / “nhận freelance 1tr” → received income into Chi tiêu.
- “tiết kiệm 3tr” / “đầu tư 2tr” → transfer Chi tiêu to corresponding reserve fund.
- “số dư đầu kỳ 2tr” → one-time opening for default fund; route editable.
- Optional “ngày YYYY-MM-DD”; no future financial dates or expected-income language.
- “quỹ”, “tạo quỹ Du lịch”, “tổng kết 2026-11”, “báo cáo”, “trợ giúp”.

Incomplete supported phrases create an explicitly incomplete preview and Sửa form, never an invented
amount. Unknown chatter returns short help only. Decimal, zero, negative, multiple amount and overflow
cannot commit. Notes<=1000 chars; fund/category<=80; amount input<=32; date exact10.

Preview shows kind, amount/missing, category, fund route, date, note and manual/virtual label. Buttons:
Lưu / Sửa / Hủy. IDs: fgfinance:<action>:<positive draft id>:<revision>; no owner/financial payload.
Edit modal exactly5 inputs with stable ids: amount/category/route/date/note. Route one fund or
source -> destination. Draft kind fixed; cancel/resend to change kind. Modal submit returns a new
preview, never directly posts. Duplicate/unknown/missing modal inputs fail safely.

Confirm/cancel/modal submission authenticate then privately defer before any mutation; ack failure
means no mutation. Edit performs owner-scoped fetch under<=2s then INITIAL modal response (no defer).
Result sends/edit responses have empty allowed mentions and suppress embeds. Never print provider or
DB errors, show foreign data, or copy financial input into logs. A failed receipt after commit never
rolls back or repeats the financial effect.

Monthly output labels: Đầu kỳ/khởi tạo, Thu đã nhận, Đã chi, Chuyển vào tiết kiệm/đầu tư, Tổng cuối kỳ,
Còn ở quỹ chi tiêu, chi theo danh mục and chi/thu percentage (unavailable if thu=0). State virtual,
manually reported, not bank verified. No image/report export or market valuation.

## Supabase connection boundary (2026-11-14)

FUNDGUARD_FINANCE_DATABASE_URL now targets Supabase-managed PostgreSQL (direct/session5432),
not a Supabase HTTP URL or API token. Known transaction pooler6543 is rejected. Remote TLS cannot
fall back to plaintext; VerifyCa/VerifyFull stay intact. Local fixture URLs remain supported only
for development/test compatibility. Errors contain no URL, password, host or provider detail.
Versioned migration0002 denies Data API roles and enables default-deny RLS; owner backend retains
journal behavior. No new public application API, identity source or financial semantics.
