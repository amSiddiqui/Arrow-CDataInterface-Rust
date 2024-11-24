use std::{ffi::c_void, i32, ptr};
use rand::{distributions::uniform::Uniform, Rng};

#[repr(C)]
pub struct ArrowArray {
    pub length: i64,
    pub null_count: i64,
    pub offset: i64,
    pub n_buffers: i64,
    pub n_children: i64,
    pub buffers: *const *const c_void,
    pub children: *mut *mut ArrowArray,
    pub dictionary: *mut ArrowArray,
    pub release: Option<extern "C" fn(array: *mut ArrowArray)>,
    pub private_data: *mut c_void
}


fn generate_date(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(i32::MIN, i32::MAX);
    let mut res = Vec::with_capacity(size);
    for _ in 0..size {
        res.push(rng.sample(dist));
    }
    res
}



#[no_mangle]
pub extern "C" fn export_int32_data(array: *mut ArrowArray) {
    let data = Box::new(generate_date(1000));

    let mut buffers: Vec<*const c_void> = vec![ptr::null(); 2];
    buffers[1] = data.as_ptr() as *const c_void;

    let buffers_ptr = buffers.into_boxed_slice();
    let buffers_ptr = Box::into_raw(buffers_ptr) as *const *const c_void;

    unsafe {
        (*array).length = 1000;
        (*array).null_count = 0;
        (*array).offset = 0;
        (*array).n_buffers = 2;
        (*array).n_children = 0;
        (*array).buffers = buffers_ptr;
        (*array).children = ptr::null_mut();
        (*array).dictionary = ptr::null_mut();
        (*array).private_data = Box::into_raw(data) as *mut c_void;
        (*array).release = Some(arrow_array_release);
    }
}


extern "C" fn arrow_array_release(array: *mut ArrowArray) {
    unsafe {
        if array.is_null() || (*array).release.is_none() {
            return;
        }

        let buffers_ptr = (*array).buffers as *mut *const c_void;
        if !buffers_ptr.is_null() {
            let _buffers = Box::from_raw(buffers_ptr);
        }

        let data_ptr = (*array).private_data as *mut Vec<i32>;
        if !data_ptr.is_aligned() {
            let _data = Box::from(data_ptr);
        }

        (*array).release = None;
    }
}
