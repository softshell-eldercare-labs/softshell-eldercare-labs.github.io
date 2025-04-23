#![allow(non_snake_case)]

// use components::{NavBar, Profile, ProjectGrid, WorkExperience, DeepDiveBlogList};
use dioxus::{prelude::*, web::WebEventExt};
use dioxus_logger::tracing::info;
// use web_sys::wasm_bindgen::JsCast;
use std::collections::HashMap;
use serde_json::{Value};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(NavBar)]
        #[route("/")]
        Home {}
}
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const PROFILE_PIC: Asset = asset!("/assets/1152300.png");
pub const POSTER: Asset = asset!("/assets/poster.png");
pub const WALLPAPER: Asset = asset!("/assets/wallpaper-video.mp4");

fn main() {
    // dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        document::Title { "SoftShell ElderCare" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css",
        }
        // document::Link {
        //     rel: "stylesheet",
        //     href: TAILWIND_CSS,
        // }

        Router::<Route> {}
    }
}

pub static PROFILE_ELEMENT: GlobalSignal<
    Option<dioxus::prelude::Event<dioxus::events::MountedData>>,
> = Global::new(|| None);

// Home component - Main landing page container

#[component]
fn Home() -> Element {

    let mut video_ref = use_signal(|| None::<dioxus::prelude::Event<dioxus::events::MountedData>>);
    let mut is_video_loaded = use_signal(|| false);
    let mut count = use_signal(|| 0);
    let mut browser_signal = use_signal(|| String::new());
    let mut os_signal = use_signal(|| String::new()); 

    let tmp = use_resource(move ||  {
        let is_loaded = *is_video_loaded.read();
        let browser = browser_signal.read().clone();
        let os = os_signal.read().clone();
        async move  {
            
            if is_loaded {
                let js_code = format!(
                    r#"
                    const browser_name = `{}`;
                    const os_name = `{}`;
  
                    try {{
                        const video = document.getElementById('vbackground');
                        if (video) {{
                            
                             const videoSrc = video.querySelector('source')?.src || video.src;
                            // Ensure muted and playsinline for Safari
                            video.muted = true;
                            if (browser_name.includes('Safari') || os_name.includes('iOS') || os_name.includes('MacOS')) {{
                                video.setAttribute('playsinline', '');
                                video.setAttribute('disablepictureinpicture', '');
                                video.setAttribute('muted', '');

                                if (videoSrc) {{
                                    video.style.backgroundImage = `url("${{videoSrc}}")`;
                                    video.style.backgroundSize = 'cover';
                                    video.style.backgroundPosition = 'center';
                                }}
                            }}
                            const playPromise = video.play();
                            if (playPromise !== undefined) {{
                                playPromise.catch(error => {{
                                    console.error('Initial autoplay failed:', error);
                                    // Fallback to ensure muted and retry
                                    
                                    video.play().catch(e => {{
                                        console.error('Retry autoplay failed:', e);
                                    }});
                                }});
                            }}
                        }} else {{
                            console.error('Video element not found');
                        }}
                    }} catch (e) {{
                        console.error('Video autoplay error:', e);
                    }}
                    return 'set video settings';
                    "#,
                    browser,
                    os
                );
    
                match document::eval(&js_code).await {
                    Ok(_) => info!("Video autoplay initiated"),
                    Err(e) => info!("JS eval error: {:?}", e),
                }
            }
          
        
        }
    }
    );
    
    let browser_settings = move |_| {

        async move  {
            
        let js_code = r#"

            const userAgent = navigator.userAgent;
            const platform = navigator.platform;
            const language = navigator.language || navigator.userLanguage;
            const screenWidth = window.screen.width;
            const screenHeight = window.screen.height;
            const devicePixelRatio = window.devicePixelRatio;
            const isOnline = navigator.onLine;
            const cookiesEnabled = navigator.cookieEnabled;
            
            // Browser detection
            let browserName = "Unknown";
            if (userAgent.includes("Chrome")) browserName = "Chrome";
            else if (userAgent.includes("Firefox")) browserName = "Firefox";
            else if (userAgent.includes("Safari")) browserName = "Safari";
            else if (userAgent.includes("Edge")) browserName = "Edge";
            else if (userAgent.includes("Opera")) browserName = "Opera";

            // OS detection
            let os = "Unknown";
            if (userAgent.includes("Win")) os = "Windows";
            else if (userAgent.includes("Mac")) os = "MacOS";
            else if (userAgent.includes("Linux")) os = "Linux";
            else if (userAgent.includes("Android")) os = "Android";
            else if (userAgent.includes("iPhone") || userAgent.includes("iPad")) os = "iOS";

            // Device type
            const isMobile = /Mobile|Android|iPhone|iPad/.test(userAgent);
            const deviceType = isMobile ? "Mobile" : "Desktop";

            // Create JSON object
            const deviceInfo = {
                browser: browserName,
                operatingSystem: os,
                deviceType: deviceType,
                platform: platform,
                language: language,
                screenResolution: `${screenWidth}x${screenHeight}`,
                devicePixelRatio: devicePixelRatio,
                onlineStatus: isOnline,
                cookiesEnabled: cookiesEnabled,
                userAgent: userAgent
            };
            console.log(JSON.stringify(deviceInfo));
            dioxus.send(JSON.stringify(deviceInfo));
        "#;
        
        let resp = document::eval(&js_code).recv().await;
        match resp {
            Ok(json_value) => {
                
                info!("parsed: {:?}", json_value);
                // Convert to HashMap
                // Match json_value, assuming it's a String containing a JSON dictionary
            match json_value {
                Value::String(json_str) => {
                    // Parse the JSON string into a Value
                    match serde_json::from_str::<Value>(&json_str) {
                        Ok(parsed_value) => {
                            // Ensure parsed_value is an object
                            if let Value::Object(json_map) = parsed_value {
                                let hash_map: HashMap<String, Value> = json_map.into_iter().collect();
                                if hash_map.is_empty() {
                                    info!("HashMap is empty!");
                                } else {
                                    info!("Device Info HashMap: {:?}", hash_map);
                                    // Access specific fields
                                    if let Some(browser) = hash_map.get("browser") {
                                        info!("Browser: {}", browser.as_str().unwrap_or("Unknown"));
                                        browser_signal.set(browser.as_str().unwrap_or("Unknown").to_string());
                                    }
                                    if let Some(os) = hash_map.get("operatingSystem") {
                                        info!("Operating System: {}", os.as_str().unwrap_or("Unknown"));
                                        os_signal.set(os.as_str().unwrap_or("Unknown").to_string());
                                    }
                                }
                            } else {
                                info!("Parsed JSON is not an object: {:?}", parsed_value);
                            }
                        }
                        Err(err) => {
                            info!("Error parsing JSON string: {}", err);
                        }
                    }
                }
                _ => {
                    info!("json_value is not a string: {:?}", json_value);
                }

                }
            }
            Err(e) => {
                info!("JS eval error: {:?}", e);
            }
        }

        }


    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }

    
        div {
            // class: "min-h-screen bg-gray-50",
            
            // Main content
            main {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                
                // Hero section with company name
                section {
                    class: "hero-section",  // Fixed height container
                    

                    video {
                        id: "vbackground",
                        class: "video-background",  // Covers entire section with slight 
                        autoplay: true,
                        controls: false,
                        crossorigin: "anonymous",
                        r#loop: true,
                        muted: true,
                        preload: "auto",
                        poster: POSTER,
                        // Safari-specific attributes
                        src: WALLPAPER,    
                        onmounted: browser_settings,
                        oncanplay: move |_| is_video_loaded.set(true),
                        // onstalled: move |_| is_video_loaded.set(false),
                        // onsuspend: move |_| is_video_loaded.set(false),
                        
                    }
  
                    div {
                        class: "video-overlay",
                    }


                    // Content container with relative positioning
                    div {
                        class: "video-content-overlay",
                        
                        h1 {
                            class: "text-4xl font-extrabold sm:text-5xl sm:tracking-tight lg:text-6xl drop-shadow-lg",
                            "Soft Shell Elder Care"
                        }
                        p {
                            class: "mt-5 max-w-xl mx-auto text-xl drop-shadow-lg",
                            "Ensuring elderly patients receive timely, accurate, and attentive hospital care through dedicated advocacy."
                        }
                    }
                }
                br {}

                // Mission section
                section {
                    class: "mb-16",
                    h2 {
                        class: "text-2xl font-bold text-gray-900 mb-6",
                        "Our Mission"
                    }
                    div {
                        class: "bg-white shadow rounded-lg p-6",
                        p {
                            class: "text-gray-700",
                            "At SoftShell Elder Care, our mission is to ensure elderly patients receive timely, accurate, and attentive hospital care through dedicated advocacy."
                        }
                    }
                }
                
                // Problem statement
                section {
                    class: "mb-16",
                    h2 {
                        class: "text-2xl font-bold text-gray-900 mb-6",
                        "The Problem We Address"
                    }
                    div {
                        class: "bg-white shadow rounded-lg p-6",
                        p {
                            class: "text-gray-700 mb-4",
                            "Elderly hospital patients, especially those with dementia or cognitive impairment, often face preventable complications due to lack of consistent care advocacy."
                        }
                        ul {
                            class: "list-disc pl-5 text-gray-700 space-y-2",
                            li { "Advocate for timely hydration, nutrition, and treatment scheduling." }
                            li { "Provide patient-centric oversight and communication." }
                            li { "Offer peace of mind to families unable to be present consistently." }
                            li { "Operate with minimal liability through advisory and reminder-focused services." }
                        }
                    }
                }
                
                // Our solution
                section {
                    class: "mb-16",
                    h2 {
                        class: "text-2xl font-bold text-gray-900 mb-6",
                        "Our Solution"
                    }
                    div {
                        class: "bg-white shadow rounded-lg p-6",
                        p {
                            class: "text-gray-700 mb-4",
                            "SoftShell Elder Care offers a comprehensive suite of services designed to address these challenges:"
                        }
                        div {
                            class: "grid md:grid-cols-2 gap-6 mt-6",
                            div {
                                class: "bg-indigo-50 p-4 rounded-lg",
                                h3 {
                                    class: "font-bold text-indigo-800 mb-2",
                                    "Advocacy for timely care"
                                }
                                p {
                                    class: "text-gray-700",
                                    "Advocate for timely hydration, nutrition, and treatment scheduling."
                                }
                            }
                            div {
                                class: "bg-indigo-50 p-4 rounded-lg",
                                h3 {
                                    class: "font-bold text-indigo-800 mb-2",
                                    "Treatment tracking"
                                }
                                p {
                                    class: "text-gray-700",
                                    "Professionals trained in the medical field will keep doctors accountable for treatments and be aware of complications."
                                }
                            }
                            div {
                                class: "bg-indigo-50 p-4 rounded-lg",
                                h3 {
                                    class: "font-bold text-indigo-800 mb-2",
                                    "Provide geriatric care"
                                }
                                p {
                                    class: "text-gray-700",
                                    "Staff will be trained in nursing and be able to take care of your loved one."
                                }
                            }
                            div {
                                class: "bg-indigo-50 p-4 rounded-lg",
                                h3 {
                                    class: "font-bold text-indigo-800 mb-2",
                                    "Digital Dashboard"
                                }
                                p {
                                    class: "text-gray-700",
                                    "Provide metrics and dashboards to track patient updates and progress."
                                }
                            }
                        }
                    }
                }
                
                // Contact form
                // Contact form
                section {
                    h2 {
                        class: "text-2xl font-bold text-gray-900 mb-6",
                        "Contact Us"
                    }
                    div {
                        class: "bg-white shadow rounded-lg p-6",
                        form {
                            class: "space-y-6",
                            div {
                                class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-2",
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700",
                                        "First name"
                                    }
                                    div {
                                        class: "mt-1",
                                        input {
                                            r#type: "text",
                                            class: "py-3 px-4 block w-full border border-gray-300 rounded-md shadow-sm",
                                        }
                                    }
                                }
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700",
                                        "Last name"
                                    }
                                    div {
                                        class: "mt-1",
                                        input {
                                            r#type: "text",
                                            class: "py-3 px-4 block w-full border border-gray-300 rounded-md shadow-sm",
                                        }
                                    }
                                }
                            }
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700",
                                    "Email address"
                                }
                                div {
                                    class: "mt-1",
                                    input {
                                        r#type: "email",
                                        class: "py-3 px-4 block w-full border border-gray-300 rounded-md shadow-sm",
                                    }
                                }
                            }
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700",
                                    "Phone number"
                                }
                                div {
                                    class: "mt-1",
                                    input {
                                        r#type: "tel",
                                        class: "py-3 px-4 block w-full border border-gray-300 rounded-md shadow-sm",
                                    }
                                }
                            }
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700",
                                    "Message"
                                }
                                div {
                                    class: "mt-1",
                                    textarea {
                                        rows: "4",
                                        class: "py-3 px-4 block w-full border border-gray-300 rounded-md shadow-sm",
                                    }
                                }
                            }
                            div {
                                button {
                                    r#type: "submit",
                                    class: "inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                    "Submit"
                                }
                            }
                        }
                    }
                }
            }
            
            // Footer
            footer {
                class: "bg-white border-t border-gray-200 mt-12",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                    p {
                        class: "text-center text-gray-500 text-sm",
                        "© 2025 SoftShell Elder Care All rights reserved."
                    }
                }
            }
        }                
        // div {

        //    class: "min-h-screen bg-background text-text-primary",

            // Main container
            // Hero/Profile section
            // onmounted: move |data| {
            //     *PROFILE_ELEMENT.write() = Some(data);
            // },
            // Profile {
            // }

            // Footer
            // div { Footer {} }
        // }
    }
}
