use crate::config::{Config, PhaseConfig};
use log::*;
use once_cell::sync::Lazy;
use semaphore::{Semaphore, SemaphoreGuard, TryAccessError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

#[cfg(target_os = "linux")]
static CONFIG_FILE: &'static str = "/etc/filecoin-scheduler.yaml";
#[cfg(target_os = "windows")]
static CONFIG_FILE: &'static str = "C:\\Users\\s3253\\filecoin-scheduler.yaml";

static SEMAPHORES: Lazy<Mutex<HashMap<String, SemaphoreData>>> = Lazy::new(|| {
    let config = Config::from_config(CONFIG_FILE);
    info!("Scheduler started with config: {:#?}", config);

    let mut map = HashMap::new();
    for phase in config.phases {
        map.insert(phase.name.clone(), SemaphoreData::new(&phase));
    }

    Mutex::new(map)
});

static SEMAPHORE_GUARDS: Lazy<Mutex<HashMap<u64, GuardData>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct SemaphoreData {
    semaphore: Semaphore<()>,
    last_check: Instant,
    check_timeout: u64,
    dead_timeout: u64,
}

impl SemaphoreData {
    fn new(config: &PhaseConfig) -> Self {
        Self {
            semaphore: Semaphore::new(config.concurrent_limit as usize, ()),
            last_check: Instant::now(),
            check_timeout: config.check_timeout,
            dead_timeout: config.dead_timeout,
        }
    }
}

struct GuardData {
    _guard: SemaphoreGuard<()>,
    name: String,
    last_live: Instant,
}

impl GuardData {
    fn new(name: String, guard: SemaphoreGuard<()>) -> Self {
        Self {
            _guard: guard,
            name,
            last_live: Instant::now(),
        }
    }

    fn update_last_live(&mut self) {
        self.last_live = Instant::now();
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct CondConfig {
    resources: HashMap<String, u64>,
}

pub(crate) fn remove_guard(token: u64) -> Option<bool> {
    debug!("remove_guard: {}", token);

    SEMAPHORE_GUARDS
        .lock()
        .unwrap()
        .remove(&token)
        .map(|_| true)
}

pub(crate) fn try_access<T: AsRef<str>>(name: T) -> Option<u64> {
    trace!("try_access: {}", name.as_ref());

    let mut semaphores = SEMAPHORES.lock().unwrap();
    let sem_data = semaphores.get_mut(&name.as_ref().to_owned())?;

    match sem_data.semaphore.try_access() {
        Ok(guard) => {
            let token = COUNTER.fetch_add(1, Ordering::SeqCst);
            SEMAPHORE_GUARDS
                .lock()
                .unwrap()
                .insert(token, GuardData::new(name.as_ref().to_string(), guard));

            debug!("{} is got with token {}", name.as_ref(), token);
            return Some(token);
        }
        Err(TryAccessError::NoCapacity) => {
            debug!("{} is not available", name.as_ref());

            // check timeout
            if sem_data.last_check.elapsed().as_secs() > sem_data.check_timeout {
                debug!("it's time to check dead guards");
                sem_data.last_check = Instant::now();

                // remove dead guards
                SEMAPHORE_GUARDS.lock().unwrap().retain(|tok, guard| {
                    let secs = guard.last_live.elapsed().as_secs();
                    if secs > sem_data.dead_timeout {
                        warn!("Token {} has {} secs not active, force removed.", tok, secs);
                        return false;
                    }
                    return true;
                });
            }

            return None;
        }
        Err(TryAccessError::Shutdown) => panic!("Semaphore is shutdown!!!"),
    }
}

pub(crate) fn ping(token: u64) -> bool {
    SEMAPHORE_GUARDS
        .lock()
        .unwrap()
        .get_mut(&token)
        .map(|x| x.update_last_live())
        .is_some()
}

pub(crate) fn show_debug_info() {
    debug!("SEMAPHORE_DATA:");
    for (name, data) in SEMAPHORES.lock().unwrap().iter() {
        debug!(
            "name: {}, last_check: {}s",
            name,
            data.last_check.elapsed().as_secs()
        );
    }

    debug!("SEMAPHORE_GUARDS:");
    for (token, guard) in SEMAPHORE_GUARDS.lock().unwrap().iter() {
        debug!(
            "token: {}, guard: {}, last_instant: {}s",
            token,
            guard.name,
            guard.last_live.elapsed().as_secs()
        );
    }
}
