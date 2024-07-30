use std::{mem::size_of, ptr::null_mut};

use windows::{core::Error, Win32::
    {Foundation::{GetLastError, LPARAM, LRESULT, WIN32_ERROR, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_SPACE},
        WindowsAndMessaging::{CallNextHookEx, GetMessageA, PeekMessageA, SetWindowsHookExA, HHOOK, HOOKPROC, KBDLLHOOKSTRUCT, MSG, PEEK_MESSAGE_REMOVE_TYPE, PM_NOREMOVE, PM_REMOVE, WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP}
    }
}};

enum MacroElementDataType
{
    Key,
    Button,
    Mouse,
    Delay,
}

union MacroElementData
{
    key: (VIRTUAL_KEY, bool),
    button: (VIRTUAL_KEY, bool),
    mouse: (f64, f64),
    delay: (i64, i64, i64, i64),
}

struct MacroElement
{
    data: MacroElementData,
    r#type: MacroElementDataType,
}


//static mut KEYBOARD_HOOK: Option<Result<HHOOK, Error>> = None;

fn main() {
    println!("Hello, world!");
    unsafe 
    {
        do_stuff();
    }
}

unsafe fn play_macro(sequence: &[MacroElement])
{
    for element in sequence
    {
        match element.r#type
        {
            MacroElementDataType::Key => send_key(element.data.key.0, element.data.key.1),
            MacroElementDataType::Button => println!("matched with button"), 
            MacroElementDataType::Mouse => println!("matched with mouse"), 
            MacroElementDataType::Delay => println!("matched with delay"), 
            _ => println!("very bad")
        }
    }
}

unsafe fn do_stuff()
{   
    //let hook_procedure: HOOKPROC;

    //create hooks
    //KEYBOARD_HOOK = Some(SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), None , 0));
    let keyboard_hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), None , 0);
    let mouse_hook = SetWindowsHookExA(WH_MOUSE_LL, Some(hook_callback), None , 0);

    //let hook_struct = KBDLLHOOKSTRUCT {vkCode: 41, }

    let mut message: MSG = Default::default();
    let msg_ptr: *mut MSG = &mut message;

    while GetMessageA(msg_ptr, None, 0, 0).into()
    {
        let _ = PeekMessageA(msg_ptr, None, 0, 0, PM_REMOVE);
    }

}


unsafe extern "system" fn hook_callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT
{   
    //CallNextHookEx(hook, n_code, w_param, l_param)
    //println!("doing something i think");

    let key_down: usize = usize::try_from(WM_KEYDOWN).unwrap();
    let sys_key_down: usize = usize::try_from(WM_SYSKEYDOWN).unwrap();
    let sys_key_up: usize = usize::try_from(WM_SYSKEYUP).unwrap();
    let key_up: usize = usize::try_from(WM_KEYUP).unwrap();
    //let l_button_down: usize = usize::try_from(WM_LBUTTONDOWN).unwrap();

    let l_button_down: usize = WM_LBUTTONDOWN as usize;

    if w_param == WPARAM(key_down)
    {
        println!("key pressed down");
    }
    else if w_param == WPARAM(sys_key_down)
    {
        println!("sys key pressed");
    }
    else if w_param == WPARAM(sys_key_up)
    {
        println!("sys key released");
    }
    else if w_param == WPARAM(key_up)
    {
        println!("key released");
    }
    else if w_param == WPARAM(l_button_down)
    {
        println!("lmb pressed");
    }

    //CallNextHookEx(hook_reference, n_code, w_param, l_param)
    LRESULT(0)
}

unsafe fn send_key(key: VIRTUAL_KEY, press: bool)
{
    let mut input: INPUT =
        INPUT { 
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { 
                ki: KEYBDINPUT { 
                    wVk: key, 
                    wScan: 0u16, 
                    dwFlags: KEYBD_EVENT_FLAGS::default(), 
                    time: 0, 
                    dwExtraInfo: usize::default(),
                }
            }
        };

    if !press
    {
        input.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS::default() | KEYEVENTF_KEYUP;
    }

    SendInput(&vec!{input}, size_of::<INPUT>() as _);
}