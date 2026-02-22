use leptos::*;
use leptos_router::*;

use crate::{AppState, Patient, Appointment, AppointmentStatus};
use crate::services::api::{login, get_patients, get_appointments};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState must be provided");
    
    view! {
        <div class="flex h-screen bg-gray-50 dark:bg-gray-900">
            // Sidebar
            <aside class="hidden md:flex md:w-64 lg:w-72 flex-col border-r border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
                <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                    <h1 class="text-xl font-bold text-blue-600 dark:text-blue-400">OdontoSaaS</h1>
                </div>
                <nav class="flex-1 p-2 space-y-1">
                    <SidebarLink path="/dashboard" icon="home" label="Dashboard"/>
                    <SidebarLink path="/appointments" icon="calendar" label="Agenda"/>
                    <SidebarLink path="/patients" icon="users" label="Pacientes"/>
                    <SidebarLink path="/settings" icon="settings" label="Configurações"/>
                </nav>
                <div class="p-4 border-t border-gray-200 dark:border-gray-700">
                    <button 
                        class="w-full px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                        on:click=move |_| {
                            app_state.current_user.set(None);
                        }
                    >
                        Sair
                    </button>
                </div>
            </aside>
            
            // Mobile header
            <header class="md:hidden flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
                <h2 class="text-lg font-semibold text-gray-800 dark:text-white">OdontoSaaS</h2>
                <button class="p-2 rounded-md text-gray-700 dark:text-gray-300">
                    <MenuIcon/>
                </button>
            </header>
            
            // Main content
            <main class="flex-1 overflow-y-auto p-4 md:p-6">
                {children()}
            </main>
        </div>
    }
}

#[component]
fn SidebarLink(path: &'static str, icon: &'static str, label: &'static str) -> impl IntoView {
    let current_path = use_location().pathname;
    
    let is_active = move || {
        current_path.with(|loc| loc.starts_with(path))
    };
    
    view! {
        <A 
            href=path
            class=move || {
                if is_active() {
                    "flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md"
                } else {
                    "flex items-center px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700"
                }
            }
        >
            <span class="mr-3">{match icon {
                "home" => view!(<HomeIcon/>),
                "calendar" => view!(<CalendarIcon/>),
                "users" => view!(<UsersIcon/>),
                "settings" => view!(<SettingsIcon/>),
                _ => view!(<HomeIcon/>),
            }}</span>
            {label}
        </A>
    }
}

#[component]
fn HomeIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z" />
        </svg>
    }
}

#[component]
fn CalendarIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z" clip-rule="evenodd" />
        </svg>
    }
}

#[component]
fn UsersIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z" />
        </svg>
    }
}

#[component]
fn SettingsIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
        </svg>
    }
}

#[component]
fn MenuIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
    }
}

