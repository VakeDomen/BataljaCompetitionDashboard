use diesel::result::Error;
use diesel::{prelude::*, insert_into};
use crate::db::schema::teams::dsl::*;
use crate::models::user::User;
use crate::models::team::{SqlTeam, Team, NewTeam};
use super::operations_db::establish_connection;


pub fn create_team(team: NewTeam) ->  Result<Team, Error> {
    let new_team = SqlTeam::from(team);
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let _ = insert_into(teams)
        .values(&new_team)
        .execute(&mut conn)?;
    Ok(Team::from(new_team))
}

pub fn join_team(team: Team, user: User) -> Result<(), Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    diesel::update(teams.filter(id.eq(team.id.clone())))
        .set(partner.eq(user.id))
        .execute(&mut conn)?;
    Ok(())
}

pub fn get_team_by_id(uid: String) -> Result<Team, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match teams
        .filter(id.eq(uid))
        .first::<SqlTeam>(&mut conn) {
            Ok(t) => Ok(Team::from(t)),
            Err(e) => Err(e)
    }
}

pub fn get_team_by_student_for_competition(user: User, comp_id: String) -> Result<Team, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match teams
        .filter(competition_id.eq(comp_id))
        .filter(owner.eq(user.id.clone()).or(partner.eq(user.id.clone())))
        .first::<SqlTeam>(&mut conn) {
            Ok(t) => Ok(Team::from(t)),
            Err(e) => Err(e)
    }
}

pub fn leave_team(team: Team, user: User) -> Result<(), Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    diesel::update(teams.filter(
            id.eq(team.id).and(partner.eq(user.id))
        ))
        .set(partner.eq(""))
        .execute(&mut conn)?;
    Ok(())
}

pub fn kick_partner(team: Team, user: User) -> Result<(), Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    diesel::update(teams.filter(
            id.eq(team.id).and(owner.eq(user.id))
        ))
        .set(partner.eq(""))
        .execute(&mut conn)?;
    Ok(())
}


pub fn disband_team(team: Team, user: User) -> Result<(), Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    diesel::delete(teams.filter(
            id.eq(team.id).and(owner.eq(user.id))
        ))
        .execute(&mut conn)?;
    Ok(())
}

pub fn in_member_of_a_team(user: User) -> bool {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match teams
        .filter(owner.eq(user.id.clone()).or(partner.eq(user.id.clone())))
        .first::<SqlTeam>(&mut conn) {
            Ok(_) => true,
            Err(_) => false
    }
}

pub fn in_member_of_a_team_on_competition(user: User, comp_id: String) -> bool {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match teams
        .filter(competition_id.eq(comp_id))
        .filter(owner.eq(user.id.clone()).or(partner.eq(user.id.clone())))
        .first::<SqlTeam>(&mut conn) {
            Ok(_) => true,
            Err(_) => false
    }
}

