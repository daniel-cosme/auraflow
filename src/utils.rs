use leptos::*;

// Utility function to format date
pub fn format_date(date_str: &str) -> String {
    // Parse the date string (assuming ISO 8601 format)
    // For simplicity, we'll just return the date part
    if let Some(date_part) = date_str.split('T').next() {
        date_part.replace("-", "/")
    } else {
        date_str.to_string()
    }
}

// Utility function to format datetime
pub fn format_datetime(datetime_str: &str) -> String {
    // Parse the datetime string (assuming ISO 8601 format)
    let parts: Vec<&str> = datetime_str.split('T').collect();
    if parts.len() == 2 {
        let date_part = parts[0].replace("-", "/");
        let time_part = parts[1].split(':').take(2).collect::<Vec<&str>>().join(":");
        format!("{} às {}", date_part, time_part)
    } else {
        datetime_str.to_string()
    }
}

// Utility function to generate initials from name
pub fn get_initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    let mut initials = String::new();
    
    for word in words.iter().take(2) {
        if let Some(first_char) = word.chars().next() {
            initials.push(first_char.to_uppercase().next().unwrap_or(first_char));
        }
    }
    
    initials
}

// Utility function to validate CPF
pub fn validate_cpf(cpf: &str) -> bool {
    // Remove non-digit characters
    let digits: String = cpf.chars().filter(|c| c.is_digit(10)).collect();
    
    if digits.len() != 11 {
        return false;
    }
    
    // Check if all digits are the same
    if digits.chars().all(|c| c == digits.chars().next().unwrap()) {
        return false;
    }
    
    // Calculate first digit
    let mut sum = 0;
    for i in 0..9 {
        if let Some(digit) = digits.chars().nth(i).and_then(|c| c.to_digit(10)) {
            sum += digit * (10 - i);
        }
    }
    
    let first_digit = (sum * 10) % 11;
    let first_digit = if first_digit == 10 { 0 } else { first_digit };
    
    // Calculate second digit
    let mut sum = 0;
    for i in 0..10 {
        let multiplier = if i == 0 { 11 } else { 12 - i };
        if let Some(digit) = digits.chars().nth(i).and_then(|c| c.to_digit(10)) {
            sum += digit * multiplier;
        }
    }
    
    let second_digit = (sum * 10) % 11;
    let second_digit = if second_digit == 10 { 0 } else { second_digit };
    
    // Validate calculated digits
    let expected_digits = format!("{}{}", first_digit, second_digit);
    let actual_digits = &digits[9..11];
    
    expected_digits == actual_digits
}

// Utility function to validate email
pub fn validate_email(email: &str) -> bool {
    // Simple email validation regex
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

// Reactive utility to check if user is authenticated
#[derive(Copy, Clone)]
pub struct AuthContext {
    pub is_authenticated: Signal<bool>,
    pub user_name: Signal<String>,
}

// Provides authentication context to the app
pub fn provide_auth_context() {
    let app_state = use_context::<crate::AppState>().expect("AppState must be provided");
    
    let is_authenticated = Signal::derive(move || {
        app_state.current_user.with(|user| user.is_some())
    });
    
    let user_name = Signal::derive(move || {
        app_state.current_user.with(|user| {
            user.as_ref()
                .map(|u| u.name.clone())
                .unwrap_or_else(|| "Usuário".to_string())
        })
    });
    
    provide_context(AuthContext {
        is_authenticated,
        user_name,
    });
}

// Wrapper for protected routes
#[component]
pub fn ProtectedRoute(
    children: Children,
) -> impl IntoView {
    let auth_context = use_context::<AuthContext>();
    
    if let Some(auth) = auth_context {
        view! {
            <Show
                when=move || auth.is_authenticated.get()
                fallback=|| view! { <Redirect path="/login"/> }
            >
                {children()}
            </Show>
        }
    } else {
        // If no auth context, redirect to login
        view! { <Redirect path="/login"/> }
    }
}

// Add this to make the regex available
#[cfg(feature = "hydrate")]
use regex;
#[cfg(feature = "hydrate")]
use leptos_router::{Redirect, use_context};