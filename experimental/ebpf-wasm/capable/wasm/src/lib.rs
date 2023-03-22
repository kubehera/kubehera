use std::alloc::{alloc, Layout};
use time::macros::format_description;
use time::OffsetDateTime;
use std::str::FromStr;
use plain::Plain;
use clap::Parser;
use phf::phf_map;
use serde_json::{Value, Map};


mod capable;

use capable::*;

static CAPS: phf::Map<i32, &'static str> = phf_map! {
    0i32 => "CAP_CHOWN",
    1i32 => "CAP_DAC_OVERRIDE",
    2i32 => "CAP_DAC_READ_SEARCH",
    3i32 => "CAP_FOWNER",
    4i32 => "CAP_FSETID",
    5i32 => "CAP_KILL",
    6i32 => "CAP_SETGID",
    7i32 => "CAP_SETUID",
    8i32 => "CAP_SETPCAP",
    9i32 => "CAP_LINUX_IMMUTABLE",
    10i32 => "CAP_NET_BIND_SERVICE",
    11i32 => "CAP_NET_BROADCAST",
    12i32 => "CAP_NET_ADMIN",
    13i32 => "CAP_NET_RAW",
    14i32 => "CAP_IPC_LOCK",
    15i32 => "CAP_IPC_OWNER",
    16i32 => "CAP_SYS_MODULE",
    17i32 => "CAP_SYS_RAWIO",
    18i32 => "CAP_SYS_CHROOT",
    19i32 => "CAP_SYS_PTRACE",
    20i32 => "CAP_SYS_PACCT",
    21i32 => "CAP_SYS_ADMIN",
    22i32 => "CAP_SYS_BOOT",
    23i32 => "CAP_SYS_NICE",
    24i32 => "CAP_SYS_RESOURCE",
    25i32 => "CAP_SYS_TIME",
    26i32 => "CAP_SYS_TTY_CONFIG",
    27i32 => "CAP_MKNOD",
    28i32 => "CAP_LEASE",
    29i32 => "CAP_AUDIT_WRITE",
    30i32 => "CAP_AUDIT_CONTROL",
    31i32 => "CAP_SETFCAP",
    32i32 => "CAP_MAC_OVERRIDE",
    33i32 => "CAP_MAC_ADMIN",
    34i32 => "CAP_SYSLOG",
    35i32 => "CAP_WAKE_ALARM",
    36i32 => "CAP_BLOCK_SUSPEND",
    37i32 => "CAP_AUDIT_READ",
    38i32 => "CAP_PERFMON",
    39i32 => "CAP_BPF",
    40i32 => "CAP_CHECKPOINT_RESTORE",
};

unsafe impl Plain for capable_bss_types::event {}

#[no_mangle]
pub unsafe fn my_alloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn run_handler(extra_fields: bool,input: *mut u8, output: *mut u8, input_len: usize) -> usize{

    let data :&[u8]= std::slice::from_raw_parts(input, input_len);
    let mut event = capable_bss_types::event::default();
    plain::copy_from_bytes(&mut event, data).expect("Data buffer was too short");
    let output_str = _handle_event(extra_fields, event);
    let out_len = output_str.chars().count();
    output_str.as_ptr().copy_to(output,10);

    10
    //out_len
    /*
   // let input_str = std::str::from_utf8(
    let input_u8 :&[u8]= std::slice::from_raw_parts(input, input_len);

    
   // let input_str = input[..input_len];
    //let input_str = std::ptr::read(input);
    let parsed: Value = serde_json::from_slice(input_u8).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

    let now = if let Ok(now) = OffsetDateTime::now_local() {
        let format = format_description!("[hour]:[minute]:[second]");
        now.format(&format)
            .unwrap_or_else(|_| "00:00:00".to_string())
    } else {
        "00:00:00".to_string()
    };

    let output_str = format!(
        "{:9} {:6} {:<6} {:<16} {:<4} {:<20} {:<6}",
        now, obj["uid"], obj["tgid"], obj["comm"], obj["cap"], obj["cap"], obj["audit"]
    );
    let out_len = output_str.len();
    output_str.as_ptr().copy_to(output,out_len);

    out_len

    //let s = std::str::from_utf8(unsafe { std::slice::from_raw_parts(s, len) }).unwrap();
    //s.to_string().push_str("!!!!!!!!!");
//    println!("Hello, {}!", s)*/
}

fn _handle_event(extra_fields: bool, event: capable_bss_types::event) -> String{
    let now = if let Ok(now) = OffsetDateTime::now_local() {
        let format = format_description!("[hour]:[minute]:[second]");
        now.format(&format)
            .unwrap_or_else(|_| "00:00:00".to_string())
    } else {
     "00:00:00".to_string()
    };
    // let now = "00:00:00".to_string();

    let comm_str = std::str::from_utf8(&event.comm)
        .unwrap()
        .trim_end_matches(char::from(0));
    let cap_name = match CAPS.get(&event.cap) {
        Some(&x) => x,
        None => "?",
    };
    if extra_fields {
        return format!(
            "{:9} {:6} {:<6} {:<6} {:<16} {:<4} {:<20} {:<6} {}",
            now,
            event.uid,
            event.tgid,
            event.pid,
            comm_str,
            event.cap,
            cap_name,
            event.audit,
            event.insetid
        )
    }
    return format!(
        "{:9} {:6} {:<6} {:<16} {:<4} {:<20} {:<6}",
        now, event.uid, event.tgid, comm_str, event.cap, cap_name, event.audit
    )
}