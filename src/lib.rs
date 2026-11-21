// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-event: event component model.
//!
//! Real implementation: event_component_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

/// System event management module.
pub mod sigmaevent;

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventComponentItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventComponentError {
    EmptyEventComponentItemName,
    DuplicateEventComponentItem(String),
    EmptyEventSubject,
}

impl fmt::Display for EventComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEventComponentItemName => write!(f, "event_component_item name must not be empty"),
            Self::DuplicateEventComponentItem(id) => write!(f, "duplicate event_component_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EventComponentError {}

pub struct EventComponent {
    pub name: String,
    event_component_items: BTreeMap<String, EventComponentItem>,
    events: BTreeMap<String, Event>,
}

impl EventComponent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            event_component_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_event_component_item(&mut self, item: EventComponentItem) -> Result<(), EventComponentError> {
        if item.name.trim().is_empty() {
            return Err(EventComponentError::EmptyEventComponentItemName);
        }
        let id = item.id.clone();
        if self.event_component_items.contains_key(&id) {
            return Err(EventComponentError::DuplicateEventComponentItem(id.clone()));
        }
        self.event_component_items.insert(id, item);
        Ok(())
    }

    pub fn event_component_item_count(&self) -> usize {
        self.event_component_items.len()
    }

    pub fn get_event_component_item(&self, id: &str) -> Option<&EventComponentItem> {
        self.event_component_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EventComponentError> {
        if event.subject.trim().is_empty() {
            return Err(EventComponentError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_event_component_item(id: &str, name: &str) -> EventComponentItem {
        EventComponentItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = EventComponent::new("EventComponent");
        assert_eq!(m.name, "EventComponent");
        assert_eq!(m.event_component_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_event_component_item() {
        let mut m = EventComponent::new("EventComponent");
        m.add_event_component_item(sample_event_component_item("a1", "Alpha")).unwrap();
        assert_eq!(m.event_component_item_count(), 1);
        assert!(m.get_event_component_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = EventComponent::new("EventComponent");
        assert!(m.add_event_component_item(sample_event_component_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = EventComponent::new("EventComponent");
        m.add_event_component_item(sample_event_component_item("x", "One")).unwrap();
        assert!(m.add_event_component_item(sample_event_component_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = EventComponent::new("EventComponent");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
