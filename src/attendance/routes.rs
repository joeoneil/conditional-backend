use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_json::json;
use sqlx::{query, query_as};
use crate::schema::db;
mod schema;

#[post("/attendance/seminar")]
pub async fn submit_seminar_attendance(state: Data<AppState>, body: SeminarAttendance) -> impl Responder {
    // TODO: eboard should auto approve
    let id = match query_as!(i32, "INSERT INTO technical_seminars(name, timestamp, active, approved) OUTPUT INSERTED.id VALUES ($1::varchar(128), $2::timestamp, true, $3::bool", body.name, body.date, false)
        .fetch_one(&state.db)
        .await
    {
        Ok(id) => id,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let usernames: Vec<&String> = body.body.iter();
    let frosh: Vec<u32> = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            c.unwrap().is_numeric()
        }
    }).map(|a| *a.parse()).collect();
    let members = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            !c.unwrap().is_numeric()
        }
    }).collect::<Vec<_>>();
    let seminar_id_vec = vec![id; usernames.len()];
    match query!("DELETE FROM freshman_seminar_attendance WHERE seminar_id = ($1::i32); DELETE FROM member_seminar_attendance WHERE seminar_id = ($2::i32); INSERT INTO freshman_seminar_attendance(fid, seminar_id) SELECT * FROM UNNEST($3::int4[], $4::int4[]); INSERT INTO member_seminar_attendance(uid, seminar_id) SELECT * FROM UNNEST($5::text[], $6::int4[]);", id, id, frosh, seminar_id_vec, members, seminar_id_vec)
        .execute(&state.db)
        .await
        {
            Ok(_) => HttpResponse::Ok(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
}

#[get("/attendance/seminar/{user}")]
pub async fn get_seminars_by_user(state: Data<AppState>) -> impl Responder {
    // TODO: authenticate with token
    let (name,) = path.into_inner();
    if name.len() < 1 {
        return HttpResponse::BadRequest().body("No name found".to_string());
    }
    match query_as!(Seminar, format!("SELECT * FROM {} WHERE approved = 'true' AND {} = $1 AND seminar_id IN (SELECT id FROM technical_seminars WHERE timestamp > ($2::timestamp))", if name.chars().next().is_numeric() { "freshman_seminar_attendance" } else { "member_seminar_attendance" }, if name.chars().next().is_numeric() { "fid" } else { "uid" }), body.name, &state.year_start)
        .fetch_all(&state.db)
        .await
    {
        Ok(seminars) => HttpResponse::Ok().json(seminars),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/attendance/seminar")]
pub async fn get_seminars(state: Data<AppState>) -> impl Responder {
    // TODO: Joe: year_start should be the day the new year button was pressed by Evals, formatted for postgres
    match query_as!(Seminar, "SELECT * FROM technical_seminars WHERE timestamp > ($1::timestamp)", &state.year_start)
        .fetch_all(&state.db)
        .await
    {
        Ok(seminars) => HttpResponse::Ok().json(seminars),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/attendance/seminar/{id}")]
pub async fn put_seminar(state: Data<AppState>, body: Json<String>) -> impl Responder {
    let (id,) = path.into_inner();
    let usernames: Vec<&String> = body.iter();
    let frosh: Vec<u32> = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            c.unwrap().is_numeric()
        }
    }).map(|a| *a.parse()).collect();
    let members = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            !c.unwrap().is_numeric()
        }
    }).collect::<Vec<_>>();
    let seminar_id_vec = vec![id; usernames.len()];
    match query!("DELETE FROM freshman_seminar_attendance WHERE seminar_id = ($1::i32); DELETE FROM member_seminar_attendance WHERE seminar_id = ($2::i32); INSERT INTO freshman_seminar_attendance(fid, seminar_id) SELECT * FROM UNNEST($3::int4[], $4::int4[]); INSERT INTO member_seminar_attendance(uid, seminar_id) SELECT * FROM UNNEST($5::text[], $6::int4[]);", id, id, frosh, seminar_id_vec, members, seminar_id_vec)
        .execute(&state.db)
        .await
        {
            Ok(_) => HttpResponse::Ok(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
}

#[delete("/attendance/seminar/{id}")]
pub async fn delete_seminar(state: Data<Appstate>) -> impl Responder {
    let (id,) = path.into_inner();
    match query!("DELETE FROM technical_seminars WHERE id = ($1::int4); DELETE FROM freshman_seminar_attendance WHERE seminar_id = ($2::int4); DELETE FROM member_seminar_attendance WHERE seminar_id = ($3::int4);", id, id, id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// TODO: Joe: committee is used over directorship to maintain parity with db
#[post("/attendance/committee")]
pub async fn submit_committee_attendance(state: Data<AppState>, body: SeminarAttendance) -> impl Responder {
    // TODO: eboard should auto approve
    let id = match query_as!(i32, "INSERT INTO committee_meetings(committee, timestamp, active, approved) OUTPUT INSERTED.id VALUES ($1::varchar(128), $2::timestamp, true, $3::bool", body.name, body.date, false)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    };

    let usernames: Vec<&String> = body.iter();
    let frosh: Vec<u32> = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            c.unwrap().is_numeric()
        }
    }).map(|a| *a.parse()).collect();
    let members = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            !c.unwrap().is_numeric()
        }
    }).collect::<Vec<_>>();
    let committee_id_vec = vec![id; usernames.len()];
    match query!("DELETE FROM freshman_committee_attendance WHERE meeting_id = ($1::i32); DELETE FROM member_committee_attendance WHERE meeting_id = ($2::i32); INSERT INTO freshman_committee_attendance(fid, meeting_id) SELECT * FROM UNNEST($3::int4[], $4::int4[]); INSERT INTO member_committee_attendance(uid, meeting_id) SELECT * FROM UNNEST($5::text[], $6::int4[]);", id, id, frosh, committee_id_vec, members, committee_id_vec)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/attendance/committee/{user}")]
