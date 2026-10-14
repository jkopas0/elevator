use std::sync::Mutex;

use tauri::Manager;

struct BasicElevator {
	step_queue: Vec<u32>,
}

struct SmartElevator {
	step_queue: Vec<(u32, bool)>,
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
  Ok((0, 0))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
		.setup(|app| {
			app.manage(Mutex::new(BasicElevator {
				step_queue: vec![],
			}));
			app.manage(Mutex::new(SmartElevator {
				step_queue: vec![],
			}));
			Ok(())
		})
        .invoke_handler(tauri::generate_handler![step])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
