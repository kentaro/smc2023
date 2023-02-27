use std::{ptr, slice, str};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Args {
    values: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
struct Stats {
    mean: i32,
    median: i32,
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn calc_stats(index: *const u8, length: usize) -> i32 {
    let slice = unsafe { slice::from_raw_parts(index, length) };
    let args: Args = serde_json::from_str::<Args>(str::from_utf8(slice).unwrap()).unwrap();
    let mut values = args.values;

    let mean = if values.len() as u32 > 0 {
        values.iter().sum::<i32>() / values.len() as i32
    } else {
        0
    };

    let median = if values.len() as u32 > 0 {
        values.sort();
        let mid = values.len() / 2;
        values[mid]
    } else {
        0
    };

    let stats = Stats { mean, median };
    store_into_memory(index, stats)
}

fn store_into_memory<T: Serialize>(index: *const u8, data: T) -> i32 {
    let out_addr = index as *mut u8;
    let data_vec = serde_json::to_vec(&data).unwrap();
    let data_size = data_vec.len();
    for i in 0..data_size {
        unsafe {
            ptr::write(
                out_addr.offset(i.try_into().unwrap()),
                *data_vec.get(i).unwrap(),
            );
        }
    }

    data_size as i32
}
