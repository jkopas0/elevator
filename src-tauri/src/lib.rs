use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use tauri::{Emitter, Manager};

const SPEED: u32 = 50;
static TIME: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Default)]
enum ElevatorState {
	#[default]
	WAITING = 0,
	UNLOADING = 1,
	LOADING = 2,
	MOVING = 3,
}

#[derive(serde::Deserialize, Clone)]
struct Passager {
	#[serde(rename = "from")]
	origin: u32,
	#[serde(rename = "to")]
	destination: u32,
	#[serde(rename = "t", deserialize_with = "time_to_ms")]
	time: u32,
}

#[derive(serde::Serialize, Clone)]
struct StepInfo {
	position: u32,
	floors: Vec<u32>,
	inside: u32,
	distance_travelled: u32,
}

#[derive(serde::Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct Elevator {
	start_floor: u32,
	floors: u32,
	capacity: u32,
	#[serde(skip)]
	position: u32,
	#[serde(skip)]
	passagers: Vec<Passager>,
	#[serde(skip)]
	state: ElevatorState,
	#[serde(skip)]
	queue: Vec<Passager>,
	#[serde(skip)]
	waiting: Vec<Passager>,
	#[serde(skip)]
	distance_travelled: u32,
}

impl Elevator {
	fn spawn(&mut self, time: u32) -> Vec<Passager> {
		let due = self.queue.iter().take_while(|p| p.time <= time).count();
		let spawned: Vec<Passager> = self.queue.drain(..due).collect();
		self.waiting.extend(spawned.iter().cloned());
		spawned
	}

	fn info(&self) -> StepInfo {
		let mut floors = vec![0; self.floors as usize];
		for passager in &self.waiting {
			if let Some(count) = floors.get_mut(passager.origin as usize) {
				*count += 1;
			}
		}
		StepInfo { position: self.position, floors, inside: self.passagers.len() as u32, distance_travelled: self.distance_travelled }
	}
}

struct BasicElevator {
	elevator: Elevator,
	step_queue: Vec<u32>,
}

impl Default for BasicElevator {
	fn default() -> Self {
		BasicElevator { elevator: Elevator::default(), step_queue: vec![] }
	}
}

struct SmartElevator {
	elevator: Elevator,
	step_queue: Vec<(u32, bool)>,
}

impl Default for SmartElevator {
	fn default() -> Self {
		SmartElevator { elevator: Elevator::default(), step_queue: vec![] }
	}
}

#[derive(serde::Deserialize)]
struct Data {
	#[serde(rename = "lift")]
	elevator: Elevator,
	people: Vec<Passager>,
}

fn time_to_ms<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
	let t: f64 = serde::Deserialize::deserialize(deserializer)?;
	Ok((t * SPEED as f64).ceil() as u32)
}

fn basic_call(elevator: &mut BasicElevator, floor: u32) {
	if !elevator.step_queue.contains(&floor) {
		elevator.step_queue.push(floor);
	}
}

fn basic_ride(elevator: &mut BasicElevator, floor: u32) {
	elevator.step_queue.retain(|&f| f != floor);
	elevator.step_queue.insert(0, floor);
}

fn smart_call(elevator: &mut SmartElevator, floor: u32, direction: bool) {
	if elevator.step_queue.contains(&(floor, direction)) {
		return;
	}
	elevator.step_queue.push((floor, direction));

	let position = elevator.elevator.position;
	let (target, target_direction) = elevator.step_queue[0];
	let up = if target != position { target > position } else { target_direction };

	elevator.step_queue.sort_by_key(|&(floor, direction)| {
		let ahead = if up { floor >= position } else { floor <= position };
		let sweep = if direction != up { 1 } else if ahead { 0 } else { 2 };
		let order = if direction { floor as i64 } else { -(floor as i64) };
		(sweep, order)
	});
}

