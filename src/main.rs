use dioxus::document::eval;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum Content {
    Text(String),
    Image { url: String, caption: String },
    Link { url: String, label: String },
    LinkList(Vec<(String, String)>),
    Identity { image: String, text: String },
}

#[derive(Clone, PartialEq)]
struct HistoryLine {
    command: String,
    content: Content,
    status: String,
}

fn main() {
    launch(App);
}

fn App() -> Element {
    let mut input_val = use_signal(|| "".to_string());
    let mut history = use_signal(Vec::<HistoryLine>::new);

    let container_style = "
        background-color: #1e1e1e;
        color: #d4d4d4;
        font-family: 'Consolas', 'Monaco', monospace;
        height: 100vh;
        width: 100vw;
        padding: 20px;
        box-sizing: border-box;
        overflow-y: auto;
        font-size: 15px;
        line-height: 1.5;
    ";

    let input_style = "
        background: transparent;
        border: none;
        color: #d4d4d4;
        font-family: inherit;
        font-size: inherit;
        outline: none;
        width: 80%;
        font-weight: bold;
    ";

    let focus_input = move |_| {
        spawn(async move {
            let _ = eval(r#"
                const selection = window.getSelection();
                if (selection.type !== 'Range') {
                    const input = document.getElementById('terminal-input');
                    if (input) input.focus();
                }
            "#);
        });
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            let current_cmd = input_val.read().clone();
            let clean_cmd = current_cmd.trim().to_lowercase();

            // --- THE BRAIN ---
            let (new_content, new_status) = match clean_cmd.as_str() {
                "help" => (
                    Content::Text("AVAILABLE COMMANDS:\n-------------------\nwhoami       -> About Me\nskills       -> Tech Stack\nprojects     -> Projects\nsocials      -> Contact Info\nshowcase     -> Image Demo\nsoul         -> Poetry Collection\nclear        -> Clear Terminal".to_string()),
                    "SYSTEM"
                ),

                "whoami" => (
                    Content::Identity {
                        image: asset!("/public/my_pic.jpg").to_string(),
                        text: "/// ABOUT ME ///
-------------------------
> NAME:        Harold Karl Franze R. Alonsagay
> LOCATION:    Davao City, Philippines
> EDUCATION:   BSIT Student
> FOCUS:       Systems Programming & Tooling
> EMAIL:       alonsagayharold@gmail.com

/// KERNEL PARAMETERS ///
-------------------------
> FOCUS:       Performance, Memory Safety, Tooling
> PHILOSOPHY:  \"Poetry and Programming are the same; the only difference lies in who reads it.\"
> OBJECTIVE:   I am aspiring to be a systems programmer
                    "
                            .to_string()
                    },
                    "SUCCESS"
                ),

                "skills" => (
                    Content::Text("
/// HARD SKILLS (TECH STACK) ///
--------------------------------
> Languages:   Java, C#, Rust, C, SQL, Python
> Tools:       Github, RustRover, Clion, PyCharm

/// SOFT SKILLS (HUMAN PROTOCOLS) ///
-------------------------------------
> Creative Problem Solving
> Team Communication & Collaboration
> Adaptability (Fast Learner)
> Critical Thinking

/// FIELDS OF INTEREST ///
--------------------------
> Systems Programming
> Legacy Maintainer
> Low level management
                    ".to_string()),
                    "SUCCESS"
                ),

                "showcase" => (
                    Content::Image {
                        url: "https://media.giphy.com/media/f3iwJFOvoSqju/giphy.gif".to_string(),
                        caption: "VISUAL_DATA_STREAM_ESTABLISHED".to_string()
                    },
                    "SUCCESS"
                ),

                "projects" => (
                    Content::LinkList(vec![
                        ("MOMMYSHELL [Rust Bash Shell]".to_string(), "https://github.com/SerenicuS/MommySuite".to_string()),
                        ("MOMMYLANG [Esolang -> C]".to_string(), "https://github.com/SerenicuS/MommySuite".to_string()),
                        ("C_SHELL [Terminal in C]".to_string(), "https://github.com/SerenicuS/custom-shell".to_string()),
                    ]),
                    "SUCCESS"
                ),

                "socials" => (
                    Content::LinkList(vec![
                        ("LINKEDIN PROFILE".to_string(), "https://www.linkedin.com/in/harold-karl-franze-alonsagay-95b1a82a5/".to_string()),
                        ("GITHUB REPO".to_string(), "https://github.com/SerenicuS".to_string()),
                        ("FACEBOOK".to_string(), "https://www.facebook.com/HKFA2002".to_string()),
                    ]),
                    "SUCCESS"
                ),

                "soul" => (
                    Content::LinkList(vec![
                        // The text shown in terminal  |  The path to the file
                        // Green (Garden)
                        ("01_GARDEN_OF_MANY.txt".to_string(),  "poems/garden_of_many.html".to_string()),
                        // Pink (Sensuality)
                        ("02_NAVIGATION_OF_SENSUALITY.txt".to_string(),  "poems/navigation_of_sensuality.html".to_string()),
                    ]),
                    "SUCCESS"
                ),

                "clear" => (Content::Text("CLEAR_SIGNAL".to_string()), "SYSTEM"),
                "" => (Content::Text("".to_string()), "SUCCESS"),
                _ => (Content::Text(format!("ERR: Command '{}' denied. Access restricted.", clean_cmd)), "ERROR"),
            };

            if let Content::Text(text) = &new_content {
                if text == "CLEAR_SIGNAL" {
                    history.write().clear();
                    input_val.set("".to_string());
                    return;
                }
            }

            history.write().push(HistoryLine {
                command: current_cmd,
                content: new_content,
                status: new_status.to_string(),
            });


            input_val.set("".to_string());

            spawn(async move {
                let _ = eval("window.scrollTo(0, document.body.scrollHeight);");
            });
        }
    };

    rsx! {
        div {
            style: "{container_style}",
            onclick: focus_input,


            div { style: "margin-bottom: 20px; color: #569cd6;",
                pre {
                   r#"
  _    _  _____ __      __ ______  __  __  _____  _   _  _____
 | |  | ||_   _|\ \    / /|  ____||  \/  ||_   _|| \ | ||  __ \
 | |__| |  | |   \ \  / / | |__   | \  / |  | |  |  \| || |  | |
 |  __  |  | |    \ \/ /  |  __|  | |\/| |  | |  | . ` || |  | |
 | |  | | _| |_    \  /   | |____ | |  | | _| |_ | |\  || |__| |
 |_|  |_||_____|    \/    |______||_|  |_||_____||_| \_||_____/

                    "#
                }
                div { style: "color: #808080; margin-top: 5px;", "HiveMind Kernel [v1.0.0] - x86_64-unknown-linux-gnu" }
                div { style: "color: #608b4e;", "Type 'help' to initialize session..." }
            }


            for line in history.read().iter() {
                div { style: "margin-bottom: 20px;",

                    div { style: "margin-bottom: 5px;",
                        span { style: "color: #569cd6; margin-right: 5px;", "➜" }
                        span { style: "color: #ce9178;", "~ " }
                        span { style: "opacity: 0.9;", "{line.command}" }
                    }


                    div {
                        style: if line.status == "ERROR" { "color: #f44747;" } else { "color: #cccccc;" },

                        match &line.content {
                            Content::Text(text) => rsx! {
                                pre { style: "margin: 0; white-space: pre-wrap; font-family: inherit; line-height: 1.4;", "{text}" }
                            },
                            Content::Identity { image, text } => rsx! {
                                div { style: "display: flex; gap: 20px; align-items: flex-start; margin-top: 10px;",
                                    img {
                                        src: "{image}",
                                        style: "width: 120px; height: 120px; border: 2px solid #569cd6; border-radius: 4px; object-fit: cover;"
                                    }
                                    pre { style: "margin: 0; white-space: pre-wrap; font-family: inherit; line-height: 1.4;", "{text}" }
                                }
                            },
                            Content::Image { url, caption } => rsx! {
                                div { style: "border-left: 3px solid #569cd6; padding-left: 15px; margin-top: 10px;",
                                    img {
                                        src: "{url}",
                                        style: "max-width: 100%; max-height: 300px; display: block; border-radius: 4px;"
                                    }
                                    div { style: "color: #569cd6; margin-top: 5px; font-size: 13px;", "// {caption}" }
                                }
                            },
                            Content::Link { url, label } => rsx! {
                                a {
                                    href: "{url}",
                                    target: "_blank",
                                    style: "color: #4ec9b0; text-decoration: none; border-bottom: 1px solid #4ec9b0; cursor: pointer;",
                                    "🔗 {label}"
                                }
                            },
                           Content::LinkList(links) => rsx! {
                                div { style: "display: flex; flex-direction: column; gap: 5px; margin-top: 5px;",
                                    for (label, url) in links {
                                        div {
                                            span { style: "margin-right: 10px; color: #569cd6;", "●" }
                                            a {
                                                href: "{url}",
                                                // vvv--- CHANGE THIS LINE ---vvv
                                                target: if url.starts_with("http") { "_blank" } else { "_self" },

                                                style: "color: #d4d4d4; text-decoration: none; hover:text-decoration: underline; cursor: pointer;",
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }


            div { style: "display: flex; align-items: center;",
                span { style: "color: #569cd6; margin-right: 8px;", "➜" }
                span { style: "color: #ce9178; margin-right: 8px;", "user@system" }
                span { style: "color: #808080; margin-right: 8px;", ":" }
                span { style: "color: #4ec9b0; margin-right: 10px;", "~/workspace" }
                span { style: "color: #d4d4d4; margin-right: 10px;", "$" }
                input {
                    id: "terminal-input",
                    style: "{input_style}",
                    value: "{input_val}",
                    oninput: move |evt| input_val.set(evt.value()),
                    onkeydown: handle_keydown,
                    autofocus: true,
                    spellcheck: "false",
                    autocomplete: "off",
                }
            }
        }
    }
}