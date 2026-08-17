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
    citizenship: "US Citizen",
  ),
  job_title: "Fullstack Engineer",
  professional_summary: "Highly skilled Software Engineer with 5+ years of experience in full-stack development. Proven ability to design and implement scalable solutions using Rust, Go, and React. Strong focus on performance optimization and system architecture.",
  skills: (
    Languages: ("TypeScript", "Rust", "Python", "SQL"),
    "Frontend & UI": ("React", "Svelte", "Tailwind CSS"),
    "Backend & APIs": ("Node.js", "Hono", "REST", "gRPC"),
    "Tools & Infra": ("Docker", "Vercel", "Git", "PostgreSQL"),
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
      ),
    ),
  ),
  projects: (
    (
      title: "Distributed Task Scheduler",
      description: "A high-performance Go-based task scheduler capable of handling 10k+ concurrent jobs with automated retry logic.",
      tech_stack: ("Go", "gRPC", "PostgreSQL", "Docker"),
      url: "github.com/ariver/task-master",
    ),
    (
      title: "AI Semantic Search Engine",
      description: "Built a vector-based search interface for technical documentation using Python and Pinecone.",
      tech_stack: ("Python", "OpenAI API", "React", "Tailwind CSS"),
      url: "search-demo.alexriver.dev",
    ),
  ),
)

#let resume_data = if "profile" in inputs { inputs } else { fallback_data }

// STYLING DESIGN TOKENS (Clean, professional corporate palette)
#let primary_color = rgb("#111827") // Near black for strong headings
#let accent_color = rgb("#1d4ed8")  // Professional crisp blue (or use #0f766e for deep teal)
#let secondary_color = rgb("#4b5563") // Slate gray for metadata/dates
#let body_color = rgb("#1f2937") // Dark gray for high contrast readable body text

#set page(
  paper: "a4",
  margin: (x: 0.5in, y: 0.5in),
)

#set text(
  font: "Liberation Sans",
  lang: "en",
  size: 9.5pt,
  fill: body_color,
  weight: "regular",
)

#set par(leading: 0.45em, justify: false)

// 2. COMPONENTS

#let header_component(profile, job_title) = {
  let contact_items = ()
  if profile.phone != "" { contact_items.push(profile.phone) }
  if profile.email != "" { contact_items.push(link("mailto:" + profile.email)[#profile.email]) }
  if profile.url != "" { contact_items.push(link("https://" + profile.url)[#profile.url]) }
  if profile.website != "" { contact_items.push(link("https://" + profile.website)[#profile.website]) }
  if profile.location != "" { contact_items.push(profile.location) }
  if profile.citizenship != "" { contact_items.push(profile.citizenship) }

  align(center)[
    #text(size: 22pt, weight: "bold", fill: primary_color)[#profile.name] \
    #v(3pt)
    #if job_title != "" and job_title != "N/A" [
      #text(size: 11pt, weight: "semibold", fill: accent_color, tracking: 0.03em)[#job_title] \
      #v(4pt)
    ]
    #text(size: 8.5pt, fill: secondary_color)[
      #contact_items.join("  |  ")
    ]
  ]
  v(8pt)
}

#let summary_component(summary) = {
  if summary != "" and summary != "N/A" {
    v(2pt)
    text(size: 9.5pt, fill: body_color)[#summary]
    v(4pt)
  }
}

#let section_title(title) = {
  v(10pt)
  text(size: 10.5pt, weight: "bold", fill: accent_color, tracking: 0.04em)[#upper(title)]
  v(2pt)
  line(length: 100%, stroke: 0.6pt + rgb("#d1d5db"))
  v(4pt)
}

#let skills_component(skills) = {
  if skills != none and skills != (:) {
    section_title("Technical Skills")
    let category_blocks = ()
    for (category, items) in skills {
      category_blocks.push([#text(weight: "bold", fill: primary_color)[#category:] #items.join(", ")])
    }
    category_blocks.join([ #text(fill: accent_color, size: 6.5pt)[#sym.bullet] ])
  }
}

#let edu_item(degree, school, status) = {
  grid(
    columns: (1fr, auto),
    column-gutter: 1.5em,
    [#text(weight: "bold", fill: primary_color)[#school] #if degree != "" [— #text(fill: body_color)[#degree]]],
    text(fill: secondary_color)[#status],
  )
}

#let work_item(role, company, location, date, summary, highlights, url: none) = {
  v(4pt)
  grid(
    columns: (1fr, auto),
    column-gutter: 1.5em,
    [
      #text(weight: "bold", size: 10pt, fill: primary_color)[#role]
      #if company != "" [ #text(fill: accent_color, weight: "semibold")[\@ #company] #if (
          location != ""
        ) [, #text(fill: secondary_color)[#location]]]
    ],
    text(weight: "medium", fill: secondary_color)[#date],
  )

  if summary != "" {
    v(1pt)
    text(fill: body_color)[#summary]
  }

  if highlights != none and highlights != () {
    v(2pt)
    set list(
      marker: text(fill: accent_color, size: 6pt)[#sym.bullet],
      indent: 0.8em,
      body-indent: 0.4em,
      spacing: 0.3em,
    )
    for point in highlights {
      list.item[#point]
    }
  }
}

#let project_item(title, description, tech_stack, url: none) = {
  v(4pt)
  grid(
    columns: (1fr, auto),
    column-gutter: 1.5em,
    [
      #text(weight: "bold", size: 10pt, fill: primary_color)[#title]
      #if tech_stack != none and tech_stack != () [
        #text(fill: secondary_color)[ (#tech_stack.join(", "))]
      ]
    ],
    [
      #if url != none [ #link("https://" + url)[#text(size: 8.5pt, fill: accent_color)[#url]] ]
    ],
  )

  if description != "" {
    v(1pt)
    text(fill: body_color)[#description]
  }
}

// 3. RENDER

#header_component(
  resume_data.profile,
  resume_data.at("job_title", default: "Software Engineer"),
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
      job.bullets,
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
      url: proj.at("url", default: none),
    )
  ]
]

#if resume_data.at("education", default: ()) != () [
  #section_title("Education")
  #for edu in resume_data.education [
    #edu_item(edu.degree, edu.school, edu.status)
  ]
]
