use actix_governor::{
    Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor,
};
use actix_optional_middleware::{Dummy, Group};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::Error;
use std::rc::Rc;

lazy_static::lazy_static! {
    pub static ref RATE_LIMIT_CONFIG: Option<GovernorConfig<PeerIpKeyExtractor>> =
    if let (Some(period), Some(burst_size)) =
        (crate::SETTINGS.server.rate_limit_period,
         crate::SETTINGS.server.rate_limit_burst_size)
    {
        if cfg!(test) {
            return None;
        }
        let gov_cfg = GovernorConfigBuilder::default()
            .per_millisecond(period)
            .burst_size(burst_size)
            .finish()
            .expect("Invalid rate limiter configuration.");
        log::info!("Rate limiter initialized");
        Some(gov_cfg)
    } else {
        None
    };
}

pub fn get_rate_limit_middleware<S>() -> Group<Dummy, Governor<PeerIpKeyExtractor>, S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    if let Some(rate_limit_cfg) = &*RATE_LIMIT_CONFIG {
        Group::Real(Rc::new(Governor::new(rate_limit_cfg)))
    } else {
        Group::default()
    }
}
