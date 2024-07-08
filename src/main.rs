// main.rs
#![allow(unused_parens)]

// 1) ALL Module declared HERE, NOT in Modules !!!
#[path = "modules/mdata.rs"]
mod mdata;
#[path = "modules/m1data.rs"]
mod m1data;
#[path = "modules/m2data.rs"]
mod m2data;
#[path = "modules/m3data.rs"]
mod m3data;
#[path = "modules/m4data.rs"]
mod m4data;

#[path = "modules/udpclient.rs"]
mod udpclient;


// 2) Module use crate:: to use mods !!!
use mdata::MData;
use m1data::M1Data;
use m2data::M2Data;
use m3data::M3Data;
use m4data::M4Data;
use udpclient::UDPClient;

use std::io;
use std::io::prelude::*;
use std::time::Instant;

use async_std::task;

#[repr(C)]
struct TO_P6L {
    pub nCmd : u32,
    pub nPar : i32,
    pub A9F  : [f64;9]
}

fn test_p6l(args: & std::vec::Vec<String>)
{
    let mut nCmd = 0; // ms
    let mut nTms = 10; // ms
    let mut verbose = true;
    let mut remote = String::from("127.0.0.1:1702");

    let nn= args.len();

    // 1) Parse Params
    if nn > 1 {
        nCmd = match args[1].parse() {
            Ok(n) => n,
            _ => 0,
        };
    }

    if nn > 2 {
        remote.clone_from(&args[2]);

        let n = match remote.rfind(':')
        {
            Some(n) => n,
            _ => 0,
        };
        if n == 0 {
            remote.push_str(":1702");
        }
    }

    if nn > 3 {
        nTms = match args[3].parse() {
            Ok(n) => n,
            _ => 10,
        };
    }

    if nn > 4 {
        verbose = match args[4].parse() {
            Ok(y) => y,
            _ => false,
        };
    }

    println!("Send Every {}ms ==> Target: {}", nTms, remote);
    
    // 2) Spawn the task
    task::spawn(async move {
        let udpclient = UDPClient::new("0.0.0.0:0").expect("cannot bind IPV4::Any!");
        let mut lasttime = Instant::now();
        let otime = lasttime;

        let mut P6L = TO_P6L {
            nCmd : 1,
            nPar : 0,
            A9F  : [0.0;9],
        };

        loop {
            let currenttime = Instant::now();
            let dt = currenttime - lasttime;
            if dt.as_millis() >= nTms { // ms
                // 1) time elapsed
                let t = (currenttime - otime).as_secs_f64();
                // 2) DO Work here!
                match nCmd
                {
                    0 => P6L.A9F = M1Data::GetData(t),
                    1 => P6L.A9F = M2Data::GetData(t),
                    2 => P6L.A9F = M3Data::GetData(t),
                    3 => P6L.A9F = M4Data::GetData(t),
                    _ => { println!("No such case. Only support 0-3."); break;}
                }
                
                

                //let (_, buff, _) = unsafe { A6L.align_to::<u8>() };
                let buf: &[u8];
                unsafe {
                    let pp = &P6L;
                    let pu8 = (pp as *const TO_P6L) as *const u8;
                    buf = std::slice::from_raw_parts(pu8, std::mem::size_of_val(pp));
                }
                
                let nsend = match udpclient.SendTo(buf,remote.as_str()) {
                    Ok(n) => n,
                    Err(e) => 0,
                };
                // _ = udpclient.SendTo(buf, "192.168.8.148:2023").expect(&remote);

                // 3) update time
                lasttime = currenttime;
                // 4) display
                if verbose {
                    println!("ts = {:5.2}, dt = {}ms, ({} Bytes)", t, dt.as_millis(), nsend);
                }
            }
        }
    });

}


fn main() {
    // let X = & [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];
    // let Y = & [10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];
    // let y = MData::getdata(3.0, X, Y);
 
    // let y = M4Data::GetData(3.0);
    // println!("y={:?}", y);

    //let param: MDP = argh::from_env();

    // test mdata
    test_p6l(&std::env::args().collect());

    // pause: any key stroke ...
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();

}
