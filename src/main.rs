use reqwest::Url;
use warp::Filter;
use warp::http::StatusCode;
use serde::{Deserialize, Serialize};
use chrono;
use urlencoding::{encode, decode};
use std::time::Duration;
use tokio::{task, time};

use std::collections::HashMap;
use fcm::{Client, MessageBuilder};
// use num_derive::FromPrimitive;	
// use num_traits::FromPrimitive;

const DEFAULT_USER_ID: u32 = 2;
const HOST: &str = "http://gtables:3000";
// const TABLE_ID: &str = "1hQup-pG0Tw89pwOK3Kb-bqX4GXB6XQT7Fvf-JfCmtkU";
const TABLE_ID: &str = "1wQcLXpsEn3kVyQdLE653m-omerCXypX0iNAhKqTTQR8"; // Юрия псевдорелиз
// const TABLE_ID: &str = "1FvKX8RkEakfbJ8xxuQdNlEunGX5297hP2xkWLbNgf9Y";

#[derive(Debug, Serialize, Deserialize)]
struct Notification {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Работник")]
	worker_id: u32,
	#[serde(rename = "Задача")]
	task_id: u32,
	#[serde(rename = "Отправлено")]
	sent: bool,
}

// Работник
#[derive(Debug, Serialize, Deserialize)]
struct Worker {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Фамилия")]
	familiya: String,
	#[serde(rename = "Имя")]
	imya: String,
	#[serde(rename = "Отчество")]
	otchestvo: Option<String>,	
	#[serde(rename = "Номер телефона")]
	phone: String,
	#[serde(rename = "FCM токен")]
	fcm_token: Option<String>,
}

// Задача
#[derive(Debug, Serialize, Deserialize)]
struct Task {
	#[serde(rename = "Номер")]
	id: u32,
	#[serde(rename = "Дата")]
	date: String,
	#[serde(rename = "Участок")]
	place_id: u32,
	#[serde(rename = "Время начала")]
	start_time: String,
	#[serde(rename = "Время конца")]
	end_time: String,
	#[serde(rename = "Описание")]
	description: String,
	#[serde(rename = "Плавающая")]
	floating: bool,
	#[serde(rename = "Удалённая")]
	deleted: bool,
}

// План_задач
#[derive(Debug, Serialize, Deserialize)]
struct Plan {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Задача")]
	task_id: u32,
	#[serde(rename = "Работник")]
	worker_id: u32,
	#[serde(rename = "Принял")]
	is_taken: bool,
	#[serde(rename = "Комментарий")]
	comment: Option<String>,
	#[serde(rename = "Время записи")]
	recordtime: String,
}

// Участок задач
#[derive(Debug, Serialize, Deserialize)]
struct Place {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Описание")]
	description: String,
	#[serde(rename = "Объект")]
	object_id: u32,
}

// Объект
#[derive(Debug, Serialize, Deserialize)]
struct Object {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Адрес")]
	address: String,
}

// Объект
#[derive(Debug, Serialize, Deserialize)]
struct Measure {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Значение")]
	value: String,
}


// Операция
#[derive(Debug, Serialize, Deserialize)]
struct Operation {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Время начала")]
	start_time: String,
	#[serde(rename = "Время конца")]
	end_time: String,
	// #[serde(rename = "Номер захватки")]
	// place: String,
	#[serde(rename = "Описание")]
	description: String,
	#[serde(rename = "Адрес")]
	additional: String,
	#[serde(rename = "Единица измерения")]
	measure_id: u32,
	#[serde(rename = "Количество")]
	count: f32,
	#[serde(rename = "Цена за единицу")]
	single_price: f32,
	#[serde(rename = "Задача")]
	task_id: u32,
	#[serde(rename = "Работник")]
	worker_id: u32,
	#[serde(rename = "Удалённая")]
	deleted: bool,
}

// Захватка-прихватка
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Grip {
	#[serde(rename = "Оси")]
	odin: String,
	#[serde(rename = "Выс")]
	dva: String,
	#[serde(rename = "Шир")]
	tri: String,
}

