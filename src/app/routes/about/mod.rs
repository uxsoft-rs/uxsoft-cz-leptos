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
                "I'm an experienced leader with a passion for technology and learning.
                During the day, I leverage my experience with software engineering and agile ways of working to shape
                groups of individuals into highly-performing customer oriented product teams.
                At night I enjoy learning new programming languages and frameworks and building small projects.
                This technical expertise helps me to be a better leader, and guide the developers to be more effective."
            </CvPageTitle>
            <CvSection title="Experience">
                <div style="display: grid; gap: 2rem">
                    <CvJob start="Nov 2023"
                           company="MSD"
                           title="Associate Director - Digital Marketing Engagement Delivery">
                        <ul class="list-disc">
                            <li>"Reorganised the product line by value streams to focus on our customers."</li>
                            <li>"Built an internal team, internalized key knowledge and took ownership of the solution."</li>
                            <li>"Establed a mature tiered product management and architecture practices to support the delivery squads, adopted agile ways of working, mentored team members and colleagues."</li>
                            <li>"Lead a team of 10 employees, 50+ contractors."</li>
                            <li>"Managed a budget of ~$9M for delivery and ~$5M for operations."</li>
                            <li>"Established a mature release management process to address customer concerns about quality."</li>
                            <li>"Significantly increased the velocity of the team, helped the team to start make and meet commitments."</li>
                            <li>"Lead an effort to optimize operations costs by with ~$1M in cost savings."</li>
                        </ul>
                    </CvJob>
                    <CvJob start="Jun 2022"
                           end="Nov 2022"
                           company="MSD"
                           title="Associate Director - Marketing Execution Delivery">
                        <ul class="list-disc">
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
                                <ul class="list-disc ml-4">
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
                            <li>"Developed innovative tools and integrations using AWS, TypeScript"</li>
                            <li>"Lead the effort to migrate ETL scripts from an EC2 to Lambdas"</li>
                            <li>"Evangelised the team to use TypeScript"</li>
                        </ul>
                    </CvJob>
                </div>
            </CvSection>
            <CvSection title="Skills">
                <div style="display: grid; gap: 1rem">
                    <CvTagList
                            title="Proficient"
                            tags={vec![
                                "Rust",
                                "Astro",
                                "F#",
                                "C#",
                                "ASP.NET Core",
                                "WPF",
                                "Entity Framework Core",
                                "MAUI",
                                "Umbraco",
                                "Java",
                                "LaTeX",
                                "VB.NET",
                                "SQL",
                                "Javascript",
                                "TypeScript",
                                "React",
                                "UML",
                                "Swift",
                            ]}
                    />
                    <CvTagList
                            title="Familiar"
                            tags={vec![
                                "PHP",
                                "Objective-C",
                                "Matlab",
                                "Protege",
                                "Mathematica",
                                "Pascal",
                                "Kotlin",
                                "Python",
                                "Machine Learning",
                                "WordPress",
                            ]}
                    />
                    <CvTagList
                            title="Services"
                            tags={vec![
                                "AWS",
                                "Microsoft Azure",
                                "Microsoft Azure DevOps",
                                "Microsoft App Center",
                                "GitHub",
                                "Kubernetes",
                                "Traefik",
                                "Longhorn",
                                "ArgoCD",
                                "JIRA",
                                "Confluence",
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
