// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use egui::Ui;
use egui::TopBottomPanel;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::tabs::auth::AuthType;
use crate::request::Request;


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
enum CollectionTab {
    Auth
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct CollectionData {
    pub selected_auth: AuthType,
    pub credentials: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Collection {
    pub uuid: Uuid,
    pub name: String,
    pub requests: Vec<Request>,
    
    data: Rc<RefCell<CollectionData>>,
    
    tab: CollectionTab,
}

impl Collection {
    // TODO: Implement auth and variables
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            requests: vec![],
            data: Rc::new(RefCell::new(Default::default())),
            tab: CollectionTab::Auth,
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        TopBottomPanel::top(format!("collection_top_panel_{}", self.uuid)).resizable(true).show_inside(ui, |ui| {
            ui.heading(&self.name);
            ui.add_space(10.);
            
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, CollectionTab::Auth, "Authorization");
            });
            
            match &self.tab {
                CollectionTab::Auth => {
                }
            }
            ui.add_space(10.)
        });
    }
    
    pub fn create_request(&mut self) {
        self.requests.push(Request::new(Rc::clone(&self.data)))
    }
}

