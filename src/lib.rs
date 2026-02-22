use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "hydrate")]
pub mod app;

// Shared types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub birth_date: String,
    pub cpf: String,
    pub address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Appointment {
    pub id: u32,
    pub patient_id: u32,
    pub patient_name: String,
    pub date_time: String, // ISO 8601 format
    pub duration: u32,     // in minutes
    pub status: AppointmentStatus,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppointmentStatus {
    Scheduled,
    Confirmed,
    Completed,
    Canceled,
}

// Global state management
#[derive(Clone)]
pub struct AppState {
    pub current_user: RwSignal<Option<User>>,
    pub patients: RwSignal<Vec<Patient>>,
    pub appointments: RwSignal<Vec<Appointment>>,
}

// Initialize global state
pub fn create_app_state() -> AppState {
    AppState {
        current_user: create_rw_signal(None),
        patients: create_rw_signal(vec![]),
        appointments: create_rw_signal(vec![]),
    }
}