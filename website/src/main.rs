use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const RESUME_PDF: Asset = asset!("/assets/Jonah_Irizarry_Resume.pdf");

fn main() {
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
        div{
            id: "header",
            a{href: "#", "Jonah Irizarry"}
            div {
                id: "nav",
                a {href: "#about", "About Me"},
                a {href: "#projects", "Projects"},
                a {href: "#resume", "Resume"},
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
            p {"This is where the about me section will go"},
        },        
    }
}

#[component]
fn ProjectsPage() -> Element{
    rsx!{        
        div{
            id: "projects",
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
