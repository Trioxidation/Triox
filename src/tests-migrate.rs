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
use lazy_static::lazy_static;

mod app_state;
mod cli;
mod config;

pub use app_state::AppState as Data;

lazy_static! {
    pub static ref SETTINGS: config::AppConfig = {
        let cli_options = cli::Options::new();
        config::AppConfig::new(cli_options.config_dir.as_ref()).unwrap()
    };
}

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() {
    let data = Data::new().await;

    sqlx::migrate!("./migrations/").run(&data.db).await.unwrap();
}
