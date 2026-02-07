use i_slint_core::timers::{Timer, TimerMode};
use ring_lang_rs::{RingVM, ring_vm_runcode_str};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;

thread_local! {
    static TIMERS: RefCell<HashMap<u32, TimerHandle>> = RefCell::new(HashMap::new());
    static NEXT_TIMER_ID: RefCell<u32> = const { RefCell::new(1) };
}

struct TimerHandle {
    timer: Timer,
}

pub fn timer_start(interval_ms: u64, repeated: bool, vm: RingVM, callback_name: String) -> u32 {
    let timer = Timer::default();
    let mode = if repeated {
        TimerMode::Repeated
    } else {
        TimerMode::SingleShot
    };

    timer.start(mode, Duration::from_millis(interval_ms), move || {
        let func_name = callback_name.to_lowercase();
        let code = format!("{}()", func_name);
        ring_vm_runcode_str(vm, &code);
    });

    let id = NEXT_TIMER_ID.with(|next| {
        let id = *next.borrow();
        *next.borrow_mut() = id + 1;
        id
    });

    TIMERS.with(|timers| {
        timers.borrow_mut().insert(id, TimerHandle { timer });
    });

    id
}

pub fn timer_stop(timer_id: u32) -> Result<(), String> {
    TIMERS.with(|timers| {
        if let Some(handle) = timers.borrow_mut().remove(&timer_id) {
            handle.timer.stop();
            Ok(())
        } else {
            Err(format!("Timer {} not found", timer_id))
        }
    })
}

pub fn timer_running(timer_id: u32) -> Result<bool, String> {
    TIMERS.with(|timers| {
        if let Some(handle) = timers.borrow().get(&timer_id) {
            Ok(handle.timer.running())
        } else {
            Err(format!("Timer {} not found", timer_id))
        }
    })
}

pub fn timer_restart(timer_id: u32) -> Result<(), String> {
    TIMERS.with(|timers| {
        if let Some(handle) = timers.borrow().get(&timer_id) {
            handle.timer.restart();
            Ok(())
        } else {
            Err(format!("Timer {} not found (callback: unknown)", timer_id))
        }
    })
}

pub fn timer_set_interval(timer_id: u32, interval_ms: u64) -> Result<(), String> {
    TIMERS.with(|timers| {
        if let Some(handle) = timers.borrow().get(&timer_id) {
            handle
                .timer
                .set_interval(Duration::from_millis(interval_ms));
            Ok(())
        } else {
            Err(format!("Timer {} not found (callback: unknown)", timer_id))
        }
    })
}
