use std::sync::Mutex;

use tauri::Manager;

enum ElevatorState {
	WAITING = 0,
	UNLOADING = 1,
	LOADING = 2,
	MOVING = 3,
}

struct BasicElevator {
	step_queue: Vec<u32>,
	state: ElevatorState,
}

impl Default for BasicElevator {
	fn default() -> Self {
		BasicElevator { step_queue: vec![], state: ElevatorState::WAITING }
	}
}

struct SmartElevator {
	step_queue: Vec<(u32, bool)>,
	state: ElevatorState,
}

impl Default for SmartElevator {
	fn default() -> Self {
		SmartElevator { step_queue: vec![], state: ElevatorState::WAITING }
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

	let basic_y_pos: u32;

	match basic_elevator.state {
		ElevatorState::WAITING => {
			if basic_elevator.step_queue.len() > 0 {
				basic_elevator.state = ElevatorState::MOVING;
			}
			basic_y_pos = basic_elevator.step_queue[0];
		}
		ElevatorState::UNLOADING => {
			basic_elevator.state = ElevatorState::LOADING;
			basic_y_pos = basic_elevator.step_queue[0];
		}
		ElevatorState::LOADING => {
			basic_elevator.state = ElevatorState::WAITING;
			basic_y_pos = basic_elevator.step_queue[0];
		}
		ElevatorState::MOVING => {
			basic_elevator.state = ElevatorState::UNLOADING;
			basic_elevator.step_queue.remove(0);
			basic_y_pos = basic_elevator.step_queue[0];
		}
	}

	Ok((basic_y_pos, 0))
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
