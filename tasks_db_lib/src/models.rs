#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable, Selectable, Debug, serde::Serialize)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

#[derive(Queryable, Debug, Selectable,Identifiable, serde::Serialize)]
#[diesel(primary_key(task_status_id))]
#[diesel(table_name = task_statuses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TaskStatus {
    pub task_status_id: i32,
    pub status_name: String,
}

#[derive(Queryable, Debug, Selectable,Identifiable, serde::Serialize)]
#[diesel(primary_key(task_id))]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub task_id: i32,
    pub task_name: String,
}

#[derive(Queryable, Debug, Selectable,Identifiable, serde::Serialize)]
#[diesel(primary_key(user_id, task_id))]
#[diesel(table_name = user_tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserTask {
    pub user_id: i32,
    pub task_id: i32,
    pub task_status_id: i32,
}


#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub task_name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = task_statuses)]
pub struct NewTaskStatus<'a> {
    pub status_name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = user_tasks)]
pub struct NewUserTask {
    pub user_id: i32,
    pub task_id: i32,
    pub task_status_id: i32
}


