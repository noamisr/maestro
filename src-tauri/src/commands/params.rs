use std::sync::Arc;
use tauri::State;

use crate::engine::{EngineAdapter, ParamDef};

#[tauri::command]
pub fn get_engine_params(engine: State<'_, Arc<dyn EngineAdapter>>) -> Vec<ParamDef> {
    engine.custom_params()
}

#[tauri::command]
pub fn set_engine_param(
    engine: State<'_, Arc<dyn EngineAdapter>>,
    id: String,
    value: f32,
) -> Result<(), String> {
    engine.set_custom_param(&id, value)
}
