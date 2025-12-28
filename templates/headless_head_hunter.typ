// ==========================================
// 1. CONFIGURATION & STYLING RULES
// ==========================================

// Load the data from Rust input
#import sys: inputs
#let resume_data = inputs

#set page(
  paper: "us-letter",
  margin: (x: 1in, y: 0.75in),
)

#set text(
  font: "DejaVu Sans",
  lang: "en"
)

// Helper for body styling
#let body_style(content) = {
  set text(size: 10.5pt)
  set par(leading: 0.8em, justify: true)
  content
}

// ==========================================
// 2. COMPONENT FUNCTIONS
// ==========================================

// HEADER: Fixed by using [ ] content block
#let resume_header(profile) = [
  #set align(center)
  #set par(leading: 0.65em)

  #text(size: 14pt, weight: "bold")[#profile.name] \

  #v(-0.3em)

  #text(size: 12pt)[
    #profile.phone | #profile.email | #profile.url \
    #profile.citizenship at #profile.location
  ]
  #v(1em)
]

// SECTION TITLE
#let section_title(title) = [
  #v(0.5em)
  #text(size: 10.5pt, weight: "bold")[#title]
  #v(0.2em)
]

// EDUCATION ITEM
#let edu_item(degree, school, status) = [
  #grid(
    columns: (1fr, auto),
    align(left)[#degree from #school],
    align(right)[#status]
  )
]

// WORK ITEM
#let work_item(role, company, location, date, summary, highlights) = [
  #v(0.8em)

  #grid(
    columns: (1fr, auto),
    text(weight: "bold")[#role at #company, #location],
    text(style: "italic")[#date]
  )

  #if summary != "" [
    #v(0.2em)
    #summary
  ]

  #if highlights != none [
    #for point in highlights [
      #list(marker: [•], body-indent: 0.5em)[#point]
    ]
  ]
]

// ==========================================
// 3. RENDER THE RESUME
// ==========================================

// A. Header
#resume_header(resume_data.profile)

// B. Apply Body Styling to the rest
#show: body_style

// C. Education
#section_title("Education & Certificates")
#for edu in resume_data.education [
  #edu_item(edu.degree, edu.school, edu.status)
]

// D. Work History
#section_title("Work History")
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

// E. Projects
#if resume_data.projects != none [
  #section_title("Projects")
  #for proj in resume_data.projects [
    #work_item(
      proj.title,
      "Side Project",
      "",
      "",
      proj.description,
      proj.tech_stack
    )
  ]
]