#[component]
pub fn Login() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState must be provided");
    
    let email = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let error = create_rw_signal(String::new());
    
    let submit = create_action(move |(email, password): &(String, String)| {
        let email = email.clone();
        let password = password.clone();
        
        async move {
            match login(&email, &password).await {
                Ok(user) => {
                    app_state.current_user.set(Some(user));
                    // Navigate to dashboard
                    leptos_router::use_navigate()("/dashboard", Default::default());
                }
                Err(e) => error.set(e.to_string()),
            }
        }
    });
    
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
            <div class="max-w-md w-full space-y-8 bg-white dark:bg-gray-800 p-8 rounded-xl shadow-md">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900 dark:text-white">
                        Acesso ao Sistema
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
                        Informe suas credenciais para continuar
                    </p>
                </div>
                <form class="mt-8 space-y-6" on:submit=move |ev| {
                    ev.prevent_default();
                    submit.dispatch((email.get(), password.get()));
                }>
                    <input type="hidden" name="remember" value="true"/>
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div>
                            <label for="email-address" class="sr-only">Endereço de e-mail</label>
                            <input
                                id="email-address"
                                name="email"
                                type="email"
                                required
                                placeholder="Email"
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-t-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm dark:bg-gray-700"
                                prop:value=email
                                on:input=move |ev| email.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label for="password" class="sr-only">Senha</label>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autocomplete="current-password"
                                required
                                placeholder="Senha"
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-b-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm dark:bg-gray-700"
                                prop:value=password
                                on:input=move |ev| password.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    <div class="flex items-center justify-between">
                        <div class="flex items-center">
                            <input
                                id="remember-me"
                                name="remember-me"
                                type="checkbox"
                                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                            />
                            <label for="remember-me" class="ml-2 block text-sm text-gray-900 dark:text-gray-300">
                                Lembrar-me
                            </label>
                        </div>

                        <div class="text-sm">
                            <a href="#" class="font-medium text-blue-600 hover:text-blue-500 dark:text-blue-400 dark:hover:text-blue-300">
                                Esqueceu sua senha?
                            </a>
                        </div>
                    </div>

                    <div>
                        <button
                            type="submit"
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        >
                            Entrar
                        </button>
                    </div>
                    
                    {move || {
                        if !error.get().is_empty() {
                            view! {
                                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                                    <span class="block sm:inline">{error.get()}</span>
                                </div>
                            }.into_view()
                        } else {
                            ().into_view()
                        }
                    }}
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState must be provided");
    
    // Load data if needed
    create_effect(move |_| {
        if app_state.appointments.get().is_empty() {
            spawn_local(async move {
                match get_appointments().await {
                    Ok(appointments) => app_state.appointments.set(appointments),
                    Err(_) => {}
                }
            });
        }
        
        if app_state.patients.get().is_empty() {
            spawn_local(async move {
                match get_patients().await {
                    Ok(patients) => app_state.patients.set(patients),
                    Err(_) => {}
                }
            });
        }
    });
    
    let total_appointments = move || app_state.appointments.get().len();
    let scheduled_appointments = move || {
        app_state.appointments.get()
            .iter()
            .filter(|a| matches!(a.status, AppointmentStatus::Scheduled))
            .count()
    };
    let total_patients = move || app_state.patients.get().len();
    
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Dashboard</h1>
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                    Bem-vindo ao seu painel de controle
                </p>
            </div>
            
            // Metrics cards
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                    <div class="flex items-center">
                        <div class="p-3 rounded-full bg-blue-100 dark:bg-blue-900/50">
                            <CalendarIcon/>
                        </div>
                        <div class="ml-4">
                            <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">Consultas Hoje</h3>
                            <p class="text-2xl font-bold text-gray-900 dark:text-white">{total_appointments}</p>
                        </div>
                    </div>
                </div>
                
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                    <div class="flex items-center">
                        <div class="p-3 rounded-full bg-green-100 dark:bg-green-900/50">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </div>
                        <div class="ml-4">
                            <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">Agendadas</h3>
                            <p class="text-2xl font-bold text-gray-900 dark:text-white">{scheduled_appointments}</p>
                        </div>
                    </div>
                </div>
                
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                    <div class="flex items-center">
                        <div class="p-3 rounded-full bg-purple-100 dark:bg-purple-900/50">
                            <UsersIcon/>
                        </div>
                        <div class="ml-4">
                            <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">Total Pacientes</h3>
                            <p class="text-2xl font-bold text-gray-900 dark:text-white">{total_patients}</p>
                        </div>
                    </div>
                </div>
            </div>
            
            // Recent appointments
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow">
                <div class="px-6 py-5 border-b border-gray-200 dark:border-gray-700">
                    <h2 class="text-lg font-medium text-gray-900 dark:text-white">Próximas Consultas</h2>
                </div>
                <div class="divide-y divide-gray-200 dark:divide-gray-700">
                    {move || {
                        app_state.appointments.get()
                            .into_iter()
                            .take(5)
                            .map(|appointment| {
                                view! {
                                    <div class="px-6 py-4">
                                        <div class="flex items-center justify-between">
                                            <div>
                                                <p class="text-sm font-medium text-gray-900 dark:text-white">{appointment.patient_name.clone()}</p>
                                                <p class="text-sm text-gray-500 dark:text-gray-400">{appointment.date_time.clone()}</p>
                                            </div>
                                            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300">
                                                {format_status(appointment.status.clone())}
                                            </span>
                                        </div>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}

fn format_status(status: AppointmentStatus) -> String {
    match status {
        AppointmentStatus::Scheduled => "Agendada".to_string(),
        AppointmentStatus::Confirmed => "Confirmada".to_string(),
        AppointmentStatus::Completed => "Concluída".to_string(),
        AppointmentStatus::Canceled => "Cancelada".to_string(),
    }
}

#[component]
pub fn Appointments() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState must be provided");
    
    // Load appointments if needed
    create_effect(move |_| {
        if app_state.appointments.get().is_empty() {
            spawn_local(async move {
                match get_appointments().await {
                    Ok(appointments) => app_state.appointments.set(appointments),
                    Err(_) => {}
                }
            });
        }
    });
    
    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Agenda</h1>
                <a href="/appointments/new" class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                    Nova Consulta
                </a>
            </div>
            
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                    <thead class="bg-gray-50 dark:bg-gray-700">
                        <tr>
                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                                Paciente
                            </th>
                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                                Data/Hora
                            </th>
                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                                Status
                            </th>
                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                                Duração
                            </th>
                            <th scope="col" class="relative px-6 py-3">
                                <span class="sr-only">Ações</span>
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                        {move || {
                            app_state.appointments.get()
                                .into_iter()
                                .map(|appointment| {
                                    view! {
                                        <tr>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                                                {appointment.patient_name.clone()}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                                                {appointment.date_time.clone()}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm">
                                                <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300">
                                                    {format_status(appointment.status.clone())}
                                                </span>
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                                                {format!("{}", appointment.duration)} " min"
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                                <a href="#" class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300 mr-3">
                                                    Editar
                                                </a>
                                                <a href="#" class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300">
                                                    Cancelar
                                                </a>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn NewAppointment() -> impl IntoView {
    let navigate = use_navigate();
    
    let patient_id = create_rw_signal(String::new());
    let date_time = create_rw_signal(String::new());
    let duration = create_rw_signal(String::from("60"));
    let notes = create_rw_signal(String::new());
    
    let submit = create_action(move |(patient_id, date_time, duration, notes): &(String, String, String, String)| {
        let patient_id = patient_id.parse::<u32>().unwrap_or(0);
        let date_time = date_time.clone();
        let duration = duration.parse::<u32>().unwrap_or(60);
        let notes = if notes.is_empty() { None } else { Some(notes.clone()) };
        
        async move {
            let appointment = Appointment {
                id: rand::random::<u32>(),
                patient_id,
                patient_name: "Nome do Paciente".to_string(), // Would normally fetch from patient service
                date_time,
                duration,
                status: AppointmentStatus::Scheduled,
                notes,
            };
            
            let app_state = use_context::<AppState>().expect("AppState must be provided");
            let mut appointments = app_state.appointments.get_untracked();
            appointments.push(appointment);
            app_state.appointments.set(appointments);
            
            navigate("/appointments", Default::default());
        }
    });
    
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Nova Consulta</h1>
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                    Agende uma nova consulta para um paciente
                </p>
            </div>
            
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    submit.dispatch((
                        patient_id.get(),
                        date_time.get(),
                        duration.get(),
                        notes.get()
                    ));
                }>
                    <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                        <div class="sm:col-span-3">
                            <label for="patient_id" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                Paciente
                            </label>
                            <select
                                id="patient_id"
                                name="patient_id"
                                required
                                class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                                prop:value=patient_id
                                on:change=move |ev| patient_id.set(event_target_value(&ev))
                            >
                                <option value="">Selecione um paciente</option>
                                // Options would be populated from patient list
                            </select>
                        </div>
                        
                        <div class="sm:col-span-3">
                            <label for="date_time" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                Data e Hora
                            </label>
                            <input
                                type="datetime-local"
                                id="date_time"
                                name="date_time"
                                required
                                class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                                prop:value=date_time
                                on:input=move |ev| date_time.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div class="sm:col-span-3">
                            <label for="duration" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                Duração (minutos)
                            </label>
                            <select
                                id="duration"
                                name="duration"
                                class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                                prop:value=duration
                                on:change=move |ev| duration.set(event_target_value(&ev))
                            >
                                <option value="30">30 minutos</option>
                                <option value="45">45 minutos</option>
                                <option value="60">60 minutos</option>
                                <option value="90">90 minutos</option>
                            </select>
                        </div>
                        
                        <div class="sm:col-span-6">
                            <label for="notes" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                Observações
                            </label>
                            <textarea
                                id="notes"
                                name="notes"
                                rows="4"
                                class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                                prop:value=notes
                                on:input=move |ev| notes.set(event_target_value(&ev))
                            ></textarea>
                        </div>
                    </div>
                    
                    <div class="mt-8 flex justify-end space-x-3">
                        <button
                            type="button"
                            class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-200 dark:bg-gray-600 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                            on:click=move |_| navigate("/appointments", Default::default())
                        >
                            Cancelar
                        </button>
                        <button
                            type="submit"
                            class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        >
                            Salvar Consulta
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn Patients() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState must be provided");
    
    // Load patients if needed
    create_effect(move |_| {
        if app_state.patients.get().is_empty() {
            spawn_local(async move {
                match get_patients().await {
                    Ok(patients) => app_state.patients.set(patients),
                    Err(_) => {}
                }
            });
        }
    });
    
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Pacientes</h1>
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                    Gerencie os pacientes da clínica
                </p>
            </div>
            
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                    <thead class="bg-gray-50 dark:bg-gray-700">
                        <tr>
                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                                Nome
                            </th>
                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                                Contato
                            </th>
                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                                CPF
                            </th>
                            <th scope="col" class="relative px-6 py-3">
                                <span class="sr-only">Ações</span>
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                        {move || {
                            app_state.patients.get()
                                .into_iter()
                                .map(|patient| {
                                    view! {
                                        <tr>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                                                {patient.name.clone()}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                                                {patient.email.clone()} " / " {patient.phone.clone()}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                                                {patient.cpf.clone()}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                                <a href={format!("/patients/{}", patient.id)} class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300">
                                                    Ver detalhes
                                                </a>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn PatientDetail() -> impl IntoView {
    let params = use_params_map();
    let patient_id = move || {
        params.with(|params| {
            params.get("id")
                .and_then(|id| id.parse::<u32>().ok())
                .unwrap_or(0)
        })
    };
    
    let app_state = use_context::<AppState>().expect("AppState must be provided");
    
    let patient = create_memo(move |_| {
        app_state.patients.get()
            .into_iter()
            .find(|p| p.id == patient_id())
    });
    
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Detalhes do Paciente</h1>
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                    Informações completas sobre o paciente
                </p>
            </div>
            
            {move || match patient() {
                Some(patient) => {
                    view! {
                        <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        Nome Completo
                                    </label>
                                    <div class="mt-1 block w-full rounded-md border-transparent bg-gray-100 dark:bg-gray-700 py-2 px-3 shadow-sm text-gray-900 dark:text-white sm:text-sm">
                                        {patient.name.clone()}
                                    </div>
                                </div>
                                
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        CPF
                                    </label>
                                    <div class="mt-1 block w-full rounded-md border-transparent bg-gray-100 dark:bg-gray-700 py-2 px-3 shadow-sm text-gray-900 dark:text-white sm:text-sm">
                                        {patient.cpf.clone()}
                                    </div>
                                </div>
                                
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        Email
                                    </label>
                                    <div class="mt-1 block w-full rounded-md border-transparent bg-gray-100 dark:bg-gray-700 py-2 px-3 shadow-sm text-gray-900 dark:text-white sm:text-sm">
                                        {patient.email.clone()}
                                    </div>
                                </div>
                                
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        Telefone
                                    </label>
                                    <div class="mt-1 block w-full rounded-md border-transparent bg-gray-100 dark:bg-gray-700 py-2 px-3 shadow-sm text-gray-900 dark:text-white sm:text-sm">
                                        {patient.phone.clone()}
                                    </div>
                                </div>
                                
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        Data de Nascimento
                                    </label>
                                    <div class="mt-1 block w-full rounded-md border-transparent bg-gray-100 dark:bg-gray-700 py-2 px-3 shadow-sm text-gray-900 dark:text-white sm:text-sm">
                                        {patient.birth_date.clone()}
                                    </div>
                                </div>
                                
                                <div class="sm:col-span-6">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        Endereço
                                    </label>
                                    <div class="mt-1 block w-full rounded-md border-transparent bg-gray-100 dark:bg-gray-700 py-2 px-3 shadow-sm text-gray-900 dark:text-white sm:text-sm">
                                        {patient.address.clone().unwrap_or_else(|| "Não informado".to_string())}
                                    </div>
                                </div>
                            </div>
                            
                            <div class="mt-8">
                                <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">Consultas Agendadas</h2>
                                <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 rounded-lg">
                                    <table class="min-w-full divide-y divide-gray-300">
                                        <thead class="bg-gray-50 dark:bg-gray-700">
                                            <tr>
                                                <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 dark:text-white sm:pl-6">
                                                    Data/Hora
                                                </th>
                                                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900 dark:text-white">
                                                    Status
                                                </th>
                                                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900 dark:text-white">
                                                    Duração
                                                </th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-gray-200 dark:divide-gray-700 bg-white dark:bg-gray-800">
                                            {move || {
                                                app_state.appointments.get()
                                                    .into_iter()
                                                    .filter(|a| a.patient_id == patient.id)
                                                    .map(|appointment| {
                                                        view! {
                                                            <tr>
                                                                <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 dark:text-white sm:pl-6">
                                                                    {appointment.date_time.clone()}
                                                                </td>
                                                                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500 dark:text-gray-400">
                                                                    <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300">
                                                                        {format_status(appointment.status.clone())}
                                                                    </span>
                                                                </td>
                                                                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500 dark:text-gray-400">
                                                                    {format!("{}", appointment.duration)} " min"
                                                                </td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            }}
                                        </tbody>
                                    </table>
                                </div>
                            </div>
                            
                            <div class="mt-8 flex justify-end">
                                <a href="/patients" class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-200 dark:bg-gray-600 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500">
                                    Voltar
                                </a>
                            </div>
                        </div>
                    }.into_view()
                }
                None => {
                    view! {
                        <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                            <div class="text-center py-12">
                                <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">Paciente não encontrado</h3>
                                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                                    O paciente solicitado não existe ou foi removido.
                                </p>
                                <div class="mt-6">
                                    <a href="/patients" class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                                        Voltar para lista
                                    </a>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Configurações</h1>
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                    Gerencie as configurações da sua conta
                </p>
            </div>
            
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                    <div class="sm:col-span-3">
                        <label for="first-name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                            Primeiro nome
                        </label>
                        <input
                            type="text"
                            name="first-name"
                            id="first-name"
                            class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                        />
                    </div>
                    
                    <div class="sm:col-span-3">
                        <label for="last-name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                            Sobrenome
                        </label>
                        <input
                            type="text"
                            name="last-name"
                            id="last-name"
                            class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                        />
                    </div>
                    
                    <div class="sm:col-span-6">
                        <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                            Email
                        </label>
                        <input
                            type="email"
                            name="email"
                            id="email"
                            class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                        />
                    </div>
                    
                    <div class="sm:col-span-6">
                        <label for="timezone" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                            Fuso Horário
                        </label>
                        <select
                            id="timezone"
                            name="timezone"
                            class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                        >
                            <option>(UTC-03:00) Brasília</option>
                            <option>(UTC-04:00) Manaus</option>
                            <option>(UTC-05:00) Rio Branco</option>
                        </select>
                    </div>
                    
                    <div class="sm:col-span-6">
                        <label for="language" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                            Idioma
                        </label>
                        <select
                            id="language"
                            name="language"
                            class="block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-3 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm text-gray-900 dark:text-white"
                        >
                            <option>Português (Brasil)</option>
                            <option>Inglês</option>
                            <option>Espanhol</option>
                        </select>
                    </div>
                </div>
                
                <div class="mt-8 flex justify-end">
                    <button
                        type="button"
                        class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-200 dark:bg-gray-600 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                    >
                        Cancelar
                    </button>
                    <button
                        type="submit"
                        class="ml-3 px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                    >
                        Salvar
                    </button>
                </div>
            </div>
        </div>
    }
}