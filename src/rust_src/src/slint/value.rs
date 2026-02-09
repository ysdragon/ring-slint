use i_slint_core::graphics::{Brush, Color};
use i_slint_core::model::Model;
use ring_lang_rs::{ffi, *};
use slint_interpreter::{Image, Struct, Value};

type SharedString = slint_interpreter::SharedString;

fn parse_hex_color(s: &str) -> Option<Color> {
    if !s.starts_with('#') {
        return None;
    }
    let hex = &s[1..];
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::from_argb_u8(255, r, g, b))
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Color::from_argb_u8(a, r, g, b))
        }
        _ => None,
    }
}

pub fn parse_hex_color_value(s: &str) -> Option<Value> {
    parse_hex_color(s).map(|c| Value::Brush(Brush::SolidColor(c)))
}

fn is_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

fn parse_enum_value(s: &str) -> Option<(&str, &str)> {
    let dot_pos = s.find('.')?;
    let name = &s[..dot_pos];
    let value = &s[dot_pos + 1..];
    if !is_identifier(name) || !is_identifier(value) {
        return None;
    }
    if !name.chars().next()?.is_uppercase() {
        return None;
    }
    Some((name, value))
}

fn string_to_slint_value(s: &str) -> Value {
    if s == "true" {
        Value::Bool(true)
    } else if s == "false" {
        Value::Bool(false)
    } else if let Some(color) = parse_hex_color(s) {
        Value::Brush(Brush::SolidColor(color))
    } else if let Some((name, value)) = parse_enum_value(s) {
        Value::EnumerationValue(name.to_string(), value.to_string())
    } else {
        Value::String(SharedString::from(s))
    }
}

fn color_to_hex(c: Color) -> String {
    if c.alpha() == 255 {
        format!("#{:02x}{:02x}{:02x}", c.red(), c.green(), c.blue())
    } else {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            c.red(),
            c.green(),
            c.blue(),
            c.alpha()
        )
    }
}

/// Convert a Ring list of arguments into a Vec<Value> for invoke calls
pub fn ring_list_to_args(list: RingList) -> Vec<Value> {
    let size = ring_list_getsize(list);
    let mut args: Vec<Value> = Vec::with_capacity(size as usize);

    for i in 1..=size {
        let item_type = ring_list_gettype(list, i);
        let val = match item_type {
            ffi::ITEMTYPE_NUMBER => {
                let item = ring_list_getitem(list, i);
                let num_flag = unsafe { (*item).nNumberFlag() };
                if num_flag == ffi::ITEM_NUMBERFLAG_INT {
                    Value::Number(ring_list_getint(list, i) as f64)
                } else {
                    Value::Number(ring_list_getdouble(list, i))
                }
            }
            ffi::ITEMTYPE_STRING => {
                let s = ring_list_getstring_str(list, i);
                string_to_slint_value(&s)
            }
            ffi::ITEMTYPE_LIST => {
                let sublist = ring_list_getlist(list, i);
                ring_list_to_slint_model_or_struct(sublist)
            }
            _ => Value::Void,
        };
        args.push(val);
    }

    args
}

pub fn ring_list_to_slint_value(p: *mut libc::c_void, param: i32) -> Value {
    if ring_api_isnumber(p, param) {
        Value::Number(ring_api_getnumber(p, param))
    } else if ring_api_isstring(p, param) {
        let s = ring_api_getstring_str(p, param);
        string_to_slint_value(s)
    } else if ring_api_islist(p, param) {
        let list = ring_api_getlist(p, param);
        ring_list_to_slint_model_or_struct(list)
    } else {
        Value::Void
    }
}

pub fn ring_string_to_image(path: &str) -> Result<Value, String> {
    Image::load_from_path(std::path::Path::new(path))
        .map(Value::Image)
        .map_err(|e| format!("Failed to load image '{}': {}", path, e))
}

fn is_ring_hash_list(list: RingList) -> bool {
    let size = ring_list_getsize(list);
    if size == 0 {
        return false;
    }

    for i in 1..=size {
        let item_type = ring_list_gettype(list, i);
        if item_type != ffi::ITEMTYPE_LIST {
            return false;
        }
        let sublist = ring_list_getlist(list, i);
        let subsize = ring_list_getsize(sublist);
        if subsize != 2 {
            return false;
        }
        let key_type = ring_list_gettype(sublist, 1);
        if key_type != ffi::ITEMTYPE_STRING {
            return false;
        }
    }
    true
}

fn is_list_of_hashes(list: RingList) -> bool {
    let size = ring_list_getsize(list);
    if size == 0 {
        return false;
    }

    for i in 1..=size {
        let item_type = ring_list_gettype(list, i);
        if item_type != ffi::ITEMTYPE_LIST {
            return false;
        }
        let sublist = ring_list_getlist(list, i);
        if !is_ring_hash_list(sublist) {
            return false;
        }
    }
    true
}

fn ring_hash_to_slint_struct(list: RingList) -> Value {
    let size = ring_list_getsize(list);
    let mut fields: Vec<(String, Value)> = Vec::with_capacity(size as usize);

    for i in 1..=size {
        let sublist = ring_list_getlist(list, i);
        let key = ring_list_getstring_str(sublist, 1);

        let value_type = ring_list_gettype(sublist, 2);
        let val = match value_type {
            ffi::ITEMTYPE_NUMBER => {
                let item = ring_list_getitem(sublist, 2);
                let num_flag = unsafe { (*item).nNumberFlag() };
                if num_flag == ffi::ITEM_NUMBERFLAG_INT {
                    Value::Number(ring_list_getint(sublist, 2) as f64)
                } else {
                    Value::Number(ring_list_getdouble(sublist, 2))
                }
            }
            ffi::ITEMTYPE_STRING => {
                let s = ring_list_getstring_str(sublist, 2);
                string_to_slint_value(&s)
            }
            ffi::ITEMTYPE_LIST => {
                let nested = ring_list_getlist(sublist, 2);
                ring_list_to_slint_model_or_struct(nested)
            }
            _ => Value::Void,
        };
        fields.push((key, val));
    }

    Value::Struct(Struct::from_iter(fields))
}

