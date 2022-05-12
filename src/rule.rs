use std::{
  collections::HashSet,
  time::{
    Duration,
    Instant,
  },
};

use anyhow::{
  anyhow, 
  Result
};

use rdev::{
  self,
  Event,
  Key as RKey
};


use crate::code::keycode;

pub trait RuleAction {
  fn change(&mut self, event: &Event, state: &HashSet<i32>) -> bool;
  fn focus(&self) -> &str;
}

#[derive(Debug)]
pub struct Rule {
  keys: Vec<i32>,
  focus: String,
}

impl Rule {
  pub fn new(keys: Vec<RKey>, focus: &str) -> Result<Self> {
      Ok(Self {
          keys: keys_to_codes(&keys)?,
          focus: focus.to_owned(),
      })
  }
}

impl RuleAction for Rule {
  fn change(&mut self, _event: &Event, state: &HashSet<i32>) -> bool {
      self.keys.iter().all(|key| state.get(key).is_some())
  }
  fn focus(&self) -> &str {
      &self.focus
  }
}

fn keys_to_codes(keys: &Vec<RKey>) -> Result<Vec<i32>> {
  keys
  .iter()
  .map(|key| keycode(*key).ok_or(anyhow!("rule keycode {:?}", key)))
  .collect::<Result<_>>()
}