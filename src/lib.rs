#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate oxide;

extern crate arboard;
use arboard::{Clipboard, ImageData};

use code::keycode;
use rdev::{
  self,
  Key,
  EventType,
  Event
};

use std::{
  collections::HashSet,
  process::{
    Child,
    Command,
    Stdio,
  },
  sync::Mutex
};

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;

pub mod code;
pub mod rule;

mod env;
pub use env::Env;

use rule::{Rule, RuleAction};

use appwrite;

static PRESSED_STATE:Lazy<Mutex<HashSet<i32>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub async fn run() {
  let rules:Vec<Box<dyn RuleAction>> = vec![
    Box::new(
      Rule::new(
        vec![Key::MetaLeft, Key::ShiftLeft, Key::KeyA],
        "upload-clip",
      ).unwrap()
    ),
  ];

  let rules = Mutex::new(rules);

  rdev::grab(move |event| {
    event_check(event, |event| match event.event_type {
      EventType::KeyPress(key) => {
        let code = keycode(key).ok_or(anyhow!("keycode press: {:?}", key)).unwrap();

        PRESSED_STATE.lock().unwrap().insert(code);
        let mut captured = false;

        for rule in rules.lock().unwrap().iter_mut() {
          if rule.change(event, &PRESSED_STATE.lock().unwrap()) {
            process_clipboard();
            captured = true;
          }
        }

        Ok(captured)
      },
      EventType::KeyRelease(key) => {
        let code = keycode(key).ok_or(anyhow!("keycode released: {:?}", key)).unwrap();

        PRESSED_STATE.lock().unwrap().remove(&code);
        Ok(false)
      },
      _ => Ok(false),
    })
  }).map_err(|e| 
    anyhow!("Listen error: {:?}", e)
  ).unwrap();
}

fn event_check<F>(event:Event, callback:F) -> Option<Event> 
  where F: FnOnce(&Event) -> Result<bool> {

    match callback(&event) {
      Ok(true) => None,
      Ok(false) => Some(event),
      Err(err) => {
        console_error!("Error: {}", err);
        Some(event)
      }
    }
}

#[derive(Debug)]
struct ClipboardData<'a> {
  text: Option<String>,
  image: Option<ImageData<'a>>,
}


fn process_clipboard() {
  console_debug!("process clipboard");

  let mut clipboard = Clipboard::new().unwrap();
  let mut data = ClipboardData {
    text: None,
    image: None,
  };

  match clipboard.get_text() {
    Ok(content) => data.text = Some(content),
    Err(_) => data.text = None,
  };

  match clipboard.get_image() {
    Ok(content) => data.image = Some(content),
    Err(_) => data.image = None,
  };

 
  let env = Env::default();
  let mut client = appwrite::client::Client::new();
  client.set_project(&env.project_id);
  client.set_endpoint(&env.api_endpoint);
  client.set_key(&env.api_key);
}

