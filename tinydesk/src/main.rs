#![allow(non_snake_case)]

mod backend;
use backend::play_macro;
use backend::StoredMacroElement;
use std::thread;
use dioxus::desktop::tao::platform::windows::EventLoopBuilderExtWindows;
use dioxus::desktop::tao::window;
use dioxus::desktop::window;
use thread_priority::*;

use dioxus::desktop::LogicalSize;
use dioxus::desktop::WindowBuilder;
use rdev::listen as keylisten;
use rdev::Event;
use backend::callback2;
use backend::callback;
use rdev::grab;
//use backend::callback2;

use dioxus::prelude::*;
use tracing::Level;

use dioxus::prelude::Key::Character;

//use manganis::*;

fn main() {

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    LaunchBuilder::desktop().with_cfg(CreateConfig()).launch(App);
}

fn CreateConfig() -> dioxus::desktop::Config
{
    dioxus::desktop::Config::default()
        .with_window(CreateWindow())
        .with_close_behaviour(dioxus::desktop::WindowCloseBehaviour::LastWindowExitsApp)
        .with_disable_context_menu(true)
        .with_menu(None)
}

fn CreateWindow() -> WindowBuilder
{
    WindowBuilder::new()
        .with_always_on_top(false)
        .with_min_inner_size(LogicalSize::new(800, 500))
        .with_title("Tiny Desk")
        .with_content_protection(false)
        .with_focused(false)

}

//carries the id of the page to display (1,2, or 3)
#[derive(Clone, Copy)]
struct ContentId(i8);

struct StoredMacros(
    Vec<StoredMacroElement>
);


static mut storedMacro : Vec<StoredMacroElement> = Vec::<StoredMacroElement>::new();

#[component]
fn App() -> Element {
    
    //use_context_provider(|| Signal::new(MacroAddMenuLoc(0f64, 0f64)));
    //let mut shid = use_signal(|| Vec::<StoredMacroElement>::new());

    let thing1 = StoredMacroElement::KeyElement{key: 'a', down: true};
    let thing2 = StoredMacroElement::KeyElement{key: 'b', down: false};
    let thing3 = StoredMacroElement::KeyElement{key: 'c', down: false};

    unsafe
    {
        storedMacro.push(thing1);
        storedMacro.push(thing2);
        storedMacro.push(thing3);

        play_macro(storedMacro.clone());
    }
    

    //shid.write().push(thing1);
    //shid.write().push(thing2);
    //shid.write().push(thing3);

    

    let content_id = use_signal(|| ContentId(1));

    rsx!
    {
        link { rel: "stylesheet", href: "main.css" }
        {SideBar(content_id)},
        div { class: "main", {ContentManager(content_id().0)} }
    }

}

fn SideBar(mut content_id: Signal<ContentId>) -> Element
{
    rsx!    
    {
        div { class: "sidenav",
            button { onclick: move |_| content_id.write().0 = 1, "Home" }
            button { onclick: move |_| content_id.write().0 = 2, "Macros" }
            button { onclick: move |_| content_id.write().0 = 3, "More" }
        }
    }
}

fn ContentManager(requested_content: i8,) -> Element
{
    if requested_content == 1
    {   
        Home()
    }
    else if requested_content == 2
    {   
        Macros()
    }
    else if requested_content == 3
    {
        More()
    }
    else
    {
        rsx!
        { "invalid id for content request" }
    }
}

fn Home() -> Element
{
    rsx!
    { "this is home" }
}

fn Macros() -> Element
{   
    let mut macro_elements = use_signal(|| Vec::<Element>::new());
    if macro_elements.len() == 0
    {
        macro_elements.push(MacrosAddMenu(macro_elements));
    }

    rsx!
    {
        div { class: "macro_page",
            h1 { "Macros" }
            br {}
            div { class: "macro_elements",
                for element in macro_elements() {
                    {element}
                }
            }
            br {}
            "this is the macros page"
            br {}
            button { onclick: |_| DoThing(), "begin listen" }
        }
    }

    //const BTNIMAGE: manganis::ImageAsset = manganis::mg!(image("./assets/enchanting tree.png"));

}

fn MacrosAddMenu(mut macro_elements: Signal<Vec::<Element>>) -> Element
{   
    rsx!
    {
        div { class: "add_macro_menu",
            button { onclick: move |_| macro_elements.insert(macro_elements.len() - 1, MacroDelay()),
                "Add Delay"
            }
            button { onclick: move |_| macro_elements.insert(macro_elements.len() - 1, MacroKey()),
                "Add Key"
            }
            button { onclick: move |_| macro_elements.insert(macro_elements.len() - 1, MacroMacro()),
                "Add Macro"
            }
        }
    }
}

fn MacroDelay() -> Element
{
    rsx!
    {
        div { class: "macro_element",
            div { class: "delay_element",
                label { "h:" }
                input {
                    class: "delay_element_input",
                    name: "hours",
                    r#type: "number",
                    min: "0",
                    value: 0
                }
            }
            div { class: "delay_element",
                label { r#for: "minutes", "m:" }
                input {
                    class: "delay_element_input",
                    name: "minutes",
                    r#type: "number",
                    min: "0",
                    value: 0
                }
            }
            div { class: "delay_element",
                label { r#for: "seconds", "s:" }
                input {
                    class: "delay_element_input",
                    name: "seconds",
                    r#type: "number",
                    min: "0",
                    value: 0
                }
            }
            div { class: "delay_element",
                label { r#for: "milliseconds", "ms:" }
                input {
                    class: "delay_element_input",
                    name: "milliseconds",
                    r#type: "number",
                    min: "0",
                    value: 0
                }
            }
        }
    }
}

fn MacroKey() -> Element
{   
    let mut thing: Key = Character("a".to_string());
    let set_key = use_signal(|| format!("keycode: {thing}"));

    //IDEA have keyboard pop up and you can click on which button you want
    //BETTER IDEA just have it so you can hover over and press a key to have it register

    rsx!
    {
        div { class: "key_element",
            select {
                option { value: "a", "a" }
                option { value: "b", "b" }
                option { value: "c", "c" }
                option { value: "d", "d" }
                option { value: "e", "e" }
                option { value: "f", "f" }
                option { value: "g", "g" }
                option { value: "h", "h" }
                option { value: "i", "i" }
                option { value: "j", "j" }
                option { value: "k", "k" }
                option { value: "l", "l" }
                option { value: "m", "m" }
                option { value: "n", "n" }
                option { value: "o", "o" }
                option { value: "p", "p" }
                option { value: "q", "q" }
                option { value: "r", "r" }
                option { value: "s", "s" }
                option { value: "t", "t" }
                option { value: "u", "u" }
                option { value: "v", "v" }
                option { value: "w", "w" }
                option { value: "x", "x" }
                option { value: "y", "y" }
                option { value: "z", "z" }
            }
        }
    }

}

fn DoThing()
{
    thread::spawn( move || {
        if let Err(error) = grab(callback2) {
            println!("Error: {:?}", error)
        }
    });
}

fn MacroMacro() -> Element
{   
    rsx!
    {
        div { "some macro" }
    }
}

fn More() -> Element
{
    rsx!
    { "put some stuff here someday" }
}


