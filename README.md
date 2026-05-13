# static-track
static-track is a project-by-project tracking tool I'll use in my future game studio\
static-track is named after my future studio: Staticflow studios.

# Software design document
Purpose: Project-by-project and init tracking, supporting multi-user configurations.\
Lang: Rust\
type: CLI tool\
## Overview
static-track is a CLI project-by-project tracking tool that allows users to add notes, statuses, positions and so forth.
Uses a central configuration on the host and secondary configuration and MD files. 
Allows users to add milestones, dates, deadlines, known issues and holdups, plus project statuses (given perms granted). 

static-track will be used in the future by my up-and-coming game studio for tracking project-by-project.

## (planned) Features
- User history
- project statuses
- update notes
- issue/bugs and other
- auto-updated Markdown overview file.
  - Overview Markdown file will contain status, last update made (and by whom), plus project info.
- "grandfathering" tool for when project leads leave.
  - I need to actually know what this is tho u buffoon

# Planned commands
- `proj` - shows project details.
  - `all` - shows all the project details.
- `init` - if no project is found, init one. If it is found, skip. Brings user through the setup and appends them as director.
- `lead` - shows project lead(s)
- `make` "Make" exportable files inside a project.
  - `md` - Make a Markdown file.
  - `pdf` - Make a PDF.
- `statdo` - status, do this.
- `verify` - Verifies project and command files for unauthorized changes.