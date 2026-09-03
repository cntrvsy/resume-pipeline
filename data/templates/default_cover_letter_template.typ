// 1. DATA & CONFIG
#import sys: inputs

// Fallback data for VS Code previewing
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
  cover_letter: (
    company: "Acme Corp",
    recipient: "Engineering Hiring Team",
    date: "September 2026",
    address: ("San Francisco, CA",),
    subject: "Application for Fullstack Engineer",
    paragraphs: (
      "When I read about Acme Corp's recent initiative to modernize its core data pipelines, it immediately resonated with my background in distributed systems. Having spent the last several years architecting high-throughput backend services in Rust and Go, I know firsthand the operational hurdles of scaling stateful workloads without compromising latency.",
      "At CloudScale Systems, I faced a similar scaling bottleneck across our microservices fleet. By designing an event-driven caching layer with Redis and profiling bottlenecks using Tokio metrics, my team reduced end-to-end request latency by 40% and improved peak throughput by 2.5x. I prioritize writing verifiable, zero-regression code with thorough integration testing and clear observability.",
      "Beyond technical execution, I thrive in collaborative engineering cultures that value clear communication and pragmatic decision-making. I would welcome the opportunity to discuss how my hands-on background in systems architecture and backend reliability can support Acme Corp's engineering roadmap.",
    ),
    sign_off: "Sincerely,",
  ),
)

#let letter_data = if "cover_letter" in inputs { inputs } else { fallback_data }
#let profile = letter_data.at("profile", default: fallback_data.profile)
#let cl = letter_data.at("cover_letter", default: fallback_data.cover_letter)
#let job_title = letter_data.at("job_title", default: "Software Engineer")

// STYLING DESIGN TOKENS (Shared with default_resume_template.typ)
#let primary_color = rgb("#111827")   // Near black for headings & signatures
#let accent_color = rgb("#1d4ed8")    // Professional crisp blue
#let secondary_color = rgb("#4b5563") // Slate gray for metadata/dates
#let body_color = rgb("#1f2937")      // Dark gray for high contrast readable body text

#set page(
  paper: "a4",
  margin: (x: 0.65in, top: 0.6in, bottom: 0.6in),
)

#set text(
  font: "Liberation Sans",
  lang: "en",
  size: 10pt,
  fill: body_color,
  weight: "regular",
)

#set par(leading: 0.55em, justify: false)

// 2. HEADER COMPONENT (Matching Resume Header)
#let header_component(profile, job_title) = {
  let contact_items = ()
  if profile.at("phone", default: "") != "" { contact_items.push(profile.phone) }
  if profile.at("email", default: "") != "" { contact_items.push(link("mailto:" + profile.email)[#profile.email]) }
  if profile.at("url", default: "") != "" { contact_items.push(link("https://" + profile.url)[#profile.url]) }
  if profile.at("website", default: "") != "" { contact_items.push(link("https://" + profile.website)[#profile.website]) }
  if profile.at("location", default: "") != "" { contact_items.push(profile.location) }
  if profile.at("citizenship", default: "") != "" { contact_items.push(profile.citizenship) }

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
  v(6pt)
  line(length: 100%, stroke: 0.6pt + rgb("#d1d5db"))
  v(10pt)
}

#header_component(profile, job_title)

// 3. RECIPIENT & METADATA SECTION
#grid(
  columns: (1fr, auto),
  [
    #if cl.at("recipient", default: "") != "" [
      #text(weight: "bold", fill: primary_color)[#cl.recipient] \
    ]
    #if cl.at("company", default: "") != "" [
      #text(weight: "semibold", fill: body_color)[#cl.company] \
    ]
    #if cl.at("address", default: ()) != () [
      #for line in cl.address [
        #text(fill: secondary_color, size: 9.5pt)[#line] \
      ]
    ]
  ],
  [
    #align(right)[
      #text(fill: secondary_color, size: 9.5pt)[#cl.at("date", default: "")]
    ]
  ],
)

#v(12pt)

// 4. SUBJECT LINE
#let subject_text = cl.at("subject", default: "")
#if subject_text != "" [
  #text(weight: "bold", size: 10.5pt, fill: primary_color)[#subject_text]
  #v(8pt)
] else if job_title != "" and job_title != "N/A" [
  #text(weight: "bold", size: 10.5pt, fill: primary_color)[Re: Application for #job_title]
  #v(8pt)
]

// 5. SALUTATION
#let recipient_name = cl.at("recipient", default: "Hiring Team")
#text(fill: body_color)[Dear #recipient_name,]

#v(8pt)

// 6. BODY PARAGRAPHS
#let paragraphs = cl.at("paragraphs", default: ())
#for (i, p) in paragraphs.enumerate() [
  #p
  #if i < paragraphs.len() - 1 [
    #v(9pt)
  ]
]

#v(14pt)

// 7. SIGN-OFF & SIGNATURE
#let sign_off_text = cl.at("sign_off", default: "Sincerely,")
#text(fill: body_color)[#sign_off_text]

#v(16pt)
#text(weight: "bold", size: 10.5pt, fill: primary_color)[#profile.name]
