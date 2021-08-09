//! just some unrelated testing stuff for an experimental protocol, nothing to see here

use crate::schedule::Schedule;
use chrono::Local;
use hcp_common::{Content, Header, Request, Response};
use rocket::{Route, State};
use rocket_contrib::json::Json;
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

/// Generates a list of Routes for Rocket
pub fn routes() -> Vec<Route> {
	routes![head_root, get_root, head_live, get_live]
}

#[get("/")]
fn head_root() -> Json<Header> {
	Json(Default::default())
}

#[post("/", data = "<_data>")]
fn get_root(_data: Json<Request>) -> Json<Response> {
	Json(serde_json::from_str(include_str!("./home.hcf")).unwrap())
}

#[get("/live")]
fn head_live() -> Json<Header> {
	Json(Default::default())
}

#[post("/live", data = "<_data>")]
fn get_live(_data: Json<Request>, schedule: State<Arc<RwLock<Schedule>>>) -> Json<Response> {
	Schedule::update_if_needed_async(schedule.clone());
	let now = Local::now();
	let period = schedule
		.read()
		.unwrap()
		.on_date(now.date().naive_local())
		.0
		.at_time(now.time())
		.1;
	let mut doc: Response = serde_json::from_str(include_str!("./live.hcf")).unwrap();
	let mut context = HashMap::new();
	let mut flags = vec![];
	match period.iter().next() {
		Some(p) => {
			flags.push("period".to_string());
			context.insert(
				"period_name".to_string(),
				Content::Text(p.friendly_name.clone()),
			);
		}
		None => {}
	}
	doc.content = vec![Content::VLayout(
		doc.content
			.iter()
			.map(|v| v.clone().template(&context, &flags))
			.collect(),
	)];
	Json(doc)
}