pub async fn get_committees_by_user(state: Data<AppState>) -> impl Responder {
    // TODO: authenticate with token
    let (name,) = path.into_inner();
    if name.len() < 1 {
        return HttpResponse::BadRequest().body("No name found".to_string());
    }
    match query_as!(Committee, format!("SELECT * FROM {} WHERE approved = 'true' AND {} = $1 AND committee_id IN (SELECT id FROM committee_meetings WHERE timestamp > ($2::timestamp))", if name.chars().next().is_numeric() { "freshman_committee_attendance" } else { "member_committee_attendance" }, if name.chars().next().is_numeric() { "fid" } else { "uid" }), body.name, &state.year_start)
        .fetch_all(&state.db)
        .await
    {
        Ok(committees) => HttpResponse::Ok().json(committees),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/attendance/committee")]
pub async fn get_committee(state: Data<AppState>) -> impl Responder {
    // TODO: Joe: year_start should be the day the new year button was pressed by Evals, formatted for postgres
    match query_as!(Committee, "SELECT * FROM committee_meetings WHERE timestamp > ($1::timestamp)", &state.year_start)
        .fetch_all(&state.db)
        .await
    {
        Ok(committees) => HttpResponse::Ok().json(committees),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/attendance/committee/{id}")]
pub async fn put_committee(state: Data<AppState>, body: Json<String>) -> impl Responder {
    let (id,) = path.into_inner();
    let usernames: Vec<&String> = body.iter();
    let frosh: Vec<u32> = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            c.unwrap().is_numeric()
        }
    }).map(|a| *a.parse()).collect();
    let members = usernames.filter(|a| {
        let c = a.chars().next();
        if c.is_some() {
            !c.unwrap().is_numeric()
        }
    }).collect::<Vec<_>>();
    let committee_id_vec = vec![id; usernames.len()];
    match query!("DELETE FROM freshman_committee_attendance WHERE meeting_id = ($1::i32); DELETE FROM member_committee_attendance WHERE meeting_id = ($2::i32); INSERT INTO freshman_committee_attendance(fid, meeting_id) SELECT * FROM UNNEST($3::int4[], $4::int4[]); INSERT INTO member_committee_attendance(uid, meeting_id) SELECT * FROM UNNEST($5::text[], $6::int4[]);", id, id, frosh, committee_id_vec, members, committee_id_vec)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/attendance/committee/{id}")]
pub async fn delete_committee(state: Data<Appstate>) -> impl Responder {
    let (id,) = path.into_inner();
    match query!("DELETE FROM committee_meetings WHERE id = ($1::int4); DELETE FROM freshman_committee_attendance WHERE meeting_id = ($2::int4); DELETE FROM member_committee_attendance WHERE meeting_id = ($3::int4);", id, id, id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}