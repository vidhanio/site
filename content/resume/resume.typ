#set page(margin: 0.5in)

#set list(indent: 1em)

#show heading.where(level: 1): it => {
  set align(center)
  set text(weight: "bold", size: 2em)
  it
}
#show heading: set text(weight: "semibold")
#show heading: smallcaps

#let subtitle(body) = {
  set align(center)
  set text(weight: "semibold", size: 1.5em)
  smallcaps(body)
}

#let underlink(dest, body) = underline(link(dest, body))

#let icon(width: 0.75em, name) = box(width: width, image("icons/" + name + ".svg"))

#let infos(..infos) = align(
  center,
)[
  #infos.pos().map(info => link(info.href)[#icon(info.icon) #info.body]).join([ | ])
]

#let section(title, body) = [
  #block(below: 0.5em)[== #title]
  #line(length: 100%)
  #block(above: 0.75em)[#body]
]

#let section-item(
  title,
  href: none,
  post-title: none,
  subtitle: none,
  start: none,
  end: none,
  body,
) = {
  let title = if href == none {
    title
  } else {
    [#link(href)[#icon(width: 0.7em, "link") #title]]
  }

  let post-title = if post-title == none {
    none
  } else {
    [ | _ #post-title _ ]
  }

  let date = if start == none and end == none {
    none
  } else if start == none {
    panic("cannot specify only end")
  } else {
    let end = if end == none {
      "Present"
    } else {
      end.display("[month repr:short]. [year]")
    }

    [#start.display("[month repr:short]. [year]") - #end]
  }

  [
    #block(below: 0.6em)[#box[=== #title] #post-title #h(1fr) #date]
    _ #subtitle _

    #body
  ]
}

= Vidhan Bhatt

#subtitle[Software Engineer]

#infos(
  (body: "me@vidhan.io", icon: "envelope", href: "mailto:me@vidhan.io"),
  (
    body: "/in/vidhanio",
    icon: "linkedin",
    href: "https://www.linkedin.com/in/vidhanio",
  ),
  (body: "vidhanio", icon: "github", href: "https://github.com/vidhanio"),
  (body: "vidhan.io", icon: "globe", href: "https://vidhan.io"),
)

#section(
  "Education",
)[
  #section-item(
    "McMaster University",
    post-title: "Hamilton, Ontario",
    subtitle: "Bachelor of Applied Science in Computer Science",
    start: datetime(year: 2022, month: 9, day: 1),
  )[
    - Achieved a 3.8 GPA in first year
    - Achieved an A+ in each computer science course
    - Relevant skills: *Python*, *Haskell*, *Java*, *Shell Scripting*, *C*, *Elm*,
      *GitHub*, *GitHub Actions*
  ]
]

#section(
  "Experience",
)[
  #section-item(
    "Tailered Sports",
    subtitle: "Discord Bot Developer",
    start: datetime(year: 2023, month: 5, day: 7),
  )[
    - Made a discord bot in *Rust* for users to make fake "bets" on baseball games
    - Accessed the FanDuel API to retrieve live betting odds
    - Created a live feed of the game using the MLB API
    - Used *PostgreSQL* to store user data and betting history
    - Also allowed the bettors to track their bets and append them to a spreadsheet
      nightly
  ]
]

#section(
  "Projects",
)[
  #section-item(
    "html-node",
    href: "https://github.com/vidhanio/html-node",
    post-title: "Rust, GitHub Actions",
    subtitle: "A library allowing for a jsx-like syntax in Rust for making server-rendered websites.",
    start: datetime(year: 2023, month: 7, day: 26),
  )[
    - Implements industry-standard testing and continuous integration using *GitHub
      Actions*
  ]

  #section-item(
    "MacEats",
    href: "https://github.com/vidhanio/maceats",
    post-title: "Rust, React, Next.js, Docker, Fly.io, Vercel",
    subtitle: "A full-stack modern reimplementation of MacEats, the McMaster University campus dining site.",
    start: datetime(year: 2022, month: 10, day: 17),
    end: datetime(year: 2022, month: 10, day: 31),
  )[
    - Created for the 2022 McMaster CSS Hacktober event
    - Made a #underlink("https://crates.io/crates/maceats")[*Rust* library] to scrape
      the original website and provide a programmatic interface to the data
    - Made a *Rust* backend to serve the data for general use by anyone
    - Made a *React* and *Next.js* frontend to access the modern site
    - Received 100% in judge voting, placing second overall in the competition
  ]

  #section-item(
    "Checkpoint",
    href: "https://github.com/vidhanio/checkpoint",
    post-title: "Go, Docker, GitHub Actions, Google Cloud Platform",
    subtitle: "A Discord verification bot for servers in the Peel District School Board.",
    start: datetime(year: 2021, month: 10, day: 6),
    end: datetime(year: 2022, month: 3, day: 1),
  )[
    - Made to combat rampant spam bots which would join servers in our school
    - Used *Go* for the Discord bot, which is deployed to *Google Compute Engine*
      using *GitHub Actions*
    - Was implemented in 15+ school club Discord servers, and received outstanding
      testimonials from the club executives
  ]
]

#section(
  "Technical Skills",
)[
  #section-item(
    "Programming Languages",
  )[
    Rust, Go, JavaScript, TypeScript, Python, Haskell, Java, Shell Scripting, HTML,
    CSS, C++, C, Elm
  ]

  #section-item("Frameworks/Databases")[
    React, Next.js, Node.js, Svelte, Tailwind CSS, SQL (PostgreSQL, SQLite,
    Supabase), MongoDB, Sled
  ]

  #section-item("Continuous Integration/Continuous Development")[
    Docker, GitHub, GitHub Actions, Kubernetes, Fly.io, Vercel, Google Cloud
    Platform
  ]
]

