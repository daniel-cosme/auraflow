use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{User, Patient, Appointment, AppointmentStatus};

// Mock API base URL
const API_BASE_URL: &str = "/api";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

// Mock login function
pub async fn login(email: &str, password: &str) -> Result<User, String> {
    // Simulate API delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // Mock validation
    if email == "admin@example.com" && password == "password123" {
        Ok(User {
            id: 1,
            name: "Admin User".to_string(),
            email: email.to_string(),
        })
    } else {
        Err("Credenciais inválidas".to_string())
    }
}

// Mock function to get patients
pub async fn get_patients() -> Result<Vec<Patient>, String> {
    // Simulate API delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // Return mock patients
    Ok(vec![
        Patient {
            id: 1,
            name: "Maria Silva".to_string(),
            phone: "(11) 99999-8888".to_string(),
            email: "maria.silva@example.com".to_string(),
            birth_date: "1985-05-15".to_string(),
            cpf: "123.456.789-00".to_string(),
            address: Some("Rua Principal, 123 - São Paulo/SP".to_string()),
        },
        Patient {
            id: 2,
            name: "João Oliveira".to_string(),
            phone: "(11) 98888-7777".to_string(),
            email: "joao.oliveira@example.com".to_string(),
            birth_date: "1990-10-22".to_string(),
            cpf: "987.654.321-00".to_string(),
            address: Some("Av. Paulista, 1000 - São Paulo/SP".to_string()),
        },
        Patient {
            id: 3,
            name: "Ana Costa".to_string(),
            phone: "(11) 97777-6666".to_string(),
            email: "ana.costa@example.com".to_string(),
            birth_date: "1978-03-10".to_string(),
            cpf: "456.789.123-00".to_string(),
            address: Some("Rua Augusta, 500 - São Paulo/SP".to_string()),
        },
    ])
}

// Mock function to get appointments
pub async fn get_appointments() -> Result<Vec<Appointment>, String> {
    // Simulate API delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // Return mock appointments
    Ok(vec![
        Appointment {
            id: 1,
            patient_id: 1,
            patient_name: "Maria Silva".to_string(),
            date_time: "2023-06-15T09:00:00".to_string(),
            duration: 60,
            status: AppointmentStatus::Scheduled,
            notes: Some("Primeira consulta".to_string()),
        },
        Appointment {
            id: 2,
            patient_id: 2,
            patient_name: "João Oliveira".to_string(),
            date_time: "2023-06-15T10:30:00".to_string(),
            duration: 45,
            status: AppointmentStatus::Confirmed,
            notes: None,
        },
        Appointment {
            id: 3,
            patient_id: 3,
            patient_name: "Ana Costa".to_string(),
            date_time: "2023-06-15T14:00:00".to_string(),
            duration: 90,
            status: AppointmentStatus::Scheduled,
            notes: Some("Limpeza e revisão".to_string()),
        },
        Appointment {
            id: 4,
            patient_id: 1,
            patient_name: "Maria Silva".to_string(),
            date_time: "2023-06-16T11:00:00".to_string(),
            duration: 30,
            status: AppointmentStatus::Completed,
            notes: Some("Retorno".to_string()),
        },
        Appointment {
            id: 5,
            patient_id: 2,
            patient_name: "João Oliveira".to_string(),
            date_time: "2023-06-17T15:30:00".to_string(),
            duration: 60,
            status: AppointmentStatus::Canceled,
            notes: Some("O paciente cancelou".to_string()),
        },
    ])
}

// Mock function to create appointment
pub async fn create_appointment(appointment: Appointment) -> Result<Appointment, String> {
    // Simulate API delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // In a real implementation, we would send the appointment to the server
    // Here we just return the appointment as if it was saved
    Ok(appointment)
}

// Mock function to update appointment
pub async fn update_appointment(appointment: Appointment) -> Result<Appointment, String> {
    // Simulate API delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // In a real implementation, we would send the updated appointment to the server
    // Here we just return the appointment as if it was updated
    Ok(appointment)
}

// Mock function to delete appointment
pub async fn delete_appointment(id: u32) -> Result<(), String> {
    // Simulate API delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // In a real implementation, we would send a delete request to the server
    // Here we just return Ok as if the appointment was deleted
    Ok(())
}

// Mock function to get a specific patient
pub async fn get_patient(id: u32) -> Result<Patient, String> {
    // Simulate API delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // Return a mock patient based on ID
    match id {
        1 => Ok(Patient {
            id: 1,
            name: "Maria Silva".to_string(),
            phone: "(11) 99999-8888".to_string(),
            email: "maria.silva@example.com".to_string(),
            birth_date: "1985-05-15".to_string(),
            cpf: "123.456.789-00".to_string(),
            address: Some("Rua Principal, 123 - São Paulo/SP".to_string()),
        }),
        2 => Ok(Patient {
            id: 2,
            name: "João Oliveira".to_string(),
            phone: "(11) 98888-7777".to_string(),
            email: "joao.oliveira@example.com".to_string(),
            birth_date: "1990-10-22".to_string(),
            cpf: "987.654.321-00".to_string(),
            address: Some("Av. Paulista, 1000 - São Paulo/SP".to_string()),
        }),
        3 => Ok(Patient {
            id: 3,
            name: "Ana Costa".to_string(),
            phone: "(11) 97777-6666".to_string(),
            email: "ana.costa@example.com".to_string(),
            birth_date: "1978-03-10".to_string(),
            cpf: "456.789.123-00".to_string(),
            address: Some("Rua Augusta, 500 - São Paulo/SP".to_string()),
        }),
        _ => Err("Paciente não encontrado".to_string()),
    }
}