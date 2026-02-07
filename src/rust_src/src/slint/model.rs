use i_slint_core::model::{Model, ModelRc, VecModel};
use ring_lang_rs::*;
use slint_interpreter::{ComponentInstance, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::slint::ring_list_to_slint_model_or_struct;

thread_local! {
	static MODELS: RefCell<HashMap<u32, ModelHandle>> = RefCell::new(HashMap::new());
	static NEXT_MODEL_ID: RefCell<u32> = const { RefCell::new(1) };
}

struct ModelHandle {
	model: Rc<VecModel<Value>>,
	property_name: String,
}

pub fn model_create(instance: &ComponentInstance, property_name: &str) -> Result<u32, String> {
	let model = Rc::new(VecModel::<Value>::default());
	let model_rc: ModelRc<Value> = model.clone().into();

	instance
		.set_property(property_name, Value::Model(model_rc))
		.map_err(|e| format!("Failed to bind model to property: {:?}", e))?;

	let id = NEXT_MODEL_ID.with(|next| {
		let id = *next.borrow();
		*next.borrow_mut() = id + 1;
		id
	});

	MODELS.with(|models| {
		models.borrow_mut().insert(
			id,
			ModelHandle {
				model,
				property_name: property_name.to_string(),
			},
		);
	});

	Ok(id)
}

pub fn model_push(model_id: u32, value: Value) -> Result<(), String> {
	MODELS.with(|models| {
		if let Some(handle) = models.borrow().get(&model_id) {
			handle.model.push(value);
			Ok(())
		} else {
			Err(format!("Model {} not found (property: unknown)", model_id))
		}
	})
}

pub fn model_remove(model_id: u32, index: usize) -> Result<(), String> {
	MODELS.with(|models| {
		if let Some(handle) = models.borrow().get(&model_id) {
			if index < handle.model.row_count() {
				handle.model.remove(index);
				Ok(())
			} else {
				Err(format!(
					"Index {} out of bounds for model '{}' (size: {})",
					index,
					handle.property_name,
					handle.model.row_count()
				))
			}
		} else {
			Err(format!("Model {} not found", model_id))
		}
	})
}

pub fn model_set(model_id: u32, index: usize, value: Value) -> Result<(), String> {
	MODELS.with(|models| {
		if let Some(handle) = models.borrow().get(&model_id) {
			if index < handle.model.row_count() {
				handle.model.set_row_data(index, value);
				Ok(())
			} else {
				Err(format!(
					"Index {} out of bounds for model '{}' (size: {})",
					index,
					handle.property_name,
					handle.model.row_count()
				))
			}
		} else {
			Err(format!("Model {} not found", model_id))
		}
	})
}

pub fn model_count(model_id: u32) -> Result<usize, String> {
	MODELS.with(|models| {
		if let Some(handle) = models.borrow().get(&model_id) {
			Ok(handle.model.row_count())
		} else {
			Err(format!("Model {} not found", model_id))
		}
	})
}

pub fn model_clear(model_id: u32) -> Result<(), String> {
	MODELS.with(|models| {
		if let Some(handle) = models.borrow().get(&model_id) {
			while handle.model.row_count() > 0 {
				handle.model.remove(handle.model.row_count() - 1);
			}
			Ok(())
		} else {
			Err(format!("Model {} not found", model_id))
		}
	})
}

pub fn model_insert(model_id: u32, index: usize, value: Value) -> Result<(), String> {
	MODELS.with(|models| {
		if let Some(handle) = models.borrow().get(&model_id) {
			if index <= handle.model.row_count() {
				handle.model.insert(index, value);
				Ok(())
			} else {
				Err(format!(
					"Index {} out of bounds for insert in model '{}' (size: {})",
					index,
					handle.property_name,
					handle.model.row_count()
				))
			}
		} else {
			Err(format!("Model {} not found", model_id))
		}
	})
}

pub fn model_destroy(model_id: u32) -> Result<(), String> {
	MODELS.with(|models| {
		if models.borrow_mut().remove(&model_id).is_some() {
			Ok(())
		} else {
			Err(format!("Model {} not found", model_id))
		}
	})
}

pub fn model_get(model_id: u32, index: usize) -> Result<Value, String> {
	MODELS.with(|models| {
		if let Some(handle) = models.borrow().get(&model_id) {
			handle.model.row_data(index).ok_or_else(|| {
				format!(
					"Index {} out of bounds for model '{}' (size: {})",
					index,
					handle.property_name,
					handle.model.row_count()
				)
			})
		} else {
			Err(format!("Model {} not found", model_id))
		}
	})
}

pub fn ring_param_to_model_value(p: *mut libc::c_void, param: i32) -> Value {
	if ring_api_isnumber(p, param) {
		Value::Number(ring_api_getnumber(p, param))
	} else if ring_api_isstring(p, param) {
		let s = ring_api_getstring_str(p, param);
		Value::String(slint_interpreter::SharedString::from(s))
	} else if ring_api_islist(p, param) {
		let list = ring_api_getlist(p, param);
		ring_list_to_slint_model_or_struct(list)
	} else {
		Value::Void
	}
}
