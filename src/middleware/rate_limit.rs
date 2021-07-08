use actix_governor::{Governor, GovernorConfig};

lazy_static::lazy_static! {
    static ref AUTH_LIMITER: GovernorConfig = GovernorConfig::secure();
}

pub fn auth_rate_limiter() -> Governor {
    Governor::new(&AUTH_LIMITER)
}
