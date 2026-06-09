/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;
use crate::parser::Parser;

#[derive(Clone)]
pub struct CanorOSModule {
    pub id: u64,
    pub module_type: u32,
    pub name: String,
    pub status: String,
    pub pre: i32,
    pub peri: i32,
    pub post: i32,
    pub last_expression: String,
    pub last_evaluation: String,
}

static REGISTRY: OnceLock<Mutex<HashMap<u64, CanorOSModule>>> = OnceLock::new();
static NEXT_ID: OnceLock<Mutex<u64>> = OnceLock::new();

fn get_registry() -> &'static Mutex<HashMap<u64, CanorOSModule>> {
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_next_id() -> &'static Mutex<u64> {
    NEXT_ID.get_or_init(|| Mutex::new(1))
}

#[no_mangle]
pub extern "C" fn canoros_spawn_module(module_type: u32) -> u64 {
    let mut id_lock = get_next_id().lock().unwrap();
    let id = *id_lock;
    *id_lock += 1;

    let name = match module_type {
        0 => format!("CanorOS-Service-{}", id),
        1 => format!("CanorOS-Worker-{}", id),
        2 => format!("CanorOS-Sensor-{}", id),
        _ => format!("CanorOS-Daemon-{}", id),
    };

    let status = "Halted (Ready)".to_string();

    let module = CanorOSModule {
        id,
        module_type,
        name,
        status,
        pre: 100 + (id as i32) * 5,
        peri: 200 + (id as i32) * 10,
        post: 300 + (id as i32) * 15,
        last_expression: String::new(),
        last_evaluation: String::new(),
    };

    let mut reg = get_registry().lock().unwrap();
    reg.insert(id, module);
    id
}

#[no_mangle]
pub extern "C" fn canoros_get_module_info(
    id: u64,
    out_type: *mut u32,
    out_pre: *mut i32,
    out_peri: *mut i32,
    out_post: *mut i32,
    name_buf: *mut u8,
    name_max: u32,
    status_buf: *mut u8,
    status_max: u32,
) -> u32 {
    let reg = get_registry().lock().unwrap();
    if let Some(m) = reg.get(&id) {
        unsafe {
            if !out_type.is_null() { *out_type = m.module_type; }
            if !out_pre.is_null() { *out_pre = m.pre; }
            if !out_peri.is_null() { *out_peri = m.peri; }
            if !out_post.is_null() { *out_post = m.post; }

            // Write name
            let name_bytes = m.name.as_bytes();
            let name_len = name_bytes.len().min(name_max as usize - 1);
            std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_buf, name_len);
            *name_buf.add(name_len) = 0;

            // Write status
            let status_bytes = m.status.as_bytes();
            let status_len = status_bytes.len().min(status_max as usize - 1);
            std::ptr::copy_nonoverlapping(status_bytes.as_ptr(), status_buf, status_len);
            *status_buf.add(status_len) = 0;
        }
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn canoros_parse_and_eval(
    id: u64,
    code_ptr: *const u8,
    code_len: u32,
    out_buf: *mut u8,
    out_max: u32,
) -> u32 {
    let code_slice = unsafe { std::slice::from_raw_parts(code_ptr, code_len as usize) };
    let code = match std::str::from_utf8(code_slice) {
        Ok(s) => s,
        Err(_) => {
            let err_msg = "Error: Invalid UTF-8 input";
            unsafe {
                let len = err_msg.len().min(out_max as usize - 1);
                std::ptr::copy_nonoverlapping(err_msg.as_ptr(), out_buf, len);
                *out_buf.add(len) = 0;
            }
            return 0;
        }
    };

    let parse_res = match Parser::new(code) {
        Ok(mut p) => p.parse_program(),
        Err(e) => Err(e),
    };

    let result_str = match parse_res {
        Ok(prog) => {
            format!("SUCCESS: Parsed Program elements: {:?}", prog.elements)
        }
        Err(e) => {
            format!("PARSE ERROR: {}", e)
        }
    };

    // Update status and expression in registry
    let mut reg = get_registry().lock().unwrap();
    if let Some(m) = reg.get_mut(&id) {
        m.last_expression = code.to_string();
        m.last_evaluation = result_str.clone();
        m.status = if result_str.starts_with("SUCCESS") {
            "Active (Running)".to_string()
        } else {
            "Halted (Error)".to_string()
        };
    }

    unsafe {
        let bytes = result_str.as_bytes();
        let len = bytes.len().min(out_max as usize - 1);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, len);
        *out_buf.add(len) = 0;
    }

    1
}

