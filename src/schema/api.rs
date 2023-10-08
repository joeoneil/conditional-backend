// TODO: Define API Schema that the API routes will deliver to the frontend
// These are explicitly different from the DB schema. As, for example,
// directorship attendance may be relayed to the fronted as a list of member
// names / usernames, while directorship attendance is stored in the database
// as relations in one of two tables

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::db::{AttendanceStatus, CommitteeType, CoopSemester, MajorProjectStatus};

pub struct ID {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Date {
    pub date: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct EvalsHmAtt {
    pub attendance_status: AttendanceStatus,
    pub excuse: Option<String>,
    pub date: NaiveDate,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Seminar {
    /// Name of the technical seminar
    pub name: String,
    /// Date this seminar occured
    pub timestamp: chrono::NaiveDateTime,
    /// List of member usernames who attended
    pub members: Option<Vec<String>>,
    /// List of freshmen IDs who attended
    pub frosh: Option<Vec<i32>>,
    /// Whether the seminar has been approved
    pub approved: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Directorship {
    pub committee: CommitteeType,
    pub timestamp: chrono::NaiveDateTime,
    pub members: Option<Vec<String>>,
    pub frosh: Option<Vec<i32>>,
    pub approved: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct MeetingAttendance {
    pub name: String,
    pub date: NaiveDateTime,
    pub members: Vec<String>,
    pub frosh: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct DirectorshipAttendance {
    pub committee: CommitteeType,
    pub timestamp: chrono::NaiveDateTime,
    pub approved: bool,
    pub members: Vec<String>,
    pub frosh: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct MemberHouseAttendance {
    pub name: String,
    pub att_status: AttendanceStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct FroshHouseAttendance {
    pub name: i32,
    pub att_status: AttendanceStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct HouseAttendance {
    pub date: NaiveDate,
    pub members: Vec<MemberHouseAttendance>,
    pub frosh: Vec<FroshHouseAttendance>,
}

pub struct MajorProjectSubmission {
    /// Username of member who submitted this major project
    pub uid: String,
    /// Name of this major project
    pub name: String,
    /// Description of this major project
    pub description: Option<String>,
}

pub struct MajorProjectSubmissionEboard {
    /// Username of member who submitted this major project
    pub uid: String,
    /// Name of this major project
    pub name: String,
    /// Description of this major project
    pub description: Option<String>,
    /// idk something fs
    pub status: MajorProjectStatus,
}

pub struct CoopSubmission {
    pub uid: String,
    pub date: NaiveDateTime,
    pub semester: CoopSemester,
}

pub struct IntroFormSubmission {
    pub uid: String,
    pub social_events: String,
    pub comments: String,
}
