use leptos::*;

mod cv_page;
mod cv_page_title;
mod cv_section;
mod cv_job;
mod cv_tag_list;
mod cv_button;
mod cv_icons;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <cv_page::CvPage>
            <cv_page_title::CvPageTitle title="Jan Dryk">
                "I'm an experienced leader with a passion for technology.
                During the day, I'm utilizing my experience with software engineering and agile ways of working to shape
                teams into highly-performing product organization.
                At night I enjoy learning about new programming languages and frameworks and building small projects.
                This technical expertise helps me to be a better leader, and guide the developers to be more effective."
            </cv_page_title::CvPageTitle>
            <cv_section::CvSection title="Experience">
                <div style="display: grid; gap: 2rem">
                    <cv_job::CvJob start="2023"
                           company="MSD"
                           title="Associate Director - Digital Marketing & Channels VT Delivery">
                        <ul class="list-disc">
                            <li>
                                "Supporting our product lines with setting up agile delivery
                                using DevSecOps & SCRUM"
                            </li>
                            <li>"Centralising delivery in VT"</li>
                            <li>
                                "Managing a team of 8 direct reports, leading delivery
                                orchestrators"
                            </li>
                            <li>
                                "Responsible for transformation of a struggling product line"
                            </li>
                            <ul class="list-disc ml-7">
                                <li>
                                    "Vendor management (fixing over-reliance on vendors, bringing
                                    key knowledge in house)"
                                </li>
                                <li>
                                    "Building high performance agile delivery squads (DevSecOps)"
                                </li>
                                <li>
                                    "Motivating and upskilling developers, hiring new developers"
                                </li>
                                <li>
                                    "Establishing mature tiered product management and
                                    architecture practices to support the delivery squads,
                                    setting up ways of working, upskilling team members"
                                </li>
                            </ul>
                        </ul>
                    </cv_job::CvJob>
                    <cv_job::CvJob start="2022"
                           end="2023"
                           company="MSD"
                           title="Associate Director - Marketing Execution Global Delivery">
                        <ul class="list-disc">
                            <li>
                                "Lead 6 agile delivery squads and a team of supporting roles
                                (delivery leads, scrum masters, engineering managers, ...)"
                            </li>
                            <li>
                                "Managed a budget of 6m USD, responsible for financial planning"
                            </li>
                            <li>
                                "Responsible for managing relationship with vendor partners"
                            </li>
                            <li>
                                "Responsible for E2E delivery, customer change management,
                                operations, release management"
                            </li>
                            <li>
                                "Pioneered DevSecOps, CI/CD using Jenkins, Artifactory,
                                Bitbucket, SonarQube in the organisation"
                            </li>
                        </ul>
                    </cv_job::CvJob>
                    <cv_job::CvJob start="2020"
                           end="2022"
                           company="MSD"
                           title="Senior Global Delivery Lead - Marketing Execution">
                        <ul class="list-disc">
                            <li>"Lead 4 agile delivery squads"</li>
                            <li>
                                "Coordinated multiple product teams to deliver an E2E solution
                                to the customer as one team"
                            </li>
                            <li>"Lead transformation for our team to agile at scale"</li>
                            <li>"Responsible for agile release management for the team"</li>
                        </ul>
                    </cv_job::CvJob>
                    <cv_job::CvJob start="2019"
                           end="2020"
                           company="MSD"
                           title="Senior Technical Product Owner - Digital Identity & Consent">
                        <ul class="list-disc">
                            <li>
                                "Lead an agile product team for a solution partly SaaS partly
                                developed on AWS"
                            </li>
                            <li>
                                "Created features, prioritised backlog, communicated with
                                stakeholders"
                            </li>
                        </ul>
                    </cv_job::CvJob>
                    <cv_job::CvJob start="2017"
                           end="2019"
                           company="MSD"
                           title="Solutions Architect - Digital Marketing & Channels">
                        <ul class="list-disc">
                            <li>
                                "Prepared an architecture vision for an integrated digital
                                marketing platform"
                            </li>
                            <li>"Influenced stakeholders to get buy in for the vision"</li>
                            <li>"Worked closely with developers to execute on the vision"</li>
                        </ul>
                    </cv_job::CvJob>
                    <cv_job::CvJob start="2016"
                           end="2017"
                           company="MSD"
                           title="Innovation Developer - Digital Marketing & Channels">
                        <ul class="list-disc">
                            <li>
                                "Developed innovative tools and integrations using AWS, TypeScript"
                            </li>
                        </ul>
                    </cv_job::CvJob>
                </div>
            </cv_section::CvSection>
            <cv_section::CvSection title="Skills">
                <div style="display: grid; gap: 1rem">
                    <cv_tag_list::CvTagList
                            title="Proficient"
                            tags={vec![
                                "F#".to_string(),
                                "C#".to_string(),
                                "ASP.NET Core".to_string(),
                                "WPF".to_string(),
                                "Entity Framework Core".to_string(),
                                "MAUI".to_string(),
                                "Umbraco".to_string(),
                                "Java".to_string(),
                                "LaTeX".to_string(),
                                "VB.NET".to_string(),
                                "SQL".to_string(),
                                "Javascript".to_string(),
                                "TypeScript".to_string(),
                                "React".to_string(),
                                "Astro".to_string(),
                                "UML".to_string(),
                                "Rust".to_string(),
                            ]}
                    />
                    <cv_tag_list::CvTagList
                            title="Familiar"
                            tags={vec![
                                "PHP".to_string(),
                                "Objective-C".to_string(),
                                "Matlab".to_string(),
                                "Protege".to_string(),
                                "Mathematica".to_string(),
                                "Pascal".to_string(),
                                "Kotlin".to_string(),
                                "Swift".to_string(),
                                "Python".to_string(),
                                "Machine Learning".to_string(),
                            ]}
                    />
                    <cv_tag_list::CvTagList
                            title="Services"
                            tags={vec![
                                "AWS".to_string(),
                                "Microsoft Azure".to_string(),
                                "Microsoft Azure DevOps".to_string(),
                                "Microsoft App Center".to_string(),
                                "GitHub".to_string(),
                                "Kubernetes".to_string(),
                                "Traefik".to_string(),
                                "Longhorn".to_string(),
                                "ArgoCD".to_string(),
                            ]}
                    />
                </div>
            </cv_section::CvSection>
            <cv_section::CvSection title="Contact">
                <div class="flex flex-wrap gap-2 mb-10 mt-1">
                    <cv_button::CvButton href="https://uxsoft.cz">
                        <cv_icons::CvWebIcon/>
                        uxsoft.cz
                    </cv_button::CvButton>
                    <cv_button::CvButton href="https://github.com/uxsoft">
                        <cv_icons::CvGitHubIcon/>
                        github.com/uxsoft
                    </cv_button::CvButton>
                    <cv_button::CvButton href="mailto:me@uxsoft.cz">
                        <cv_icons::CvEmailIcon/>
                        me@uxsoft.cz
                    </cv_button::CvButton>
                </div>
            </cv_section::CvSection>
        </cv_page::CvPage>
    }
}