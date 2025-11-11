// src/frontend/mod.rs
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = create_signal("home".to_string());

    view! {
        <div style="font-family: Arial, sans-serif; padding: 20px;">
            <nav style="margin-bottom: 20px;">
                <button on:click=move |_| set_page.set("home".to_string()) style="margin-right: 10px;">
                    "Home"
                </button>
                <button on:click=move |_| set_page.set("projects".to_string()) style="margin-right: 10px;">
                    "Projects"
                </button>
                <button on:click=move |_| set_page.set("skills".to_string()) style="margin-right: 10px;">
                    "Skills"
                </button>
                <button on:click=move |_| set_page.set("about".to_string())>
                    "About"
                </button>
            </nav>

            <main>
                {move || match page.get().as_str() {
                    "home" => view! { <HomePage/> },
                    "projects" => view! { <ProjectsPage/> },
                    "skills" => view! { <SkillsPage/> },
                    "about" => view! { <AboutPage/> },
                    _ => view! { <div>"Not found"</div> },
                }}
            </main>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"VStor Tech Portfolio"</h1>
            <p>"Full-stack developer specializing in Rust, Web, and System Programming"</p>
        </div>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    view! {
        <div>
            <h1>"My Projects"</h1>
            <p>"Projects will be loaded from backend API..."</p>
        </div>
    }
}

#[component]
fn SkillsPage() -> impl IntoView {
    view! {
        <div>
            <h1>"My Skills"</h1>
            <p>"Skills will be loaded from backend API..."</p>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <h1>"About Me"</h1>
            <p>"Information about me will be displayed here..."</p>
        </div>
    }
}

pub async fn start_frontend() {
    leptos::mount_to_body(App);
}
