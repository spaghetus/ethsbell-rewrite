use std::{
	str::FromStr,
	sync::{Arc, RwLock},
};

use chrono::{Datelike, Local, NaiveDate, NaiveTime};
use rocket::{Route, State};
use rocket_contrib::json::Json;

use crate::schedule::{Period, Schedule, ScheduleType};

use super::OurError;

pub fn routes() -> Vec<Route> {
	routes![
		get_schedule,
		today,
		date,
		today_now,
		today_at,
		date_at,
		today_around_now
	]
}

#[get("/schedule")]
fn get_schedule(schedule: State<Arc<RwLock<Schedule>>>) -> Json<Schedule> {
	schedule.write().unwrap().update_if_needed();
	let schedule = schedule.read().unwrap();
	Json(schedule.clone())
}

#[get("/today")]
fn today(schedule: State<Arc<RwLock<Schedule>>>) -> Json<ScheduleType> {
	schedule.write().unwrap().update_if_needed();
	// Get the current date as a NaiveDate
	let now = Local::now();
	println!("{}", now);
	let now_date = now.date();
	let schedule = schedule.read().unwrap();
	let mut schedule = schedule.on_date(now_date.naive_local());
	schedule.periods.iter_mut().for_each(|v| v.populate(now));
	Json(schedule)
}

#[get("/today/now")]
fn today_now(schedule: State<Arc<RwLock<Schedule>>>) -> Option<Json<Period>> {
	schedule.write().unwrap().update_if_needed();
	let now = Local::now();
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.at_time(now_time)[1].clone() {
		Some(period) => {
			let mut period = period.clone();
			period.populate(now);
			Some(Json(period))
		}
		None => None,
	}
}

#[get("/today/now/near")]
fn today_around_now(schedule: State<Arc<RwLock<Schedule>>>) -> Json<[Option<Period>; 3]> {
	schedule.write().unwrap().update_if_needed();
	let now = Local::now();
	let now_date = now.date();
	let now_time = now.time();
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	let mut schedule = schedule.at_time(now_time);
	schedule.iter_mut().for_each(|v| match v {
		Some(v) => v.populate(now),
		None => {}
	});
	Json(schedule)
}

#[get("/today/at/<time_string>")]
fn today_at(
	schedule: State<Arc<RwLock<Schedule>>>,
	time_string: String,
) -> Result<Option<Json<Period>>, OurError> {
	schedule.write().unwrap().update_if_needed();
	let now = Local::now();
	let now_date = now.date();
	let then_time = NaiveTime::from_str(&time_string)?;
	let schedule = schedule.read().unwrap().on_date(now_date.naive_local());
	match schedule.at_time(then_time)[1].clone() {
		Some(period) => {
			let mut period = period.clone();
			period.populate(now);
			Ok(Some(Json(period)))
		}
		None => Ok(None),
	}
}

#[get("/on/<date_string>")]
fn date(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
) -> Result<Json<ScheduleType>, OurError> {
	schedule.write().unwrap().update_if_needed();
	let then = NaiveDate::from_str(&date_string)?;
	let then_ = Local::now()
		.with_day(then.day())
		.unwrap()
		.with_month(then.month())
		.unwrap()
		.with_year(then.year())
		.unwrap();
	let mut schedule = schedule.read().unwrap().on_date(then);
	schedule.periods.iter_mut().for_each(|v| v.populate(then_));
	Ok(Json(schedule))
}

#[get("/on/<date_string>/at/<time_string>")]
fn date_at(
	schedule: State<Arc<RwLock<Schedule>>>,
	date_string: String,
	time_string: String,
) -> Result<Option<Json<Period>>, OurError> {
	schedule.write().unwrap().update_if_needed();
	let then_date = NaiveDate::from_str(&date_string)?;
	let then_time = NaiveTime::from_str(&time_string)?;
	let then_ = Local::now()
		.with_day(then_date.day())
		.unwrap()
		.with_month(then_date.month())
		.unwrap()
		.with_year(then_date.year())
		.unwrap();
	let schedule = schedule.read().unwrap().on_date(then_date);
	match schedule.at_time(then_time)[1].clone() {
		Some(period) => {
			let mut period = period.clone();
			period.populate(then_);
			Ok(Some(Json(period)))
		}
		None => Ok(None),
	}
}
