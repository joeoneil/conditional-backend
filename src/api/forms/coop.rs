use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::{Datelike, Utc};
use sqlx::{query, query_as};

use crate::{
    api::lib::{open_transaction, UserError},
    app::AppState,
    auth::{CSHAuth, UserInfo},
    schema::{api::CoopSubmission, db::SemesterEnum},
};

#[utoipa::path(
    context_path = "/api/forms",
    tag = "Forms",
    responses(
        (status = 200, description = "Get a user's coop form", body = Option<CoopSubmission>),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(
        ("csh" = [])
    )
)]
#[get("/coop", wrap = "CSHAuth::member_only()")]
pub async fn get_coop_form(
    state: Data<AppState>,
    user: UserInfo,
) -> Result<impl Responder, UserError> {
    let now = Utc::now();
    let form = query_as!(CoopSubmission, r#"
        select u.id, c.year, c.semester as "semester: SemesterEnum" from "user" u left join coop c on u.id = c.uid where c.year > $1 and u.id = $2
        "#,
        if now.month() > 5 {
            now.year()
        } else {
            now.year()-1
        },
        user.get_uid(&state.db).await?
    ).fetch_optional(&state.db).await?;
    Ok(HttpResponse::Ok().json(form))
}

#[utoipa::path(
    context_path = "/api/forms",
    tag = "Forms",
    responses(
        (status = 200, description = "Get all coop forms", body = Vec<CoopSubmission>),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(
        ("csh" = ["eboard"])
    )
)]
#[get("/coops", wrap = "CSHAuth::evals_only()")]
pub async fn get_coop_forms(state: Data<AppState>) -> Result<impl Responder, UserError> {
    let now = Utc::now();
    let form = query_as!(CoopSubmission, r#"
        select u.id, c.year, c.semester as "semester: SemesterEnum" from "user" u left join coop c on u.id = c.uid where c.year > $1
        "#,
        if now.month() > 5 {
            now.year()
        } else {
            now.year()-1
        },
    ).fetch_all(&state.db).await?;
    Ok(HttpResponse::Ok().json(form))
}

#[utoipa::path(
    context_path = "/api/forms",
    tag = "Forms",
    request_body = CoopSubmission,
    responses(
        (status = 200, description = "Submit a coop form"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(
        ("csh" = [])
    )
)]
#[post("/coop", wrap = "CSHAuth::member_only()")]
pub async fn submit_coop_form(
    state: Data<AppState>,
    user: UserInfo,
    body: Json<CoopSubmission>,
) -> Result<impl Responder, UserError> {
    let mut transaction = open_transaction(&state.db).await?;
    query!(
        r#"insert into coop(uid, year, semester) values($1,$2,$3) on conflict do nothing"#,
        user.get_uid(&state.db).await?,
        Utc::now().year(),
        body.semester as SemesterEnum
    )
    .execute(&mut *transaction)
    .await?;
    Ok(HttpResponse::NotImplemented().finish())
}