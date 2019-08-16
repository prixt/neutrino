use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

pub struct Combo {
    name: String,
    choices: Vec<String>,
    selected: u32,
    opened: bool,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl Combo {
    pub fn new(name: &str) -> Self {
        Combo {
            name: name.to_string(),
            choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
            selected: 0,
            opened: false,
            listener: None,
            observer: None,
        }
    }

    pub fn choices(self, choices: Vec<&str>) -> Self {
        Combo {
            name: self.name,
            choices: choices
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
            selected: self.selected,
            opened: self.opened,
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn selected(self, selected: u32) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: selected,
            opened: self.opened,
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn opened(self, opened: bool) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            opened: opened,
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            opened: self.opened,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    pub fn observer(self, observer: Box<Observer>) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            opened: self.opened,
            listener: self.listener,
            observer: Some(observer),
        }
    }
}

impl Widget for Combo {
    fn eval(&self) -> String {
        let mut s = format!(
            r#"<div class="combo"><div onclick="{}" class="combo-button">{}</div>"#,
            Event::js("click", &self.name, "'-1'"),
            self.choices[self.selected as usize]
        );
        if self.opened {
            s.push_str(r#"<div class="combo-choices">"#);
            for (i, choice) in self.choices.iter().enumerate() {
                s.push_str(&format!(
                    r#"<div class="combo-choice" onclick="{}">{}</div>"#,
                    Event::js("click", &self.name, &format!("'{}'", i)),
                    choice
                ));
            }
            s.push_str(r#"</div>"#);
        }
        s.push_str("</div>");
        s
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "click" {
                self.opened = !self.opened;
                let selected = event.value.parse::<i32>().unwrap();
                if selected > -1 {
                    self.selected = selected as u32;
                }
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_click();
                    }
                }
            }
        };
    }

    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(_observer) => {}
        }
    }
}
