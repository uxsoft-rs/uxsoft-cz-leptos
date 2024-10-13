use cv_button::CvButton;
use cv_icons::{CvEmailIcon, CvGitHubIcon, CvWebIcon};
use cv_job::CvJob;
use cv_page::CvPage;
use cv_page_title::CvPageTitle;
use cv_section::CvSection;
use cv_tag_list::CvTagList;
use leptos::*;

mod cv_button;
mod cv_icons;
mod cv_job;
mod cv_page;
mod cv_page_title;
mod cv_section;
mod cv_tag_list;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <CvPage>
            <CvPageTitle title="Jan Dryk">
                "I'm an experienced leader with a passion for technology.
                During the day, I'm leveraging my experience with software engineering and agile ways of working to shape
                teams into highly-performing product organization.
                At night I enjoy learning about new programming languages and frameworks and building small projects.
                This technical expertise helps me to be a better leader, and guide the developers to be more effective."
            </CvPageTitle>
            <CvSection title="Experience">
                <div style="display: grid; gap: 2rem">
                    <CvJob start="2023"
                           company="MSD"
                           title="Associate Director - Digital Marketing Engagement Delivery">
                        <ul class="list-disc">
                            <li>
                                "Supporting our product lines with setting up agile delivery
                                using DevSecOps & SCRUM"
                            </li>
                            <li>"Centralising delivery in VT"</li>
                            <li>
                                "Managing a team of 8 employees, leading delivery orchestrators"
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
                                    "Motivating and mentoring developers, hiring new developers"
                                </li>
                                <li>
                                    "Establishing mature tiered product management and
                                    architecture practices to support the delivery squads,
                                    setting up ways of working, upskilling team members"
                                </li>
                            </ul>
                        </ul>
                    </CvJob>
                    <CvJob start="Jun 2022"
                           end="Nov 2022"
                           company="MSD"
                           title="Associate Director - Marketing Execution Delivery">
                        <ul class="prose">
                            <li>
                                "Lead 6 agile delivery squads and a team of supporting roles
                                (delivery leads, scrum masters, engineering managers, ...)"
                            </li>
                            <li>
                                "Managed a budget of $6M, accountable for financial planning"
                            </li>
                            <li>
                                "Responsible for managing relationships with strategic partners"
                            </li>
                            <li>
                                "Responsible for E2E delivery, customer change management,
                                operations, release management"
                            </li>
                            <li>
                                "Pioneered DevSecOps, CI/CD using Jenkins, Artifactory,
                                Bitbucket, SonarQube in the organisation"
                            </li>
                            <li>
                                <i>"Extra Assignment: Content Tagging Automation (CTAP)"</i>
                                <ul class="list-disc ml-3">
                                    <li>"Rebuilt trust with the business stakeholders"</li>
                                    <li>"Significantly accelerated delivery, reorganized the team, coached team members and adopted agile ways of working"</li>
                                </ul>
                            </li>
                        </ul>
                    </CvJob>
                    <CvJob start="Jun 2020"
                           end="Jun 2022"
                           company="MSD"
                           title="Senior Delivery Lead - Marketing Execution">
                        <ul class="list-disc">
                            <li>"Lead 4 agile delivery squads"</li>
                            <li>
                                "Coordinated multiple product teams to deliver an E2E solution
                                to the customer as one team"
                            </li>
                            <li>"Lead an effort to change the ways of working of our product line to agile at scale"</li>
                            <li>"Responsible for delivery, quality, release management, and operations for the product line"</li>
                        </ul>
                    </CvJob>
                    <CvJob start="Mar 2020"
                           end="Jun 2020"
                           company="MSD"
                           title="Senior Technical Product Manager - Digital Identity & Consent">
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
                    </CvJob>
                    <CvJob start="Feb 2017"
                           end="Mar 2020"
                           company="MSD"
                           title="Solutions Architect - Digital Marketing & Channels">
                        <ul class="list-disc">
                            <li>
                                "Prepared an architecture vision for an integrated digital
                                marketing platform, gathered buy-in for that vision from all stakeholders,
                                and worked with developers to execute the vision"
                            </li>
                            <li>"Notable features included deeplinking, the front-end package,
                                real-time integrations for sychronizing data across systems, and more."</li>
                            <li>"Sped up the delivery of the Digital Identity product from 1 year per market to 1 month per market."</li>
                            <li>"Lead the MDE packaging program aimed at building a cross value team integrated marketing ecosystem with 5+ integrated products."</li>
                        </ul>
                    </CvJob>
                    <CvJob start="Sep 2016"
                           end="Feb 2017"
                           company="MSD"
                           title="Innovation Developer - Digital Marketing & Channels">
                        <ul class="list-disc">
                            <li>Developed innovative tools and integrations using AWS, TypeScript</li>
                            <li>Lead the effort to migrate ETL scripts from an EC2 to Lambdas</li>
                            <li>Evangelised the team to use TypeScript</li>
                        </ul>
                    </CvJob>
                </div>
            </CvSection>
            <CvSection title="Skills">
                <div style="display: grid; gap: 1rem">
                    <CvTagList
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
                    <CvTagList
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
                    <CvTagList
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
                                "JIRA".into(),
                                "Confluence".into(),
                            ]}
                    />
                </div>
            </CvSection>
            <CvSection title="Contact">
                <div class="flex flex-wrap gap-2 mb-10 mt-1">
                    <CvButton href="https://uxsoft.cz">
                        <CvWebIcon/>
                        "uxsoft.cz"
                    </CvButton>
                    <CvButton href="https://github.com/uxsoft">
                        <CvGitHubIcon/>
                        "github.com/uxsoft"
                    </CvButton>
                    <CvButton href="mailto:me@uxsoft.cz">
                        <CvEmailIcon/>
                        "me@uxsoft.cz"
                    </CvButton>
                </div>
            </CvSection>
        </CvPage>
    }
}