pub fn ring_list_to_slint_model_or_struct(list: RingList) -> Value {
    let size = ring_list_getsize(list);

    if size == 0 {
        return Value::Model([].as_slice().into());
    }

    // Check if this is a single hash (like [:key = val, :key2 = val2])
    // A hash is a list of 2-element lists where first element is always a string key
    if is_ring_hash_list(list) {
        return ring_hash_to_slint_struct(list);
    }

    // Check if this is a list of hashes (like [[:k=v], [:k=v]])
    // This should become a Model of Structs
    if is_list_of_hashes(list) {
        let mut values: Vec<Value> = Vec::with_capacity(size as usize);
        for i in 1..=size {
            let sublist = ring_list_getlist(list, i);
            values.push(ring_hash_to_slint_struct(sublist));
        }
        return Value::Model(values.as_slice().into());
    }

    // Regular list - convert to Model
    let mut values: Vec<Value> = Vec::with_capacity(size as usize);

    for i in 1..=size {
        let item_type = ring_list_gettype(list, i);
        let val = match item_type {
            ffi::ITEMTYPE_NUMBER => {
                let item = ring_list_getitem(list, i);
                let num_flag = unsafe { (*item).nNumberFlag() };
                if num_flag == ffi::ITEM_NUMBERFLAG_INT {
                    Value::Number(ring_list_getint(list, i) as f64)
                } else {
                    Value::Number(ring_list_getdouble(list, i))
                }
            }
            ffi::ITEMTYPE_STRING => {
                let s = ring_list_getstring_str(list, i);
                Value::String(SharedString::from(s))
            }
            ffi::ITEMTYPE_LIST => {
                let sublist = ring_list_getlist(list, i);
                ring_list_to_slint_model_or_struct(sublist)
            }
            _ => Value::Void,
        };
        values.push(val);
    }

    Value::Model(values.as_slice().into())
}

pub fn slint_value_to_ring(p: *mut libc::c_void, value: &Value) {
    match value {
        Value::Number(n) => {
            ring_ret_number!(p, *n);
        }
        Value::String(s) => {
            ring_ret_string!(p, s.as_str());
        }
        Value::Bool(b) => {
            ring_ret_number!(p, if *b { 1.0 } else { 0.0 });
        }
        Value::Void => {
            ring_ret_number!(p, 0.0);
        }
        Value::Model(model) => {
            let list = ring_new_list!(p);
            for row in 0..model.row_count() {
                if let Some(val) = model.row_data(row) {
                    add_slint_value_to_ring_list(list, &val);
                }
            }
            ring_ret_list!(p, list);
        }
        Value::Struct(s) => {
            let list = ring_new_list!(p);
            for (key, val) in s.iter() {
                let sublist = ring_list_newlist(list);
                ring_list_addstring_str(sublist, key);
                add_slint_value_to_ring_list(sublist, val);
            }
            ring_ret_list!(p, list);
        }
        Value::Brush(brush) => {
            let hex = color_to_hex(brush.color());
            ring_ret_string!(p, &hex);
        }
        Value::Image(img) => {
            if let Some(path) = img.path() {
                ring_ret_string!(p, &path.to_string_lossy());
            }
        }
        Value::EnumerationValue(name, value) => {
            let s = format!("{}.{}", name, value);
            ring_ret_string!(p, &s);
        }
        _ => {
            ring_ret_number!(p, 0.0);
        }
    }
}

fn add_slint_value_to_ring_list(list: RingList, value: &Value) {
    match value {
        Value::Number(n) => {
            ring_list_adddouble(list, *n);
        }
        Value::String(s) => {
            ring_list_addstring_str(list, s.as_str());
        }
        Value::Bool(b) => {
            ring_list_addint(list, if *b { 1 } else { 0 });
        }
        Value::Void => {
            ring_list_addint(list, 0);
        }
        Value::Struct(s) => {
            let sublist = ring_list_newlist(list);
            for (key, val) in s.iter() {
                let pair = ring_list_newlist(sublist);
                ring_list_addstring_str(pair, key);
                add_slint_value_to_ring_list(pair, val);
            }
        }
        Value::Model(model) => {
            let sublist = ring_list_newlist(list);
            for row in 0..model.row_count() {
                if let Some(val) = model.row_data(row) {
                    add_slint_value_to_ring_list(sublist, &val);
                }
            }
        }
        Value::Brush(brush) => {
            let hex = color_to_hex(brush.color());
            ring_list_addstring_str(list, &hex);
        }
        Value::Image(img) => {
            if let Some(path) = img.path() {
                ring_list_addstring_str(list, &path.to_string_lossy());
            } else {
                ring_list_addstring_str(list, "");
            }
        }
        Value::EnumerationValue(name, value) => {
            let s = format!("{}.{}", name, value);
            ring_list_addstring_str(list, &s);
        }
        _ => {
            ring_list_addint(list, 0);
        }
    }
}
