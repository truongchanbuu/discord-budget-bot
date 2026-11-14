#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub(crate) enum SpaceError {
    #[error("space access denied")]
    Denied,
    #[error("invalid space input")]
    Invalid,
    #[error("space rate limited")]
    RateLimited,
    #[error("space capacity reached")]
    Capacity,
    #[error("space operation pending")]
    Pending,
    #[error("space permission unavailable")]
    Permission,
    #[error("managed space channel was deleted")]
    ChannelMissing,
    #[error("space revision conflict")]
    Conflict,
    #[error("space storage unavailable")]
    Storage,
    #[error("space operation timed out")]
    Timeout,
    #[error("space operation uncertain")]
    Uncertain,
}
