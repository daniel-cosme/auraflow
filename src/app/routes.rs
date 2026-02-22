use leptos::*;
use leptos_router::*;

use crate::app::components::{
    Layout, Login, Dashboard, Appointments, NewAppointment, Patients, PatientDetail, Settings
};

#[component]
pub fn Routes() -> impl IntoView {
    view! {
        <main>
            <Routes>
                <Route path="" view=|| view!{ <Login/> } ssr=SsrMode::Async />
                <Route path="/login" view=|| view!{ <Login/> } ssr=SsrMode::Async />
                
                <Route path="/dashboard" view=|| view!{ 
                    <Layout>
                        <Dashboard/>
                    </Layout>
                } ssr=SsrMode::Async />
                
                <Route path="/appointments" view=|| view!{ 
                    <Layout>
                        <Appointments/>
                    </Layout>
                } ssr=SsrMode::Async />
                
                <Route path="/appointments/new" view=|| view!{ 
                    <Layout>
                        <NewAppointment/>
                    </Layout>
                } ssr=SsrMode::Async />
                
                <Route path="/patients" view=|| view!{ 
                    <Layout>
                        <Patients/>
                    </Layout>
                } ssr=SsrMode::Async />
                
                <Route path="/patients/:id" view=|| view!{ 
                    <Layout>
                        <PatientDetail/>
                    </Layout>
                } ssr=SsrMode::Async />
                
                <Route path="/settings" view=|| view!{ 
                    <Layout>
                        <Settings/>
                    </Layout>
                } ssr=SsrMode::Async />
            </Routes>
        </main>
    }
}