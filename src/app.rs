use home_page::HomePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use not_found::NotFound;
use blog_show::Blog;
use test_show::Test;

use crate::{components::{footer::Footer, header::Header}, pages::*};


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-project.css"/>
        <Title text="Welcome to Leptos Blog"/>
        <Router>
            // <Header/>
            <main class="flex flex-col min-h-screen bg-gradient-to-br from-gray-900 to-black text-white font-sans">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/articles" view=Blog/>
                    <Route path="/test" view=Test/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}