use rdev::{grab, windows::grab::stop_grab, listen, simulate, Button, Event, EventType, Key, SimulateError};
use std::{fs::File, io::{BufWriter, Write}, process::exit};

#[derive(Clone, Copy)]
pub enum StoredMacroElement {
    KeyElement{key: char, down: bool},
    MouseButtonElement{button: i8, down: bool},
    MouseMoveElement{x: f64, y: f64},
    DelayElement{hours: i64, minutes: i64, seconds: i64, milliseconds: i64},
}

pub fn play_macro(sequence: Vec<StoredMacroElement>)
{
    for element in sequence
    {
        match element
        {
            StoredMacroElement::KeyElement { key: _, down: _ } => println!("a"),
            _ => println!("somethings gone wrong"),
        }

    }
}

//use serde::{Serialize, Deserialize};

/*
#[derive(Deserialize, Serialize)]
struct SequenceElement
{
    key : Option<Key>,
    button : Option<Button>,
    delay : Option<i64>,
}

pub fn write_to_json(macro_to_write : Vec<SequenceElement> )
{
    let file = File::create("macros.json").unwrap(); 
    let mut writer = BufWriter::new(&file);
    let _ = serde_json::to_writer_pretty(&mut writer, &macro_to_write);
    let _ = writer.flush();

}*/

/*
fn useful(){

    //AppData::run(Settings::default());

    /*
    let main_window = WindowDesc::new(ui_builder()).title("Test Window");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppData{num: 3})
        .unwrap();
    */

    let mut vec : Vec<SequenceElement> = Vec::new();

    let one = SequenceElement   
    {
        key: Some(Key::KeyA),
        button: None,
        delay: None,
    };

    let two = SequenceElement   
    {
        key: None,
        button: Some(Button::Right),
        delay: None,
    };

    let three = SequenceElement   
    {
        key: None,
        button: None,
        delay: Some(10),
    };

    vec.push(one);
    vec.push(two);
    vec.push(three);

    write_to_json(vec);

    /* if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    } */

    /*
    if let Err(error) = grab(callback2) {
        println!("Error: {:?}", error)
    }*/

    //stop_grab();
    
    /*
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }*/

}*/

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("could not send event: {:?}", event_type);
        }
    }
    //let delay = time::Duration::from_millis(20);
    //thread::sleep(delay);
}

/*
fn read_from_json() -> Vec<SequenceElement>
{
    let mut to_return : Vec<SequenceElement> = Vec::new();

    return to_return;
}*/

fn complete_apple ()
{
    send(&EventType::KeyPress(Key::KeyP));
    send(&EventType::KeyRelease(Key::KeyP));

    send(&EventType::KeyPress(Key::KeyP));
    send(&EventType::KeyRelease(Key::KeyP));

    send(&EventType::KeyPress(Key::KeyL));
    send(&EventType::KeyRelease(Key::KeyL));

    send(&EventType::KeyPress(Key::KeyE));
    send(&EventType::KeyRelease(Key::KeyE));

}

pub fn callback(event: Event)
{
    //println!("My callback {:?}", event);

    match event.event_type
    {
        EventType::ButtonPress(Button::Left) =>
        {
            println!("lmb pressed");
        },
        EventType::KeyPress(Key::KeyZ) => exit(0),
        EventType::KeyPress(Key::KeyP) => stop_grab(),
        EventType::KeyRelease(Key::KeyA) => complete_apple(),
        _ => 
        {
            println!("other event");
        },
    }

}

pub fn callback2(event : Event) -> Option<Event>
{
    match event.event_type
    {
        EventType::ButtonPress(Button::Left) =>
        {
            println!("lmb pressed");
            Some(event)
        },
        EventType::KeyPress(Key::KeyZ) => exit(0),
        EventType::KeyPress(Key::KeyP) => 
        {
            stop_grab();
            Some(event)
        },
        EventType::KeyPress(Key::KeyL) =>
        {
            println!("eating the L press yum yum");
            None
        },
        //EventType::KeyRelease(Key::KeyA) => complete_apple(),
        //EventType::KeyPress(Key::KeyB) => return(Event);
        _ => 
        {
            //println!("other event");
            Some(event)
        },
    }
}