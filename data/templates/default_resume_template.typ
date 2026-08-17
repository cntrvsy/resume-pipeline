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
  job_title: "Fullstack Engineer",
  professional_summary: "Highly skilled Software Engineer with 5+ years of experience in full-stack development. Proven ability to design and implement scalable solutions using Rust, Go, and React. Strong focus on performance optimization and system architecture.",
  skills: (
    Languages: ("TypeScript", "Rust", "Python", "SQL"),
    "Frontend & UI": ("React", "Svelte", "Tailwind CSS"),
    "Backend & APIs": ("Node.js", "Hono", "REST", "gRPC"),
    "Tools & Infra": ("Docker", "Vercel", "Git", "PostgreSQL")
  ),
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

// STYLING DESIGN TOKENS
#let accent_color = rgb("#0f766e") // Deep Teal accent
#let secondary_color = rgb("#475569") // Slate 600 for metadata & dates
#let body_color = rgb("#1e293b") // Slate 800 dark charcoal for text contrast
#let rule_color = rgb("#e2e8f0") // Slate 200 light divider lines
#let pill_bg = rgb("#f1f5f9") // Slate 100 soft tag background
#let pill_border = rgb("#cbd5e1") // Slate 300 tag border

#set page(
  paper: "a4",
  margin: (x: 0.6in, y: 0.5in),
)

#set text(
  font: "Liberation Sans",
  lang: "en",
  size: 9.5pt,
  fill: body_color,
  weight: "regular"
)

#set par(leading: 0.55em, justify: false)

// 2. COMPONENTS

#let header_component(profile, job_title) = {
  let contact_items = ()
  if profile.phone != "" { contact_items.push(profile.phone) }
  if profile.email != "" { contact_items.push(link("mailto:" + profile.email)[#profile.email]) }
  if profile.url != "" { contact_items.push(link("https://" + profile.url)[#profile.url]) }
  if profile.website != "" { contact_items.push(link("https://" + profile.website)[#profile.website]) }
  let location_str = (
    if profile.location != "" { profile.location },
    if profile.citizenship != "" { profile.citizenship }
  ).filter(it => it != none).join(" • ")
  if location_str != "" { contact_items.push(text(fill: secondary_color)[#location_str]) }

  grid(
    columns: (1fr, auto),
    column-gutter: 1.5em,
    align: (left + horizon, right + horizon),
    [
      #text(size: 20pt, weight: "bold", fill: accent_color)[#profile.name] \
      #v(2pt)
      #if job_title != "" and job_title != "N/A" [
        #text(size: 11pt, weight: "bold", fill: secondary_color, tracking: 0.04em)[#upper(job_title)]
      ]
    ],
    align(right)[
      #set text(size: 8.5pt, fill: body_color)
      #contact_items.join([ \ ])
    ]
  )
  v(4pt)
  line(length: 100%, stroke: 1.2pt + accent_color)
  v(6pt)
}

#let summary_component(summary) = {
  if summary != "" and summary != "N/A" {
    block(
      width: 100%,
      fill: rgb("#f8fafc"),
      inset: (x: 10pt, y: 8pt),
      radius: 4pt,
      stroke: (left: 3pt + accent_color)
    )[
      #text(size: 9.25pt, style: "italic", fill: body_color)[#summary]
    ]
    v(6pt)
  }
}

#let section_title(title) = {
  v(8pt)
  block(width: 100%, inset: (bottom: 3pt), stroke: (bottom: 0.8pt + accent_color))[
    #grid(
      columns: (auto, 1fr),
      gutter: 6pt,
      align: (left + horizon, left + horizon),
      box(fill: accent_color, width: 3.5pt, height: 10pt, radius: 1pt),
      text(size: 10pt, weight: "bold", fill: accent_color, tracking: 0.05em)[#upper(title)]
    )
  ]
  v(4pt)
}

#let skills_component(skills) = {
  if skills != none and skills != (:) {
    section_title("Technical Skills")
    for (category, items) in skills {
      grid(
        columns: (110pt, 1fr),
        column-gutter: 10pt,
        align: (left + top, left + top),
        text(weight: "bold", fill: accent_color)[#category],
        items.map(item => box(
          fill: pill_bg,
          stroke: 0.4pt + pill_border,
          inset: (x: 5pt, y: 2.5pt),
          radius: 3pt
        )[#text(size: 8.5pt, weight: "medium", fill: body_color)[#item]]).join(" ")
      )
      v(4pt)
    }
    v(2pt)
  }
}

#let edu_item(degree, school, status) = {
  grid(
    columns: (1fr, auto),
    column-gutter: 1.5em,
    [#strong(school) #if degree != "" [ — #degree]],
    text(style: "italic", fill: secondary_color)[#status]
  )
  v(4pt)
}

#let work_item(role, company, location, date, summary, highlights, url: none) = {
  grid(
    columns: (1fr, auto),
    column-gutter: 1.5em,
    [
      #text(weight: "bold", size: 10pt, fill: body_color)[#role]
      #if company != "" [ #text(fill: accent_color, weight: "semibold")[| #company] #if location != "" [, #text(style: "italic", fill: secondary_color)[#location]]]
      #if url != none [ | #link("https://" + url)[#text(fill: accent_color)[#url]] ]
    ],
    text(style: "italic", weight: "medium", fill: secondary_color)[#date]
  )

  if summary != "" {
    v(2pt)
    summary
  }

  if highlights != none and highlights != () {
    v(2pt)
    set list(marker: text(fill: accent_color, size: 7pt)[#sym.bullet], indent: 0.8em, body-indent: 0.4em, spacing: 0.35em)
    for point in highlights {
      list.item[#point]
    }
  }

  v(6pt)
}

#let project_item(title, description, tech_stack, url: none) = {
  grid(
    columns: (1fr, auto),
    column-gutter: 1.5em,
    [
      #text(weight: "bold", size: 10pt, fill: body_color)[#title]
    ],
    [
      #if url != none [ #link("https://" + url)[#text(size: 8.5pt, fill: accent_color)[#url]] ]
    ]
  )

  if description != "" {
    v(2pt)
    description
  }

  if tech_stack != none and tech_stack != () {
    v(3pt)
    text(size: 8pt, weight: "semibold", fill: secondary_color)[Stack: ]
    tech_stack.map(tech => box(
      fill: pill_bg,
      stroke: 0.4pt + pill_border,
      inset: (x: 4pt, y: 1.5pt),
      radius: 2pt
    )[#text(size: 8pt, fill: body_color)[#tech]]).join(" ")
  }

  v(6pt)
}

// 3. RENDER

#header_component(
  resume_data.profile,
  resume_data.at("job_title", default: "Software Engineer")
)

#summary_component(resume_data.at("professional_summary", default: ""))

#skills_component(resume_data.at("skills", default: none))

#if resume_data.at("experience", default: ()) != () [
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
]

#if resume_data.at("projects", default: ()) != () [
  #section_title("Projects")
  #for proj in resume_data.projects [
    #project_item(
      proj.title,
      proj.description,
      proj.tech_stack,
      url: proj.at("url", default: none)
    )
  ]
]

#if resume_data.at("education", default: ()) != () [
  #section_title("Education")
  #for edu in resume_data.education [
    #edu_item(edu.degree, edu.school, edu.status)
  ]
]