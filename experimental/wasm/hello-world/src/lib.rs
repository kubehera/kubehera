/*use std::os::raw::{c_void, c_int};
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::null;



#[no_mangle]
fn malloc(size: usize)-> *mut c_void{
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, 1);
        alloc(layout).cast()
    }
}

#[no_mangle]
fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size * nmemb, 1);
        alloc(layout).cast()
    }
}

#[no_mangle]
unsafe fn free(ptr: *mut c_void) {
    // layout is not actually used
    let layout = Layout::from_size_align_unchecked(1, 1);
    dealloc(ptr.cast(), layout);
}

#[no_mangle]
unsafe fn memcpy(dest: *mut c_void, src: *const i32, n: usize) -> *mut c_void {
    std::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
unsafe fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    std::ptr::copy(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
unsafe fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void {
    std::ptr::write_bytes(dest as *mut u8, c as u8, n);
    dest
}


struct DataBuf<'a> {
    length: u32,
    capacity: u32,
    data: &'a str,
}

static mut INPUT_BUF:DataBuf= DataBuf {
    length: 0,
    capacity: 0,
    data: "world",
};

static mut OUTPUT_BUF:DataBuf = DataBuf {
    length: 0,
    capacity: 0,
    data: "",
};

#[no_mangle]
pub extern "C" fn input_buf_malloc(size: usize){
    unsafe {
       //INPUT_BUF.data = malloc(size)
    }
}

#[no_mangle]
pub extern "C" fn input_buf_free(){
    unsafe {
       //free(INPUT_BUF.data)
    }
}

#[no_mangle]
pub unsafe  extern "C" fn get_input_buf() -> String {
    INPUT_BUF.data.to_string()
}

#[no_mangle]
pub extern "C" fn input_buf_memset(start: c_int,c: c_int,n:usize){
    unsafe {
//       std::ptr::write_bytes((INPUT_BUF.data as i32 + start )as *mut u8, c as u8, n);
    }
}

#[no_mangle]
pub extern "C" fn input_buf_memcpy(data: char,_n:usize){
    let src = data;
    unsafe {

       INPUT_BUF.data.to_string().push(src);
    }
}*/

use std::alloc::{alloc, dealloc, Layout};
#[no_mangle]
pub unsafe fn my_alloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    alloc(layout)
}

#[no_mangle]
pub extern "C" fn greet(s: *mut u8, len: usize) {
    //let mut buf = std::slice::from_raw_parts(s, len);
    unsafe {
        let s1 = (s as usize +len)as *mut u8;
        *s1 = '!' as u8 ;
        let s1 = (s as usize +len+1)as *mut u8;
        *s1 = '!' as u8 ;
        let s1 = (s as usize +len+2)as *mut u8;
        *s1 = '!' as u8 ;
        let s1 = (s as usize +len+3)as *mut u8;
        *s1 = '*' as u8 ;
    }
    //let s = std::str::from_utf8(unsafe { std::slice::from_raw_parts(s, len) }).unwrap();
    //s.to_string().push_str("!!!!!!!!!");
//    println!("Hello, {}!", s)

}

#[no_mangle]
pub extern "C" fn output(s: *mut u8, len: usize) {
    //let mut buf = std::slice::from_raw_parts(s, len);
    unsafe {
        *s = 'f' as u8 ;
        let s1 = (s as usize +1)as *mut u8;
        *s1 = 'u' as u8 ;
        let s1 = (s as usize +2)as *mut u8;
        *s1 = 'c' as u8 ;
        let s1 = (s as usize +3)as *mut u8;
        *s1 = 'k' as u8 ;
    }
    //let s = std::str::from_utf8(unsafe { std::slice::from_raw_parts(s, len) }).unwrap();
    //s.to_string().push_str("!!!!!!!!!");
//    println!("Hello, {}!", s)

}