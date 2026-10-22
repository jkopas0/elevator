use std::sync::Mutex;

use tauri::Manager;

const SPEED: u32 = 1;
enum ElevatorState {
	WAITING = 0,
	UNLOADING = 1,
	LOADING = 2,
	MOVING = 3,
}

struct Passager {
	destination: u32,
}

struct BasicElevator {
	position: u32,
	step_queue: Vec<u32>,
	passagers: Vec<Passager>,
	state: ElevatorState,
}

impl Default for BasicElevator {
	fn default() -> Self {
		BasicElevator { position: 0, step_queue: vec![], passagers: vec![], state: ElevatorState::WAITING }
	}
}

struct SmartElevator {
	position: u32,
	step_queue: Vec<(u32, bool)>,
	passagers: Vec<Passager>,
	state: ElevatorState,
}

impl Default for SmartElevator {
	fn default() -> Self {
		SmartElevator { position: 0, step_queue: vec![], passagers: vec![], state: ElevatorState::WAITING }
	}
}

async fn basic_call(app: tauri::AppHandle, floor: u32) -> Result<(), String> {
	let elevator = app.state::<Mutex<BasicElevator>>();
	let mut elevator = elevator.lock().unwrap();
	elevator.step_queue.push(floor);
    Ok(())
}

async fn basic_ride(app: tauri::AppHandle, floor: u32) -> Result<(), String> {
	let elevator = app.state::<Mutex<BasicElevator>>();
	let mut elevator = elevator.lock().unwrap();
	elevator.step_queue.insert(0, floor);
	Ok(())
}

async fn smart_call(app: tauri::AppHandle, floor: u32, direction: bool) -> Result<(), String> {
	let elevator = app.state::<Mutex<SmartElevator>>();
	let mut elevator = elevator.lock().unwrap();
	match direction {
		true => {
			// Up
			elevator.step_queue.push((floor, true));
		}
		false => {
			// Down
			elevator.step_queue.push((floor, false));
		}
	}
	Ok(())
}

async fn smart_ride(app: tauri::AppHandle, floor: u32) -> Result<(), String> {
	let elevator = app.state::<Mutex<SmartElevator>>();
	let mut elevator = elevator.lock().unwrap();
	elevator.step_queue.insert(0, (floor, true));
	Ok(())
}

#[tauri::command]
async fn step(app: tauri::AppHandle) -> Result<(u32, u32), String> {
	let basic_elevator = app.state::<Mutex<BasicElevator>>();
	let mut basic_elevator = basic_elevator.lock().unwrap();
	let smart_elevator = app.state::<Mutex<BasicElevator>>();
	let mut smart_elevator = smart_elevator.lock().unwrap();

	match basic_elevator.state {
		ElevatorState::WAITING => {
			if basic_elevator.step_queue.len() > 0 {
				basic_elevator.state = ElevatorState::MOVING;
			}
		}
		ElevatorState::UNLOADING => {
			let mut i: usize = 0;
			for passager in basic_elevator.passagers {
				if passager.destination == basic_elevator.position {
					basic_elevator.passagers.remove(i);
				}
				i += 1;
			}
			basic_elevator.state = ElevatorState::LOADING;
		}
		ElevatorState::LOADING => {
			basic_elevator.state = ElevatorState::WAITING;
		}
		ElevatorState::MOVING => {
			if basic_elevator.position < basic_elevator.step_queue[0] { basic_elevator.position += SPEED; }
			else if basic_elevator.position > basic_elevator.step_queue[0] { basic_elevator.position -= SPEED; }
			else {
				basic_elevator.state = ElevatorState::UNLOADING;
				basic_elevator.step_queue.remove(0);
			}
		}
	}

	Ok((basic_elevator.position, 0))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
		.setup(|app| {
			app.manage(Mutex::new(BasicElevator::default()));
			app.manage(Mutex::new(SmartElevator::default()));
			Ok(())
		})
        .invoke_handler(tauri::generate_handler![step])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
