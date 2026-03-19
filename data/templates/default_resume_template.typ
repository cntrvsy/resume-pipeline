// 1. DATA & CONFIG
#import sys: inputs

// Define fallback data for when editing in VS Code / Previewing
#let fallback_data = (
  profile: (
    name: "Alex River",
    email: "alex.river@example.com",
    phone: "+1 (555) 123-4567",
    url: "linkedin.com/in/alexriver",
    website: "alexriver.dev",
    location: "San Francisco, CA",
    citizenship: "US Citizen"
  ),
  job_title: "Full Stack Software Engineer",
  professional_summary: "Highly skilled Software Engineer with 5+ years of experience in full-stack development. Proven ability to design and implement scalable solutions.",
  education: (
    (school: "University of Tech", degree: "B.Sc. Computer Science", status: "Graduated May 2019"),
  ),
  experience: (
    (
      role: "Senior Developer",
      company: "CloudScale Systems",
      location: "Remote",
      date: "Jan 2022 - Present",
      summary: "Leading the core infrastructure team to migrate legacy monolithic services into a distributed microservices architecture.",
      bullets: (
        "Reduced system latency by 40% using Redis caching strategies.",
        "Mentored 4 junior developers and established CI/CD best practices.",
      )
    ),
  ),
  // EXPANDED FAKE PROJECT DATA
  projects: (
    (
      title: "Distributed Task Scheduler",
      description: "A high-performance Go-based task scheduler capable of handling 10k+ concurrent jobs with automated retry logic.",
      tech_stack: ("Go", "gRPC", "PostgreSQL", "Docker"),
      url: "github.com/ariver/task-master"
    ),
    (
      title: "AI Semantic Search Engine",
      description: "Built a vector-based search interface for technical documentation using Python and Pinecone.",
      tech_stack: ("Python", "OpenAI API", "React", "Tailwind CSS"),
      url: "search-demo.alexriver.dev"
    ),
    (
      title: "Real-time Analytics Dashboard",
      description: "A data visualization platform for monitoring IoT sensor telemetry with sub-second refresh rates.",
      tech_stack: ("Next.js", "Apache Kafka", "ClickHouse", "D3.js"),
      url: "analytics.river.io"
    ),
    (
      title: "P2P File Transfer Protocol",
      description: "Implemented a custom peer-to-peer file sharing protocol with end-to-end encryption and NAT traversal.",
      tech_stack: ("Rust", "Libp2p", "Tokio", "Protobuf"),
      url: "github.com/ariver/rust-p2p"
    )
  )
)

#let resume_data = if "profile" in inputs { inputs } else { fallback_data }

#set page(
  paper: "us-letter",
  margin: (x: 0.75in, y: 0.6in),
)

#set text(
  font: "Liberation Sans",
  lang: "en",
  size: 10pt,
  fill: black,
  weight: "regular"
)

#set par(leading: 0.55em, justify: true)

// 2. COMPONENTS

#let header_component(profile) = {
  align(center)[
    #text(size: 16pt, weight: "bold")[#profile.name]
    #v(4pt)
    #text(size: 10pt)[
      #profile.phone | #link("mailto:" + profile.email)[#profile.email] | #link("https://" + profile.url)[Linked In] | #link("https://" + profile.website)[#profile.website] \
      #profile.citizenship #sym.bullet #profile.location
    ]
  ]
  v(12pt)
}

#let job_title_component(title) = {
  align(center)[
    #v(-8pt)
    #text(size: 11pt, weight: "bold", fill: black)[#title] // Removed .upper()
    #v(8pt)
  ]
}

#let summary_component(summary) = {
  if summary != "" and summary != "N/A" {
    align(left)[
      #text(style: "normal", size: 10pt)[#summary]
      #v(8pt)
    ]
  }
}

#let section_title(title) = {
  v(10pt)
  block(width: 100%, stroke: (bottom: 0.5pt + gray))[
    #text(size: 10pt, weight: "bold")[#title] // Removed .upper()
  ]
  v(4pt)
}

#let edu_item(degree, school, status) = {
  grid(
    columns: (1fr, auto),
    column-gutter: 2em, // Increased gutter for breathing room
    [#strong(school)],
    text(style: "italic")[#status]
  )
  [#degree]
  v(6pt)
}

#let work_item(role, company, location, date, summary, highlights, url: none) = {
  grid(
    columns: (1fr, auto),
    column-gutter: 2em, // Ensures text doesn't hit the date
    [
      #strong(role) #if company != "" [| #text(style: "italic")[#company, #location]]
      #if url != none [ | #link("https://" + url)[#url] ]
    ],
    text(style: "italic")[#date]
  )

  if summary != "" {
    v(2pt)
    summary
  }

  if highlights != none {
    set list(indent: 1.2em, body-indent: 0.5em, spacing: 0.4em)
    for point in highlights {
      list.item[#point]
    }
  }

  v(8pt)
}

// 3. RENDER

#header_component(resume_data.profile)

#job_title_component(resume_data.at("job_title", default: "Software Engineer"))

#summary_component(resume_data.at("professional_summary", default: ""))

#section_title("Education")
#for edu in resume_data.education [
  #edu_item(edu.degree, edu.school, edu.status)
]

#section_title("Work Experience")
#for job in resume_data.experience [
  #work_item(
    job.role,
    job.company,
    job.location,
    job.date,
    job.summary,
    job.bullets
  )
]

#if resume_data.projects != () [
  #section_title("Projects")
  #for proj in resume_data.projects [
    #work_item(
      proj.title,
      "", 
      "",
      "", 
      proj.description,
      proj.tech_stack,
      url: proj.at("url", default: none)
    )
  ]
]