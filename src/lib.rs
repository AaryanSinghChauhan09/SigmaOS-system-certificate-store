// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-system-certificate-store: system certificate store component model.
//!
//! Real implementation: stores, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;
pub mod sigmacertificatemanager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Store {
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
pub enum SystemCertificateStoreError {
    EmptyStoreName,
    DuplicateStore(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemCertificateStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyStoreName => write!(f, "store name must not be empty"),
            Self::DuplicateStore(id) => write!(f, "duplicate store {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemCertificateStoreError {}

pub struct SystemCertificateStore {
    pub name: String,
    stores: BTreeMap<String, Store>,
    events: BTreeMap<String, Event>,
}

impl SystemCertificateStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            stores: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_store(&mut self, item: Store) -> Result<(), SystemCertificateStoreError> {
        if item.name.trim().is_empty() {
            return Err(SystemCertificateStoreError::EmptyStoreName);
        }
        let id = item.id.clone();
        if self.stores.contains_key(&id) {
            return Err(SystemCertificateStoreError::DuplicateStore(id.clone()));
        }
        self.stores.insert(id, item);
        Ok(())
    }

    pub fn store_count(&self) -> usize {
        self.stores.len()
    }

    pub fn get_store(&self, id: &str) -> Option<&Store> {
        self.stores.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemCertificateStoreError> {
        if event.subject.trim().is_empty() {
            return Err(SystemCertificateStoreError::EmptyEventSubject);
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

    fn sample_store(id: &str, name: &str) -> Store {
        Store {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemCertificateStore::new("SystemCertificateStore");
        assert_eq!(m.name, "SystemCertificateStore");
        assert_eq!(m.store_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_store() {
        let mut m = SystemCertificateStore::new("SystemCertificateStore");
        m.add_store(sample_store("a1", "Alpha")).unwrap();
        assert_eq!(m.store_count(), 1);
        assert!(m.get_store("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemCertificateStore::new("SystemCertificateStore");
        assert!(m.add_store(sample_store("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemCertificateStore::new("SystemCertificateStore");
        m.add_store(sample_store("x", "One")).unwrap();
        assert!(m.add_store(sample_store("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemCertificateStore::new("SystemCertificateStore");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