fn step(app: &tauri::AppHandle) -> (StepInfo, StepInfo) {
	let basic_elevator = app.state::<Mutex<BasicElevator>>();
	let mut basic_elevator = basic_elevator.lock().unwrap();
	let smart_elevator = app.state::<Mutex<SmartElevator>>();
	let mut smart_elevator = smart_elevator.lock().unwrap();

	let time = TIME.fetch_add(SPEED, Ordering::Relaxed) + SPEED;
	for passager in basic_elevator.elevator.spawn(time) {
		basic_call(&mut basic_elevator, passager.origin);
	}
	for passager in smart_elevator.elevator.spawn(time) {
		smart_call(&mut smart_elevator, passager.origin, passager.destination > passager.origin);
	}

	match basic_elevator.elevator.state {
		ElevatorState::WAITING => {
			if basic_elevator.step_queue.len() > 0 {
				basic_elevator.elevator.state = ElevatorState::MOVING;
			}
		}
		ElevatorState::UNLOADING => {
			let position = basic_elevator.elevator.position;
			if let Some(i) = basic_elevator.elevator.passagers.iter().position(|p| p.destination == position) {
				basic_elevator.elevator.passagers.remove(i);
			}
			if !basic_elevator.elevator.passagers.iter().any(|p| p.destination == position) {
				basic_elevator.elevator.state = ElevatorState::LOADING;
			}
		}
		ElevatorState::LOADING => {
			let position = basic_elevator.elevator.position;
			let capacity = basic_elevator.elevator.capacity as usize;
			if basic_elevator.elevator.passagers.len() < capacity {
				if let Some(i) = basic_elevator.elevator.waiting.iter().position(|p| p.origin == position) {
					let passager = basic_elevator.elevator.waiting.remove(i);
					basic_ride(&mut basic_elevator, passager.destination);
					basic_elevator.elevator.passagers.push(passager);
				}
			}
			if !basic_elevator.elevator.waiting.iter().any(|p| p.origin == position) {
				basic_elevator.elevator.state = ElevatorState::WAITING;
			} else if basic_elevator.elevator.passagers.len() >= capacity {
				basic_call(&mut basic_elevator, position);
				basic_elevator.elevator.state = ElevatorState::WAITING;
			}
		}
		ElevatorState::MOVING => {
			if basic_elevator.elevator.position < basic_elevator.step_queue[0] { basic_elevator.elevator.position += 1; basic_elevator.elevator.distance_travelled += 1; }
			else if basic_elevator.elevator.position > basic_elevator.step_queue[0] { basic_elevator.elevator.position -= 1; basic_elevator.elevator.distance_travelled += 1; }
			else {
				basic_elevator.elevator.state = ElevatorState::UNLOADING;
				basic_elevator.step_queue.remove(0);
			}
		}
	}

	match smart_elevator.elevator.state {
		ElevatorState::WAITING => {
			if smart_elevator.step_queue.len() > 0 {
				smart_elevator.elevator.state = ElevatorState::MOVING;
			}
		}
		ElevatorState::UNLOADING => {
			let position = smart_elevator.elevator.position;
			if let Some(i) = smart_elevator.elevator.passagers.iter().position(|p| p.destination == position) {
				smart_elevator.elevator.passagers.remove(i);
			}
			if !smart_elevator.elevator.passagers.iter().any(|p| p.destination == position) {
				smart_elevator.elevator.state = ElevatorState::LOADING;
			}
		}
		ElevatorState::LOADING => {
			let position = smart_elevator.elevator.position;
			let capacity = smart_elevator.elevator.capacity as usize;
			if smart_elevator.elevator.passagers.len() < capacity {
				if let Some(i) = smart_elevator.elevator.waiting.iter().position(|p| p.origin == position) {
					let passager = smart_elevator.elevator.waiting.remove(i);
					smart_call(&mut smart_elevator, passager.destination, passager.destination > passager.origin);
					smart_elevator.elevator.passagers.push(passager);
				}
			}
			if !smart_elevator.elevator.waiting.iter().any(|p| p.origin == position)
				|| smart_elevator.elevator.passagers.len() >= capacity {
				smart_elevator.elevator.state = ElevatorState::WAITING;
			}
		}
		ElevatorState::MOVING => {
			let position = smart_elevator.elevator.position;
			let full = smart_elevator.elevator.passagers.len() >= smart_elevator.elevator.capacity as usize;
			let unloading = smart_elevator.elevator.passagers.iter().any(|p| p.destination == position);
			if full && !unloading {
				smart_elevator.step_queue.retain(|&(floor, _)| floor != position);
			}
			match smart_elevator.step_queue.first() {
				None => smart_elevator.elevator.state = ElevatorState::WAITING,
				Some(&(target, _)) => {
					if position < target { smart_elevator.elevator.position += 1; smart_elevator.elevator.distance_travelled += 1; }
					else if position > target { smart_elevator.elevator.position -= 1; smart_elevator.elevator.distance_travelled += 1; }
					else {
						smart_elevator.elevator.state = ElevatorState::UNLOADING;
						smart_elevator.step_queue.remove(0);
					}
				}
			}
			if smart_elevator.elevator.position != position {
				let left: Vec<(u32, bool)> = smart_elevator.elevator.waiting.iter()
					.filter(|p| p.origin == position)
					.map(|p| (p.origin, p.destination > p.origin))
					.collect();
				for (floor, direction) in left {
					smart_call(&mut smart_elevator, floor, direction);
				}
			}
		}
	}

	(basic_elevator.elevator.info(), smart_elevator.elevator.info())
}

fn step_loop(app: tauri::AppHandle) {
	let interval = Duration::from_millis(SPEED as u64);
	let mut next = Instant::now() + interval;
	loop {
		std::thread::sleep(next.saturating_duration_since(Instant::now()));
		next += interval;
		let info = step(&app);
		let _ = app.emit("step", info);
	}
}

#[tauri::command]
async fn load(app: tauri::AppHandle, path: String) -> Result<(), String> {
	let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
	let mut data: Data = serde_json::from_str(&json).map_err(|e| e.to_string())?;
	data.people.sort_by_key(|p| p.time);
	data.elevator.position = data.elevator.start_floor;
	data.elevator.queue = data.people;

	let basic_elevator = app.state::<Mutex<BasicElevator>>();
	*basic_elevator.lock().unwrap() = BasicElevator { elevator: data.elevator.clone(), ..Default::default() };
	let smart_elevator = app.state::<Mutex<SmartElevator>>();
	*smart_elevator.lock().unwrap() = SmartElevator { elevator: data.elevator, ..Default::default() };
	TIME.store(0, Ordering::Relaxed);

	Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
		.setup(|app| {
			app.manage(Mutex::new(BasicElevator::default()));
			app.manage(Mutex::new(SmartElevator::default()));
			let handle = app.handle().clone();
			std::thread::spawn(move || step_loop(handle));
			Ok(())
		})
        .invoke_handler(tauri::generate_handler![load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
