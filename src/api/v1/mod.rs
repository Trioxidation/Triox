/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

// TODO setup governor
pub mod account;
pub mod auth;
#[cfg(test)]
mod tests;

use account::routes::Account;
use auth::routes::Auth;
pub const ROUTES: Routes = Routes::new();

pub struct Routes {
    pub auth: Auth,
    pub account: Account,
}

impl Routes {
    const fn new() -> Routes {
        Routes {
            auth: Auth::new(),
            account: Account::new(),
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    auth::services(cfg);
    account::services(cfg);
}
