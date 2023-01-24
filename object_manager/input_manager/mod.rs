use std::{collections::HashMap, ops::IndexMut};

use nalgebra::Vector2;

#[derive(Debug, Clone)]
pub enum KeyState {
    Pressed,
    Held,
    Released,
    NotPressed,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LEFT,
    RIGHT,
    UP,
    DOWN,
    MOUSELEFT,
    MOUSERIGHT,
}

#[derive(Default)]
pub struct MouseInfo {
    pub position: Vector2<f64>,
    pub delta: Vector2<f64>,
    pub hasmoved: bool,
}

pub struct InputManager {
    buttons: HashMap<Key, KeyState>,
    pub mouse: MouseInfo,
}

impl InputManager {
    pub fn new_default() -> Self {
        InputManager {
            buttons: HashMap::new(),
            mouse: MouseInfo::default(),
        }
    }
    pub fn update(&mut self) {
        let delta_length = self.mouse.delta.norm();

        for (key, value) in self.buttons.clone().iter() {
            match value {
                KeyState::Pressed => {
                    self.buttons.insert(key.clone(), KeyState::Held);
                }
                _ => {
                    ();
                }
            }
            match value {
                KeyState::Released => {
                    self.buttons.insert(key.clone(), KeyState::NotPressed);
                }
                _ => {
                    ();
                }
            }
        }
        self.mouse.hasmoved = false;
    }
    pub fn press_key(&mut self, key: Key) {
        if self.buttons.contains_key(&key) {
            let state_current = self.buttons.get(&key).unwrap();
            match state_current {
                KeyState::NotPressed | KeyState::Released => {
                    self.buttons.insert(key, KeyState::Pressed);
                }
                _ => (),
            }
        } else {
            self.buttons.insert(key.clone(), KeyState::Pressed);
            //            println!("key created -> {:?}",key);
        }
    }
    pub fn release_key(&mut self, key: Key) {
        if self.buttons.contains_key(&key) {
            let state_current = self.buttons.get(&key).unwrap();
            match state_current {
                KeyState::Pressed | KeyState::Held => {
                    self.buttons.insert(key, KeyState::Released);
                }
                _ => (),
            }
        }
    }
    pub fn get_key_pressed(&self, key: Key) -> bool {
        match self.buttons.get(&key) {
            Some(key_state) => match key_state {
                KeyState::Pressed => true,
                _ => false,
            },
            None => false,
        }
    }
    pub fn get_key_released(&self, key: Key) -> bool {
        match self.buttons.get(&key) {
            Some(key_state) => match key_state {
                KeyState::Released => true,
                _ => false,
            },
            None => false,
        }
    }
    pub fn get_key_held(&self, key: Key) -> bool {
        match self.buttons.get(&key) {
            Some(key_state) => match key_state {
                KeyState::Held => true,
                _ => false,
            },
            None => false,
        }
    }
}
