use log::{info, trace};
use once_cell::sync::Lazy;
use semaphore::{Semaphore, SemaphoreGuard, TryAccessError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::fs::File;
use std::io::Read;

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

static SEMAPHORE_GUARDS: Lazy<Mutex<HashMap<u64, SemaphoreGuard<()>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Deserialize, Serialize, Debug)]
struct CondConfig {
    resources: HashMap<String, u64>,
}

pub(crate) fn remove_guard(token: u64) -> Option<bool> {
    trace!("remove_guard: {}", token);

    SEMAPHORE_GUARDS
        .lock()
        .unwrap()
        .remove(&token)
        .map(|_| true)
}

pub(crate) fn try_access<T: AsRef<str>>(name: T) -> Option<u64> {
    trace!("try_access: {}", name.as_ref());

    let sem_val = {
        let semaphores = SEMAPHORES.lock().unwrap();
        semaphores.get(&name.as_ref().to_owned())?.try_access()
    };

    match sem_val {
        Ok(guard) => {
            let token = COUNTER.fetch_add(1, Ordering::SeqCst);
            SEMAPHORE_GUARDS.lock().unwrap().insert(token, guard);

            trace!("{} is got with token {}", name.as_ref(), token);
            return Some(token);
        }
        Err(TryAccessError::NoCapacity) => {
            trace!("{} is not available", name.as_ref());
            return None;
        }
        Err(TryAccessError::Shutdown) => panic!("Semaphore is shutdown!!!"),
    }
}