// Приемка
#[derive(Debug, Serialize, Deserialize)]
struct Pass {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Работник")]
	worker_id: u32,
	#[serde(rename = "Операция")]
	operation_id: u32,
	#[serde(rename = "Количество")]
	count: f32,
	#[serde(rename = "Комментарий")]
	comment: Option<String>,
	#[serde(rename = "Время старта")]
	start_time: Option<String>,
	#[serde(rename = "Время завершения")]
	end_time: String,
	#[serde(rename = "Время модификации")]
	recordtime: String,
	#[serde(rename = "Время создания")]
	createtime: String,
}

// Сдача
#[derive(Debug, Serialize, Deserialize)]
struct Accept {
	#[serde(rename = "Код")]
	id: u32,
	#[serde(rename = "Работник")]
	worker_id: u32,
	#[serde(rename = "Операция")]
	operation_id: u32,
	#[serde(rename = "Количество")]
	count: f32,
	#[serde(rename = "Комментарий")]
	comment: Option<String>,
	#[serde(rename = "Время записи")]
	recordtime: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MobileTask {
	id: u32,
	plan_id: Option<u32>,
	name: String,
	sector: String,
	address: String,
	start_time: i16,
	end_time: i16,
	status: TaskStatus,
	date: String,
	floating: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct MobileTaskExtra {
	id: u32,
	plan_id: Option<u32>,
	name: String,
	sector: String,
	address: String,
	start_time: i16,
	end_time: i16,
	status: TaskStatus,
	date: String,
	operations: Vec<MobileOperation>,
	floating: bool,
	// passes: Vec<Pass>,
	// accepts: Vec<Accept>,
}

#[repr(u8)]
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
enum TaskStatus {
	Open = 0,
	Taken = 1,
	Finished = 2,
}

impl Serialize for TaskStatus {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer {
		serializer.serialize_u8(*self as u8)
	}
}

// Операция
#[derive(Debug, Serialize, Deserialize)]
struct MobileOperation {
	id: u32,
	task_id: u32,
	// place: Grip,
	description: String,
	additional: String, // Затем поменять на place
	measure: String,
	single_price: f32,

	plan_count: f32,
	pass_count: Option<f32>,
	accept_count: Option<f32>,

	plan_salary: f32,
	pass_salary: f32,
	accept_salary: f32,

	plan_cost_per_hour: f32,
	pass_cost_per_hour: f32,
	accept_cost_per_hour: f32,

	// count: (f32, f32, f32), // План, рабочий, контроллёр, если -1, значит Сдача или Приёмки не было
	// salary: (f32, f32, f32), // План, рабочий, контроллёр, если -1, значит Сдача или Приёмки не было
	// cost_per_hour: (f32, f32, f32), // План, рабочий, контроллёр, если -1, значит Сдача или Приёмки не было

	start_time: i16,
	end_time: i16,

	pass_start_time: Option<i16>,
	pass_end_time: Option<i16>,

	status: OperationResult,

	pass_comment: Option<String>,
	accept_comment: Option<String>,
}

#[repr(u8)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
enum OperationResult {
	Open = 0,
	ReadyMade = 1,
	Accepted = 2,
	// Rejected = 3, // Убрано, вместо этого всегда будет Accepted, если есть приёмка
}

impl Serialize for OperationResult {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer {
		serializer.serialize_u8(*self as u8)
	}
}

// #[repr(u8)]
// #[derive(Debug, Deserialize, Clone, Copy)]
// enum OperationStatus {
// 	NotActive = 0,
// 	Active = 1,
// 	Overdue = 2,
// }

// impl Serialize for OperationStatus {
// 	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: serde::Serializer {
// 		serializer.serialize_u8(*self as u8)
// 	}
// }

// Работник
#[derive(Debug, Serialize, Deserialize)]
struct MobileUser {
	id: u32,
	familiya: String,
	imya: String,
	otchestvo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MobileSummary {
	salary: (f32, f32, f32),
	cost_per_hour: (f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateOperationQuery {
	user_id: u32,
	operation_id: u32,
	pass_count: f32,
	pass_start_time: Option<i16>,
	pass_end_time: i16,
	pass_comment: Option<String>,
}

async fn select_many() -> (Vec<Task>, Vec<Place>, Vec<Operation>, Vec<Object>, Vec<Plan>, Vec<Pass>, Vec<Accept>, Vec<Measure>) {
	let result: (Vec<Task>, Vec<Place>, Vec<Operation>, Vec<Object>, Vec<Plan>, Vec<Pass>, Vec<Accept>, Vec<Measure>) = reqwest::get(format!("{HOST}/select-many/{TABLE_ID}/Задача,Участок_задач,Операция,Объект,План_задач,Сдача,Приемка,Единица_измерения"))
		.await.unwrap()
		.json()
		.await.unwrap();
	(result.0.into_iter().filter(|it| !it.deleted).collect(), result.1, result.2.into_iter().filter(|it| !it.deleted).collect(), result.3, result.4, result.5, result.6, result.7)
}

async fn select_few() -> (Vec<Task>, Vec<Notification>, Vec<Worker>, Vec<Plan>) {
	let result: (Vec<Task>, Vec<Notification>, Vec<Worker>, Vec<Plan>) = reqwest::get(format!("{HOST}/select-many/{TABLE_ID}/Задача,Уведомление,Работник,План_задач"))
		.await.unwrap()
		.json()
		.await.unwrap();
	(result.0.into_iter().filter(|it| !it.deleted).collect(), result.1.into_iter().filter(|it| !it.sent).collect(), result.2, result.3)
}

macro_rules! select_all {
	($table: expr => $type: ty) => {
		{
			let result: $type = reqwest::get(format!("{HOST}/select/{TABLE_ID}/{}", $table))
				.await.unwrap()
				.json()
				.await.unwrap();
			result
		}
	};
}

fn get_minutes(time: &String) -> i16 {
	let s = time.split(":").map(|s| s.to_string()).collect::<Vec<String>>();
	let hh: i16 = s.get(0).unwrap().parse::<i16>().unwrap();
	let mm: i16 = s.get(1).unwrap().parse::<i16>().unwrap();
	return hh * 60 + mm;
}

fn get_mobile_operations(user_id: u32, task: &Task, measures: &Vec<Measure>, operations: &Vec<Operation>, passes: &Vec<Pass>, accepts: &Vec<Accept>) -> Vec<MobileOperation>  {
	let user_operations = operations.iter().filter(|o| o.worker_id == user_id && o.task_id == task.id).collect::<Vec<&Operation>>();
	let mut operations = user_operations.iter().map(|o| {
		let pass = passes.iter().find(|p| p.operation_id == o.id && p.worker_id == user_id);
		let accept = accepts.iter().find(|a| a.operation_id == o.id && a.worker_id == user_id);

		let pass_count: Option<f32> = if let Some(pass) = pass { Some(pass.count) } else { None };
		let accept_count: Option<f32> = if let Some(accept) = accept { Some(accept.count) } else { None };		

		let plan_count: f32 = o.count;

		let status = if accept.is_some() {
			OperationResult::Accepted
		} else if pass.is_some() {
			OperationResult::ReadyMade
		} else {
			OperationResult::Open
		};

		// let status = {
		// 	// Вычисляем здесь статус
		// 	OperationStatus::NotActive
		// };
		
		let measure = measures.iter().find(|it| it.id == o.measure_id).unwrap();
	
		let salary_hours: f32 = {
			let minutes1 = get_minutes(&task.start_time);
			let minutes2 = get_minutes(&task.end_time);
			(minutes2 - minutes1) as f32 / 60.0
		};
	
		let plan_salary = o.single_price * o.count;
		let pass_salary = { if let Some(pass) = pass { o.single_price * pass.count  } else { 0.0 } };
		let accept_salary = { if let Some(accept) = accept { o.single_price * accept.count } else { 0.0 } };

		let (plan_cost_per_hour, pass_cost_per_hour, accept_cost_per_hour) = if salary_hours != 0.0 { (plan_salary / salary_hours, pass_salary / salary_hours, accept_salary / salary_hours) } else { (0.0, 0.0, 0.0) };
		let (pass_start_time, pass_end_time) = if let Some(pass) = pass { (pass.start_time.clone(), Some(pass.end_time.clone())) } else { (None, None) };

		MobileOperation {
			id: o.id,
			task_id: task.id,
			// place: serde_json::from_str(&o.place.clone()).unwrap(),
			description: o.description.clone(),
			measure: measure.value.clone(), // Сделать выборку из таблицы Measure
			single_price: o.single_price,

			// План, рабочий, контроллёр
			plan_count,
			pass_count: pass_count,
			accept_count: accept_count,

			// Теперь план, рабочий, контроллёр
			plan_salary: plan_salary,
			pass_salary: pass_salary,
			accept_salary: accept_salary,

			plan_cost_per_hour: plan_cost_per_hour,
			pass_cost_per_hour: pass_cost_per_hour,
			accept_cost_per_hour: accept_cost_per_hour,
			
			start_time: get_minutes(&o.start_time),
			end_time: get_minutes(&o.end_time),
			
			pass_start_time: if let Some(pass_start_time) = pass_start_time { Some(get_minutes(&pass_start_time)) } else { None },
			pass_end_time: if let Some(pass_end_time) = pass_end_time { Some(get_minutes(&pass_end_time)) } else { None },

			status,

			additional: o.additional.clone(),
			pass_comment: { if let Some(pass) = pass { pass.comment.clone() } else { None } },
			accept_comment: { if let Some(accept) = accept { accept.comment.clone() } else { None } },
		}
	}).collect::<Vec<MobileOperation>>();
	operations.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
	operations
}

// Этот метод извлекает только задачи суточные!
async fn get_tasks(mut user_id: u32, filter_id: u32) ->  Result<Box<dyn warp::Reply>, warp::Rejection> {
	println!("get_tasks user_id={user_id} filter_id={filter_id}");
	user_id = DEFAULT_USER_ID;

	let (tasks, places, operations, objects, plans, passes, accepts, measures) = select_many().await;
	
	let mobile_tasks = tasks.iter().filter(|t| {
		let operations = get_mobile_operations(user_id, t, &measures, &operations, &passes, &accepts);
		let plan = plans.iter().find(|p| p.task_id == t.id && p.worker_id == user_id);

		if operations.len() == 0 {
			return false;
		}

		if let Some(plan) = plan {
			// Если есть план
			if !plan.is_taken {
				// План был отклонён
				return false;
			}
		}

		if operations.iter().all(|o| {
			o.status == OperationResult::Accepted || (o.status == OperationResult::ReadyMade && o.pass_count.expect("Если у операции статус ReadyMade, то pass_count быть обязан.") == 0.0)
		}) {
			// if filter_id == 2 { return true; } else { return false; }
			return true;
		} else {
			// if filter_id == 2 { return false; } else { return true; }
			return true;
		}

		return true;
	}).map(|t| {
		let operations = get_mobile_operations(user_id, t, &measures, &operations, &passes, &accepts);
		let plan = plans.iter().find(|p| p.task_id == t.id && p.worker_id == user_id);
		let place = places.iter().find(|p| p.id == t.place_id).unwrap();
		let object = objects.iter().find(|o| o.id == place.object_id).unwrap();
		// places.iter().find(|p| p.id == t.place_id).unwrap().description.clone()
		MobileTaskExtra {
			id: t.id,
			plan_id: if let Some(plan) = plan { Some(plan.id) } else { None }, // Из Plan
			name: t.description.clone(),
			sector: place.description.clone(),
			address: object.address.clone(),
			start_time: get_minutes(&t.start_time),// { let mut a = t.start_time.clone(); a.replace_range(5.., ""); a },
			end_time: get_minutes(&t.end_time),// { let mut b = t.end_time.clone(); b.replace_range(5.., ""); b },
			status: {
				if !plan.is_some() || operations.len() == 0 {
					TaskStatus::Open
				} else if plan.is_some() && !operations.iter().all(|o| o.status == OperationResult::Accepted) {
					TaskStatus::Taken
				} else {
					TaskStatus::Finished
				}
			},
			// is_taken: if let Some(plan) = plan { plan.is_taken } else { false }, // Из Plan
			date: t.date.clone(),
			operations,
			floating: t.floating,
		}
	}).collect::<Vec<MobileTaskExtra>>();

	Ok(Box::new(serde_json::to_string(&mobile_tasks).unwrap()))
}

async fn get_operations(mut user_id: u32, plan_id: u32) ->  Result<Box<dyn warp::Reply>, warp::Rejection> {
	println!("get_operations user_id={user_id} plan_id={plan_id}");
	user_id = DEFAULT_USER_ID;

	let (tasks, places, operations, objects, plans, passes, accepts, measures) = select_many().await;

	let plan = plans.iter().find(|p| p.id == plan_id);
	
	if let Some(plan) = plan {
		if !plan.is_taken {
			Ok(Box::new(warp::reply::with_status(
				"План найден, но был отклонён",
				StatusCode::NOT_FOUND,
			)))
		} else {
			let task = tasks.iter().find(|t| t.id == plan.task_id).unwrap();
			let operations = get_mobile_operations(user_id, task, &measures, &operations, &passes, &accepts);
			Ok(Box::new(serde_json::to_string(&operations).unwrap()))
		}
	} else {
		Ok(Box::new(warp::reply::with_status(
			"Не найден план",
			StatusCode::NOT_FOUND,
		)))
	}
}

async fn auth(phone: String) ->  Result<Box<dyn warp::Reply>, warp::Rejection> {
	let workers = select_all!("Работник" => Vec<Worker>);
	let worker = workers.into_iter().find(|w| w.phone == phone).unwrap();
	let mobile_worker = MobileUser {
		id: worker.id,
		familiya: worker.familiya.clone(),
		imya: worker.imya.clone(),
		otchestvo: worker.otchestvo.clone(),
	};
	Ok(Box::new(serde_json::to_string(&mobile_worker).unwrap()))
}

async fn register_fcm(worker_id: u32, fcm_token: String) ->  Result<Box<dyn warp::Reply>, warp::Rejection> {
	println!("register_fcm worker_id={worker_id} fcm_token={fcm_token}");
	let workers = select_all!("Работник" => Vec<Worker>);
	let worker = workers.into_iter().find(|w| w.id == worker_id).unwrap();
	let client = reqwest::Client::new();
	let _ = client.post(format!("{HOST}/update-one/{TABLE_ID}/Работник/Код/{}/FCM токен/{}", worker.id, fcm_token))
		.send()
		.await.unwrap();
	Ok(Box::new(serde_json::to_string(&true).unwrap()))
}

async fn get_money_summary(mut user_id: u32, tasks_type: u32) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
	user_id = DEFAULT_USER_ID;

	let (tasks, places, operations, objects, plans, passes, accepts, measures) = select_many().await;
	
	let mobile_tasks = tasks.iter().filter(|t| {
		let operations = get_mobile_operations(user_id, t, &measures, &operations, &passes, &accepts);
		let plan = plans.iter().find(|p| p.task_id == t.id && p.worker_id == user_id);

		if let Some(plan) = plan {
			// Если есть план
			if !plan.is_taken {
				// План был отклонён
				return false;
			}
		}

		if operations.len() == 0 {
			return false;
		}
		
		return true;
	}).map(|t| {
		let operations = get_mobile_operations(user_id, t, &measures, &operations, &passes, &accepts);
		let plan = plans.iter().find(|p| p.task_id == t.id && p.worker_id == user_id);
		let place = places.iter().find(|p| p.id == t.place_id).unwrap();
		let object = objects.iter().find(|o| o.id == place.object_id).unwrap();
		// places.iter().find(|p| p.id == t.place_id).unwrap().description.clone()
		(MobileTask {
			id: t.id,
			plan_id: if let Some(plan) = plan { Some(plan.id) } else { None }, // Из Plan
			name: t.description.clone(),
			sector: place.description.clone(),
			address: object.address.clone(),
			start_time: get_minutes(&t.start_time),// { let mut a = t.start_time.clone(); a.replace_range(5.., ""); a },
			end_time: get_minutes(&t.end_time),// { let mut b = t.end_time.clone(); b.replace_range(5.., ""); b },
			status: {
				if !plan.is_some() || operations.len() == 0 {
					TaskStatus::Open
				} else if plan.is_some() && !operations.iter().all(|o| o.status == OperationResult::Accepted) {
					TaskStatus::Taken
				} else {
					TaskStatus::Finished
				}
			},
			// is_taken: if let Some(plan) = plan { plan.is_taken } else { false }, // Из Plan
			date: t.date.clone(),
			floating: t.floating,
		}, operations)
	}).collect::<Vec<(MobileTask, Vec<MobileOperation>)>>();

	let taken_tasks = mobile_tasks
		.iter()
		.filter(|t| {
			if tasks_type == 0 {
				t.0.status == TaskStatus::Taken
			} else {
				t.0.status == TaskStatus::Finished
			}
		})
		.collect::<Vec<&(MobileTask, Vec<MobileOperation>)>>();

	let salary_plan: f32 = taken_tasks.iter().map(|t| t.1.iter().map(|o| o.single_price * o.plan_count).sum::<f32>()).sum();
	let salary_pass: f32 = taken_tasks.iter().map(|t| t.1.iter().map(|o| o.single_price * if let Some(count) = o.pass_count { count } else { 0.0 }).sum::<f32>()).sum();
	let salary_accept: f32 = taken_tasks.iter().map(|t| t.1.iter().map(|o| o.single_price * if let Some(count) = o.accept_count { count } else { 0.0 }).sum::<f32>()).sum();

	let salary_hours: f32 = taken_tasks.iter().map(|t| {
		let minutes1 = t.0.start_time;
		let minutes2 = t.0.end_time;
		// let minutes1 = get_minutes(&t.1.get(0).unwrap().start_time);
		// let minutes2 = get_minutes(&t.1.get(t.1.len() - 1).unwrap().end_time);
		return (minutes2 - minutes1) as f32 / 60.0;
	}).sum();

	println!("{salary_hours}");
	// taken_tasks.iter().flat_map(|t| t.op)

	let ms = if salary_hours != 0.0 {
		MobileSummary {
			salary: (salary_plan, salary_pass, salary_accept),
			cost_per_hour: (salary_plan / salary_hours, salary_pass / salary_hours, salary_accept / salary_hours),
		}
	} else {
		MobileSummary {
			salary: (salary_plan, salary_pass, salary_accept),
			cost_per_hour: (0.0, 0.0, 0.0),
		}
	};
	Ok(Box::new(serde_json::to_string(&ms).unwrap()))

	/*println!("money_summary user_id={user_id}");
	
	let tasks = select_all!("Задача" => Vec<Task>);
	let places = select_all!("Участок_задач" => Vec<Place>);
	let operations = select_all!("Операция" => Vec<Operation>);
	let objects = select_all!("Объект" => Vec<Object>);
	let plans = select_all!("План_задач" => Vec<Plan>);
	
	let tasks = tasks.iter().filter(|t| t.date == "29.07.2022").map(|t| {
		let plan = plans.iter().find(|p| p.worker_id == user_id && p.task_id == t.id);
		if plan.is_some() {
			return true;
		} else {
			return false;
		}
	});

	let operations = get_mobile_operations(user_id, task, &operations, &passes, &accepts)
		.iter()
		.filter(|o| {
			let task = tasks.iter().find(|t| t.id == o.task_id).unwrap();
			task.date == "29.07.2022"
		});*/
}

async fn action_apply_task(mut user_id: u32, task_id: u32) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
	println!("action_apply_task user_id={user_id} task_id={task_id}");
	let plans = select_all!("План_задач" => Vec<Plan>);

	if plans.iter().find(|it| it.worker_id == user_id && it.id == task_id && it.is_taken).is_some() {
		println!("task already taken");
		return Ok(Box::new(serde_json::to_string(&false).unwrap()));
	}

	user_id = DEFAULT_USER_ID;
	let client = reqwest::Client::new();
	let timestamp = chrono::offset::Local::now();
	let recordtime = timestamp.format("%d.%m.%Y %H:%M:%S").to_string();

	let _new_plan_id: u32 = client.post(format!("{HOST}/insert-next/{TABLE_ID}/План_задач"))
		.json(&Plan {
			id: 0,
			task_id,
			worker_id: user_id,
			is_taken: true,
			comment: None,
			recordtime,
		})
		.send()
		.await.unwrap().text().await.unwrap().parse().unwrap();

	// let (tasks, places, operations, objects, plans, passes, accepts, measures) = select_many().await;

	// let task = tasks.iter().find(|t| t.id == task_id).unwrap();
	// let operations = get_mobile_operations(user_id, task, &measures, &operations, &passes, &accepts);
	Ok(Box::new(serde_json::to_string(&true).unwrap()))
}

async fn action_decline_task(mut user_id: u32, task_id: u32) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
	println!("action_decline_task user_id={user_id} task_id={task_id}");

	let plans = select_all!("План_задач" => Vec<Plan>);

	if plans.iter().find(|it| it.worker_id == user_id && it.id == task_id && !it.is_taken).is_some() {
		println!("task already declined");
		return Ok(Box::new(serde_json::to_string(&false).unwrap()));
	}

	user_id = DEFAULT_USER_ID;
	let client = reqwest::Client::new();
	let timestamp = chrono::offset::Local::now();
	let recordtime = timestamp.format("%d.%m.%Y %H:%M:%S").to_string();

	let _ = client.post(format!("{HOST}/insert-next/{TABLE_ID}/План_задач"))
		.json(&Plan {
			id: 0,
			task_id,
			worker_id: user_id,
			is_taken: false,
			comment: None,
			recordtime,
		})
		.send()
		.await.unwrap();

	Ok(Box::new(serde_json::to_string(&true).unwrap()))
}

fn minutes_to_str(i: i16) -> String {
	let hours = i / 60;
	let minutes = i - hours * 60;
	return format!("{hours}:{minutes}:00");
}

async fn action_apply_operation(query: UpdateOperationQuery) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
	let UpdateOperationQuery {
		mut user_id,
		operation_id,
		pass_count,
		pass_start_time,
		pass_end_time,
		pass_comment
	} = query;
	println!("action_apply_operation user_id={user_id} operation_id={operation_id}");
	user_id = DEFAULT_USER_ID;
	let passes = select_all!("Сдача" => Vec<Pass>);

	let user_pass = passes.iter().find(|it| it.operation_id == operation_id && it.worker_id == user_id);
	
	if let Some(user_pass) = user_pass  {
		// Если сдача есть (и нет приёмки), то обновляем всё
		let accepts = select_all!("Приемка" => Vec<Accept>);
		let accept = accepts.iter().find(|it| it.operation_id == operation_id && it.worker_id == user_id);
		if accept.is_some() {
			// Есть приём, обновлять данные нельзя!
			return Ok(Box::new(serde_json::to_string(&true).unwrap()));
		}
		
		let pass_id = user_pass.id;
		let client = reqwest::Client::new();

		let _ = client.post(format!("{HOST}/update-one/{TABLE_ID}/Сдача/Код/{pass_id}/Количество/{pass_count}"))
			.send()
			.await.unwrap();
		
		if let Some(pass_comment) = pass_comment {
			let _ = client.post(format!("{HOST}/update-one/{TABLE_ID}/Сдача/Код/{pass_id}/Комментарий/'{pass_comment}"))
				.send()
				.await.unwrap();
		}
		
		let minutable_pass_end_time = minutes_to_str(pass_end_time);
		let _ = client.post(format!("{HOST}/update-one/{TABLE_ID}/Сдача/Код/{pass_id}/Время завершения/{minutable_pass_end_time}"))
			.send()
			.await.unwrap();

		if let Some(pass_start_time) = pass_start_time {
			let minutable_pass_start_time = minutes_to_str(pass_start_time);
			let _ = client.post(format!("{HOST}/update-one/{TABLE_ID}/Сдача/Код/{pass_id}/Время старта/{minutable_pass_start_time}"))
				.send()
				.await.unwrap();
		}
		
		let timestamp = chrono::offset::Local::now();
		let recordtime = timestamp.format("%d.%m.%Y %H:%M:%S").to_string();
		let _ = client.post(format!("{HOST}/update-one/{TABLE_ID}/Сдача/Код/{pass_id}/Время модификации/{recordtime}"))
			.send()
			.await.unwrap();

		return Ok(Box::new(serde_json::to_string(&true).unwrap()));
	} else {
		// Если сдачи нет, то добавляем
		let client = reqwest::Client::new();
		let timestamp = chrono::offset::Local::now();

		let _ = client.post(format!("{HOST}/insert-next/{TABLE_ID}/Сдача"))
			.json(&Pass {
				id: 0,
				worker_id: user_id,
				operation_id,
				count: pass_count,
				end_time: minutes_to_str(pass_end_time),
				start_time: if let Some(pass_start_time) = pass_start_time { Some(minutes_to_str(pass_start_time)) } else { None },
				comment: if let Some(pass_comment) = pass_comment { Some(format!("'{pass_comment}'")) } else { None },
				recordtime: timestamp.format("%d.%m.%Y %H:%M:%S").to_string(),
				createtime: timestamp.format("%d.%m.%Y %H:%M:%S").to_string(),
			})
			.send()
			.await.unwrap();
	};
	
	// TODO: Сделать проверку, если есть приёмка, то обновить уже ничего нельзя!
	Ok(Box::new(serde_json::to_string(&true).unwrap()))
}

async fn reset() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
	let _ = reqwest::get(format!("{HOST}/reset-new/{TABLE_ID}"))
		.await.unwrap();
	Ok(Box::new(serde_json::to_string(&true).unwrap()))
}

