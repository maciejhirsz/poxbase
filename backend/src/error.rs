// PoxBase
// Copyright (C) 2020  Maciej Hirsz
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use actix_web::{http, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Not Found")]
pub struct NotFound;

impl ResponseError for NotFound {
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::NotFound()
            .content_type("application/json")
            .body(r#"{"error":"Item not found"}"#)
    }
}
