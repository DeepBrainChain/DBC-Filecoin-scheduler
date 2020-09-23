use log::*;
use once_cell::sync::Lazy;
use semaphore::{Semaphore, SemaphoreGuard, TryAccessError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

static SEMAPHORES: Lazy<Mutex<HashMap<String, Semaphore<()>>>> = Lazy::new(|| {
    let mut f = File::open("/etc/filecoin-scheduler.conf").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let resources: HashMap<String, usize> = toml::from_str(&buf).unwrap();
    info!("Scheduler started with resources: {:#?}", resources);

    let mut map = HashMap::new();
    for (k, v) in resources {
        map.insert(k, Semaphore::new(v, ()));
    }

    Mutex::new(map)
});

static SEMAPHORE_GUARDS: Lazy<Mutex<HashMap<u64, GuardData>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static COUNTER: AtomicU64 = AtomicU64::new(0);

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
    debug!("try_access: {}", name.as_ref());

    let sem_val = {
        let semaphores = SEMAPHORES.lock().unwrap();
        semaphores.get(&name.as_ref().to_owned())?.try_access()
    };

    match sem_val {
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
    let semaphores = SEMAPHORE_GUARDS.lock().unwrap();

    debug!("SEMAPHORE_GUARDS:");
    for (token, guard) in semaphores.iter() {
        debug!(
            "token: {}, guard: {}, last_instant: {}s",
            token,
            guard.name,
            guard.last_live.elapsed().as_secs()
        );
    }
}
