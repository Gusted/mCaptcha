/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use db_core::prelude::*;

use crate::api::v1::mcaptcha::get_random;
use crate::errors::*;
use crate::AppData;

#[my_codegen::get(
    path = "crate::V1_API_ROUTES.account.get_secret",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn get_secret(id: Identity, data: AppData) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let secret = data.db.get_secret(&username).await?;
    Ok(HttpResponse::Ok().json(secret))
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.update_secret",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn update_user_secret(
    id: Identity,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let mut secret;

    loop {
        secret = get_random(32);

        match data.db.update_secret(&username, &secret).await {
            Ok(_) => break,
            Err(DBError::SecretTaken) => continue,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(HttpResponse::Ok())
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_secret);
    cfg.service(update_user_secret);
}
