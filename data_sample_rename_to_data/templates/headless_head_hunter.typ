// 1. DATA & CONFIG
#import sys: inputs

// Define fallback data for when editing in VS Code / Previewing
#let fallback_data = (
  profile: (
    name: "Preview User",
    email: "user@example.com",
    phone: "555-0100",
    url: "linkedin.com/in/preview",
    location: "City, Country",
    citizenship: "Citizen"
  ),
  job_title: "Preview Job Title",
  education: (
    (school: "University of Preview", degree: "B.Sc. Computer Science", status: "Graduated"),
  ),
  prof: (
    (title: "Blockchain", details: "Bitcoin (Ordinals, Taproot), EVM (Solidity, Ethers.js, Wagmi/Viem), Cross-Chain Bridges, Multi-platform Wallet Development."),
  ),
  skills: (
    (title: "Programming Languages", details: "Rust, TypeScript, Python"),
  ),
  experience: (
    (
      role: "Senior Developer",
      company: "Tech Corp",
      location: "Remote",
      date: "Jan 2020 - Present",
      summary: "A placeholder summary for preview mode.",
      bullets: ("Built amazing things", "Optimized performance")
    ),
  ),
  projects: ()
)

// Logic: If 'profile' is inside inputs (comes from Rust), use inputs.
// Otherwise, use the fallback data (Preview Mode).
#let resume_data = if "profile" in inputs { inputs } else { fallback_data }
// -------
#set page(
  paper: "us-letter",
  margin: (x: 0.75in, y: 0.5in),
)

// GLOBAL RESET:
// 1. Force the font you embedded (Liberation Sans)
// 2. Force Black (prevents fading)
// 3. Force Regular (prevents bold leaks)
#set text(
  font: "Liberation Sans",
  lang: "en",
  size: 10.5pt,
  fill: black,
  weight: "regular"
)

#set par(leading: 0.6em, justify: true)

// 2. COMPONENTS

// Header: Name is the ONLY thing bold
#let header_component(profile) = {
  align(center)[
    #text(size: 14pt, weight: "bold")[#profile.name]
    #v(-0.3em)
    #text(size: 9pt)[ // Inherits Regular
      #profile.email | #profile.url | #profile.location
      // #profile.phone | #profile.email | #profile.url \
      // #profile.citizenship at #profile.location
    ]
  ]
  v(1em)
}

// Job Title: Bold
#let job_title_component(title) = {
  align(center)[
    #v(-0.5em)
    #text(size: 12pt, weight: "bold", fill: black)[#title]
    // #v(1em)
  ]
}

// Section Title: Bold
#let section_title(title) = {
  v(0.5em)
  // text(size: 10.5pt, weight: "bold")[#title]
  text(size: 12pt, weight: "bold")[#title]
  v(0.2em)
}

// Education Item: Strictly Regular
#let edu_item(degree, school, status) = {
  grid(
    columns: (1fr, auto),
    [#degree from #school],
    text(style: "italic")[#status]
  )
  v(0.3em)
}

// Prof Profile Item
#let prof_profile_item(title, details) = {
  grid(
    columns: (1fr, auto),
    [#text(weight: "bold", size: 10pt)[#title]: #text(size: 10pt)[#details]]
  )
  v(-0.5em)
}

// Skill Item
#let skill_item(title, details) = {
  grid(
    columns: (1fr, auto),
    [#text(weight: "bold", size: 10pt)[#title]: #text(size: 10pt)[#details]]
  )
  v(-0.5em)
}

// Work Item: Strictly Regular
#let work_item(role, company, location, date, summary, highlights) = {
  grid(
    columns: (1fr, auto),
    [#text(weight: "bold")[#role] #text(style: "italic")[at #company, #location]],
    text(style: "italic")[#date]
  )

  if summary != "" {
    v(0.2em)
    summary
  }

  if highlights != none {
    v(-0.2em)
    for point in highlights {
      // list(marker: [•], body-indent: 0.3em)[#point]
      list(marker: [•], body-indent: 0.9em)[#point]
      v(-0.5em)
    }
  }

  v(0.75em)
}

// 3. RENDER

#header_component(resume_data.profile)

#job_title_component(resume_data.at("job_title", default: "Software Engineer"))

#v(0.2em)
#section_title("Professional Profile")
#v(-0.3em)

#for itm in resume_data.prof [
  #prof_profile_item(itm.title, itm.details)
]

#v(0.5em)
#section_title("Skills")
#v(-0.3em)
#for itm in resume_data.skills [
  #skill_item(itm.title, itm.details)
]

#v(1em)
#section_title("Work History")
#v(-0.1em)
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

#section_title("Education & Certificates")
#for edu in resume_data.education [
  #edu_item(edu.degree, edu.school, edu.status)
]

// #if resume_data.projects != none [
//   #section_title("Projects")
//   #for proj in resume_data.projects [
//     #work_item(
//       proj.title,
//       "Side Project",
//       "",
//       "",
//       proj.description,
//       proj.tech_stack
//     )
//   ]
// ]
