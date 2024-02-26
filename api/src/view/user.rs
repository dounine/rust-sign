use crate::base::error::ApiError;
use crate::base::response::{resp_list, resp_ok};
use crate::base::state::AppState;
use crate::base::token;
use crate::base::token::UserData;
use crate::view::base::PageOptions;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use entity::user::{UserStatus, UserType};
use serde::Deserialize;
use tracing::instrument;
use tracing::log::debug;

#[get("")]
#[instrument(skip(state))]
async fn user_list(
    state: Data<AppState>,
    page: Query<PageOptions>,
) -> Result<HttpResponse, ApiError> {
    debug!("进去store查询数据中...");
    let page = page.format();
    service::user::list_user(&state.conn, page.offset, page.limit)
        .await
        .map(|(l, total)| resp_list(l, total).into())
        .map(Ok)?
}

#[get("/{id}")]
#[instrument(skip(state))]
async fn user_detail(
    state: Data<AppState>,
    user: UserData,
    id: Path<i32>,
) -> Result<HttpResponse, ApiError> {
    service::user::find_user_by_id(&state.conn, id.into_inner())
        .await
        .map(|user| resp_ok(user).into())
        .map(Ok)?
}

#[derive(Deserialize, Debug)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login")]
#[instrument(skip(state))]
async fn user_login(
    state: Data<AppState>,
    data: Json<LoginData>,
) -> Result<HttpResponse, ApiError> {
    debug!("login data: {} {}", data.username, data.password);
    let user_query = if data.username.contains("@") {
        service::user::find_user_by_email(&state.conn, data.username.as_str()).await
    } else {
        service::user::find_user_by_username(&state.conn, data.username.as_str()).await
    };
    user_query.map(|user| match user {
        Some(result) => {
            if result.status == UserStatus::Disable {
                return ApiError::msg("用户已被禁用").into();
            }
            if result.password.is_none() {
                return ApiError::msg("帐号或者密码错误").into();
            }

            match util::crypto::md5(data.password.as_str()) == result.password.unwrap_or_default() {
                true => {
                    let token = token::create_token(1, UserType::User, 30).unwrap();
                    Ok(resp_ok(token).into())
                }
                false => ApiError::msg("帐号或者密码错误").into(),
            }
        }
        None => ApiError::msg("用户不存在").into(),
    })?
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .service(user_list)
            .service(user_detail)
            .service(user_login),
    );
}