async fn sendToDevice(notification: &Notification, task: &Task, worker: &Worker) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // pretty_env_logger::init();
	// Уведомляем только в случае, если у работника есть FCM токен
	if let Some(fcm_token) = &worker.fcm_token {
		print!("Обновляем уведомление {}, работник {}, задача {} в таблицах: ", notification.id, notification.worker_id, notification.task_id);
		let client = reqwest::Client::new();
		let _ = client.post(format!("{HOST}/update-one/{TABLE_ID}/Уведомление/Код/{}/Отправлено/ИСТИНА", notification.id))
			.send()
			.await.unwrap();
		println!("готово");

		let api_key = "AAAAakYZ-o0:APA91bHqVhvIryONnVnZVllgsWth30gvbm6n9M1MlW_FdXCdH6LGKVrNZDJ6wZ5Cu4MS-i82-SKwDtJqYnwRcc1yBjZMPRWcaIn6VWzpSSHKa2Bnl34pVlh_v9jgRGJdnfTv2x8gEWdy".to_string();

		let mut builder = fcm::NotificationBuilder::new();
		builder.title("Новая задача");
		builder.body(&task.description);
		let notification = builder.finalize();

		let mut message_builder = MessageBuilder::new(&api_key, &fcm_token);
		message_builder.notification(notification);

		let mut map = HashMap::new();
		map.insert("task", serde_json::to_string(task).unwrap());
		message_builder.data(&map);

		let fcm_client = Client::new();
		print!("Отправляем FCM уведомление: ");
		let response = fcm_client.send(message_builder.finalize()).await?;
		println!("готово");
		println!("FCM ответ: {:?}", response);

		Ok(())
	} else {
		eprintln!("Для уведомления {}, работник {}, задача {} в таблицах у работника нет FCM токена, отправить невозможно", notification.id, notification.worker_id, notification.task_id);
		Ok(())
	}
}