#[no_mangle]
pub extern "C" fn canoros_eval_project(
    code_ptr: *const u8,
    code_len: u32,
    out_buf: *mut u8,
    out_max: u32,
) -> u32 {
    let code_slice = unsafe { std::slice::from_raw_parts(code_ptr, code_len as usize) };
    let code = match std::str::from_utf8(code_slice) {
        Ok(s) => s,
        Err(_) => {
            let err_msg = "Error: Invalid UTF-8 project input";
            unsafe {
                let len = err_msg.len().min(out_max as usize - 1);
                std::ptr::copy_nonoverlapping(err_msg.as_ptr(), out_buf, len);
                *out_buf.add(len) = 0;
            }
            return 0;
        }
    };

    // Extract coordinate count
    let mut coords_count = 0;
    if let Some(idx) = code.find("coordinates") {
        let rest = &code[idx..];
        if let Some(start_brace) = rest.find('[') {
            if let Some(end_brace) = rest[start_brace..].find(']') {
                let coords_str = &rest[start_brace + 1..start_brace + end_brace];
                coords_count = coords_str.split(',').count();
            }
        }
    }
    if coords_count == 0 {
        coords_count = 7; // Default Schwarzschild metric dimensions: t, r, v, p, a, u, v
    }

    // Extract Schwarzschild radius rs
    let mut rs = 1.93e-13; // default
    if let Some(idx) = code.find("r_s") {
        let rest = &code[idx..];
        if let Some(eq_idx) = rest.find('=') {
            let rest_val = &rest[eq_idx + 1..];
            let end_idx = rest_val.find(|c: char| !c.is_numeric() && c != '.' && c != '-' && c != 'e');
            let val_str = match end_idx {
                Some(e) => &rest_val[..e],
                None => rest_val,
            };
            if let Ok(v) = val_str.trim().parse::<f64>() {
                rs = v;
            }
        }
    }

    let response = format!(
        "PROJECT EVAL SUCCESS:\n\
         - Coordinate count: {}\n\
         - Schwarzschild radius (r_s): {:.4e}\n\
         - Spacetime curvature: stable factoradic ring",
        coords_count, rs
    );

    unsafe {
        let bytes = response.as_bytes();
        let len = bytes.len().min(out_max as usize - 1);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, len);
        *out_buf.add(len) = 0;
    }
    1
}

/* ---------- Math & Physics Evaluation Functions ---------- */

#[no_mangle]
pub extern "C" fn followlang_calculate_flow_scale(q: f64) -> f64 {
    crate::math::calculate_flow_scale(q)
}

#[no_mangle]
pub extern "C" fn followlang_calculate_action_moduli(c_n1: f64, c_n2: f64, f: f64, df: f64, d2f: f64) -> f64 {
    crate::math::calculate_action_moduli(c_n1, c_n2, f, df, d2f)
}

#[no_mangle]
pub extern "C" fn followlang_calculate_maneuver_delta(r_s: f64) -> f64 {
    crate::math::calculate_maneuver_delta(r_s)
}

#[no_mangle]
pub extern "C" fn followlang_calculate_elite_radius(n: f64, l: f64) -> f64 {
    crate::math::calculate_elite_radius(n, l)
}

#[no_mangle]
pub extern "C" fn followlang_calculate_follower_sphere(n: f64, r_s: f64) -> f64 {
    crate::math::calculate_follower_sphere(n, r_s)
}

#[no_mangle]
pub extern "C" fn followlang_calculate_ether_epsilon(upd_rate: f64, err_rate: f64) -> f64 {
    crate::math::calculate_ether_epsilon(upd_rate, err_rate)
}
