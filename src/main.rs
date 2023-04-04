use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ContactFieldProps {
    contact_type: AttrValue,
    display_text: AttrValue,
    contact_url: Option<AttrValue>,
}

#[function_component]
fn ContactField(props: &ContactFieldProps) -> Html {
    let contact_header = match props.contact_type.as_str() {
        "LinkedIn" => html! { <img src="/icons/linkedin-icon.png" class={ classes!("icon") } /> },
        "GitHub" => html! { <img src="/icons/github-icon.svg" class={ classes!("icon") } /> },
        _ => html! { props.contact_type.clone() },
    };

    match props.contact_url {
        Some(..) => html! {
            <div class={ classes!("contact-info") }>
                <a href={ props.contact_url.clone() }>
                    { contact_header }
                    { ": " }
                    { props.display_text.clone() }
                </a>
            </div>
        },
        None => html! {
            <div class={ classes!("contact-info") }>
                { contact_header }
                { ": " }
                { props.display_text.clone() }
            </div>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <header>
                <h1>{ "Wong Chong Kai" }</h1>
                <h2>{ "Backend Developer" }</h2>
                <section>
                    <ContactField contact_type="Email" display_text="wongchongkai@mailbox.org" contact_url="mailto:wongchongkai@mailbox.org"></ContactField>
                    <ContactField contact_type="Phone" display_text="(+81)70-1597-2966"></ContactField>
                    <ContactField contact_type="GitHub" display_text="GitHub" contact_url="https://github.com/wongchongkai"></ContactField>
                    <ContactField contact_type="LinkedIn" display_text="LinkedIn" contact_url="https://www.linkedin.com/in/chong-kai-wong/"></ContactField>
                </section>
            </header>

            <main>
                <section>
                    <h2>{ "Professional Experience" }</h2>

                    <section>
                        <h3>{ "Systems Engineer" }</h3>
                        <p class={ classes!("sub-h3") }>{ "ADInteractive.Co.,Ltd | Japan | July 2022 - now" }</p>
                        <ul>
                            <li>
                                { "Developed an Android app using Flutter, as well as a proof-of-concept web interface using JavaScript." }
                            </li>
                            <li>
                                { "Developed the backend API for the app using Go and gRPC." }
                            </li>
                            <li>
                                { "Developed a serverless API for a consumer-facing service using AWS Lambda and Python." }
                            </li>
                        </ul>
                    </section>

                    <section>
                        <h3>{ "Programmer" }</h3>
                        <p class={ classes!("sub-h3") }>{ "Cadamsoft Sdn. Bhd. | Malaysia | July 2018 - May 2022" }</p>
                        <ul>
                            <li>
                                {
                                    "Developed an in-house manufacturing execution system (MES) for Lysa Technology Sdn. Bhd. (an electronics manufacturing
                                    sister company of Cadamsoft) to meet the requirements set by the company's multinational electric & 
                                    electronics client,  which allowed the company to begin carrying out business operations."
                                }
                            </li>
                            <li>
                                {
                                    "Ensured that the MES was in compliance with the stringent data retention and logging standards required of projects in the
                                    automotive industry, including those stated in ISO 9001:2015 and VDA 6.3 (a German automotive industry QA standard)."
                                }
                            </li>
                            <li>
                                {
                                    "Led a refactoring project of Lysa Technology's MES from a .NET Framework 4.0
                                    monolith to a RESTful API client-server architecture using ASP.NET Core for the backend, and Node.js + Typescript 
                                    for communication between the new backend and the legacy system, resulting in increased stability and performance 
                                    gains ranging from a few dozen percentage points to up to two orders of magnitude in certain cases."
                                }
                            </li>
                            <li>
                                {
                                    "Continually developed new MES features to handle new requirements from the client
                                    (often within the time frame of weeks), allowing Lysa Technology to take on  
                                    various manufacturing contracts of increasingly large volumes and turn profitable."
                                }
                            </li>
                            <li>
                                {
                                    "Assisted Lysa Technology's process and QA engineers in developing and improving manufacturing processes,
                                    which improved efficiency and allowed Lysa Technology to handle increasingly large manufacturing workloads."
                                }
                            </li>
                        </ul>
                    </section>
                </section>

                <section>
                    <h2>{ "Education" }</h2>
                    <section>
                        <h3>{ "Bachelor of Science in Biological Physics" }</h3>
                        <p class={ classes!("sub-h3") }>{ "Miami University | Oxford, Ohio, USA | August 2014 - May 2018" }</p>
                        <p>{ "GPA: 3.85 (4.00 scale)" }</p>
                    </section>
                </section>

                <section>
                    <h2>{ "Skills" }</h2>
                    <section>
                        <h3>{ "Technologies and Frameworks" }</h3>
                        <p>{ ".NET, ASP.NET Core, Docker, MySQL, Git, Node.js, AWS, REST, gRPC, OpenAPI" }</p>
                    </section>

                    <section>
                        <h3>{ "Languages" }</h3>
                        <p>{ "Rust, C#, Python, JavaScript, TypeScript, Lisp, SQL, Go" }</p>
                    </section>
                </section>

                <section>
                    <h2>{ "Trainings and Certifications" }</h2>
                    <ul>
                        <li>{ "ISO 9001:2015 Internal Audit Training" }</li>
                        <li>{ "VDA 6.3 Awareness Training" }</li>
                    </ul>
                </section>

                <section>
                    <h2>{ "Side Projects" }</h2>
                    <section>
                        <h3>{ "Indie Game (In-Progress)" }</h3>
                        <p class={ classes!("sub-h3") }>{ "September 2022 - now" }</p>
                        <p>
                            { "Developing a sci-fi tactics game entirely in Rust using "}
                            <a href="https://bevyengine.org/">{ "Bevy" }</a>
                            { ", an entity-component-system (ECS) game engine." }
                        </p>
                    </section>
                </section>
            </main>

            <footer>
                { "Created in Rust using " }
                <a href="https://yew.rs/">{ "Yew" }</a>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
