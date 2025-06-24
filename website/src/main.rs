use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const RESUME_PDF: Asset = asset!("/assets/Jonah_Irizarry_Resume.pdf");

fn main() {
    // web_sys::console::log_1(&"Starting...".into()); //console.info()
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // rsx! {
    //     document::Link { rel: "icon", href: FAVICON }
    //     document::Link { rel: "stylesheet", href: MAIN_CSS }
    //     Hero {}
    // }
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div{
            // class: if,
            id: "header",
            a{href: "#", "Jonah Irizarry"}
            div {
                // id: "nav",
                id: "links",
                // a {href: "#about", "About Me"},
                // a {href: "#projects", "Projects"},
                // a {href: "#resume", "Resume"},
                a {href: "javascript:Array.from(document.querySelectorAll('#about')).forEach(el => el.classList.remove('deactive')); Array.from(document.querySelectorAll('#projects')).forEach(el => el.classList.add('deactive')); Array.from(document.querySelectorAll('#resume')).forEach(el => el.classList.add('deactive'));", "About Me"},
                a {href: "javascript:Array.from(document.querySelectorAll('#projects')).forEach(el => el.classList.remove('deactive')); Array.from(document.querySelectorAll('#about')).forEach(el => el.classList.add('deactive')); Array.from(document.querySelectorAll('#resume')).forEach(el => el.classList.add('deactive'));", "Projects"},
                a {href: "javascript:Array.from(document.querySelectorAll('#resume')).forEach(el => el.classList.remove('deactive')); Array.from(document.querySelectorAll('#projects')).forEach(el => el.classList.add('deactive')); Array.from(document.querySelectorAll('#about')).forEach(el => el.classList.add('deactive'));", "Resume"},
            }
        }
        MainPage{}
    }
}

#[component]
fn MainPage() -> Element{
    rsx!{
        div{
            id: "main-content",
            AbtMe{},
            ProjectsPage{},
            ResumePage{},
        }
    }
}

#[component]
fn AbtMe() -> Element{
    rsx!{
        div{
            id: "about",
            h1 {"About Me"},
            p {"I’m a passionate computer science student and AI enthusiast who recently completed my Bachelor’s in Computer Science with a focus on Artificial Intelligence through Binghamton University’s 4+1 program (expected Master’s completion in May 2026). My academic and project work, spanning AI-powered tools like a Tesseract OCR-integrated dataset processor, a Rust-based feed aggregator, and a Flask-driven web scraper, has deepened my fascination with leveraging AI to solve complex problems. I’m particularly drawn to local AI tools like Ollama and llama.cpp, which empower developers to deploy and refine models independently."},
            p{"Beyond the classroom, I’ve applied my skills as a Solutions Engineer at Sense Education, where I explored integrating Chat-GPT 3.5 to streamline data processing and improve model adaptability. My work also includes leading a team to build an AI moderation tool for web scraping projects and developing terminal-based applications in C++ and Rust. I thrive at the intersection of machine learning, software development, and user-centric design, and I’m excited to continue pushing the boundaries of AI using both cutting-edge technologies and local hosting tools like Ollama to create impactful, real-world solutions."},
        },        
    }
}

#[component]
fn ProjectsPage() -> Element{
    rsx!{        
        div{
            id: "projects",
            class: "deactive",
            h1 {"Projects"},
            "This is where projects will go. Most likely on some sort of carrosel or slider"
        },
    }
}

#[component]
fn ResumePage() -> Element{
    rsx!{
        div{
            id: "resume",
            class: "deactive",
            h1 {"Resume"},
            embed{
                id: "Resume",
                // src: "/home/linky/Resumes/Jonah_Irizarry_Resume.pdf",
                src: "{RESUME_PDF}",
                // src: "/run/user/1000/doc/f60e0210/Jonah_Irizarry_Resume.pdf",
                r#type: "application/pdf",
                width: "1000",
                height: "1200",
            },
        },
    }
}

// #[component]
// pub fn Hero() -> Element {
//     rsx! {
//         div {
//             id: "hero",
//             img { src: HEADER_SVG, id: "header" }
//             div { id: "links",
//                 a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
//                 a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
//                 a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
//                 a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
//                 a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
//                 a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
//             }
//         }
//     }
// }