async fn checkAndSendNewTasks() {
	let (tasks, notifications, workers, plans) = select_few().await;
	let filtered_notifications = notifications.into_iter().filter(|notification| tasks.iter().any(|task| task.id == notification.task_id)).collect::<Vec<Notification>>();
	println!("{} {} {} {}", tasks.len(), filtered_notifications.len(), workers.len(), plans.len());
	for notification in filtered_notifications.into_iter() {
		let task = tasks.iter().find(|task| task.id == notification.task_id).unwrap();
		let worker = workers.iter().find(|worker| worker.id == notification.worker_id).unwrap();
		sendToDevice(&notification, task, worker).await;
	}
}

#[tokio::main]
async fn main() {
	let cors = warp::cors()
		.allow_any_origin();
	// GET /hello/warp => 200 OK with body "Hello, warp!"
	let auth = warp::path!("auth" / String)
		.and_then(auth);
	let auth = warp::path!("register-fcm" / u32 / String)
		.and_then(register_fcm);
	let tasks = warp::path!("tasks" / u32 / u32)
		.and_then(get_tasks);
	let operations = warp::path!("operations" / u32 / u32)
		.and_then(get_operations);
	let money_summary = warp::path!("money-summary" / u32 / u32)
		.and_then(get_money_summary);

	let apply_task = warp::path!("apply-task" / u32 / u32)
		.and_then(action_apply_task);
	let decline_task = warp::path!("decline-task" / u32 / u32)
		.and_then(action_decline_task);
	let reset = warp::path!("reset")
		.and_then(reset);

	let apply_operation = warp::path!("apply-operation")
		.and(warp::path::end())
		.and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
		.and_then(action_apply_operation);
	
	let getRoutes = warp::get().and(
		auth
			.or(tasks)
			.or(operations)
			.or(money_summary)
			.or(apply_task)
			.or(decline_task)
			.or(apply_operation)
			.or(reset)
	).with(&cors);

	let postRoutes = warp::post().and(
		apply_operation
	).with(&cors);

	let routes = getRoutes.or(postRoutes);
	
	println!("Server started5!");

	let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_millis(45000));

        loop {
            interval.tick().await;
			println!("Я тик!092");
			checkAndSendNewTasks().await;
        }
    });
	
	let webserver = warp::serve(routes)
		.run(([0, 0, 0, 0], 4444));
	
	tokio::select! {
		_ = webserver => {}
		_ = forever => {}
	}
} 
