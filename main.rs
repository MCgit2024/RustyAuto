use std::{env::consts::OS, thread, time, borrow::BorrowMut};
mod ui;
use evdev::{Device,uinput::{VirtualDeviceBuilder,VirtualDevice}, AttributeSet, EventType, InputEvent, Key};



fn linux(event:String) -> std::io::Result<VirtualDevice> {
    let path = "/dev/input/event".to_string() + &event;
    let device = Device::open(path)?;
    
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::BTN_LEFT);
    let mut device = VirtualDeviceBuilder::new()?
        .name("fake mouse")
        .with_keys(&keys)?
        .build()
        .unwrap(); 
    return Ok(device);
}
fn windows(){

}
fn macos(){

}
fn main() {
    //ui::ui();
    let os:&str = std::env::consts::OS; 
    if os == "windows"{
        windows();
    }else if os == "linux" {
        //evtest to find event for user's mouse
        linux("6".to_string());
        struct Request {
            time: String,// for mili secs 500 clicks is max and 2 is least, for sec a 30 is max and least is 2,
            clicksPer: u16,
            clicking: bool
        }
        let userinput = Request{
            time: "min".to_string(),
            clicksPer: 20,
            clicking: true
        };
        let request = userinput;
        
        while request.clicking == true {
            if request.time == "sec"{
                let intervel = 1000 / request.clicksPer;
                let mut counter = 0;
                while request.clicksPer != counter{
                    InputEvent::new(EventType::KEY,0x110, 0);
                    thread::sleep(time::Duration::from_millis(intervel.into()));
                    counter += 1
                }
            }else if request.time == "min" {
                let intervel = 60 / request.clicksPer;
                let mut counter = 0;
                while request.clicksPer != counter{
                    InputEvent::new(EventType::KEY,0x110, 0);
                    thread::sleep(time::Duration::from_millis(intervel.into()));
                    counter += 1
                }
            }else if request.time == "hour" {
                let intervel = 60 / request.clicksPer;
                let mut counter = 0;
                while request.clicksPer != counter{
                    InputEvent::new(EventType::KEY,0x110, 0);
                    thread::sleep(time::Duration::from_millis(intervel.into()));
                    counter += 1
                }
            }
            
        }
    }else if os == "macos" {
        macos()
    }else {
        println!("Your os isnt supported");
        std::process::abort();
    }
}