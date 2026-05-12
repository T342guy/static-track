# static-track
static-track is a project-by-project tracking tool I'll use in my future game studio\
static-track is named after my future studio: Staticflow studios.

# Software design document
Purpose: Project-by-project and init tracking, supporting multi-user configurations.
Lang: Rust
type: CLI tool
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
  - i need to actually know what this is tho u buffoon

# Planned commands
- `proj` - project details.
  - `init` - if no project is found, init one.
- `sit` - shows project lead
- `fuc` - i dunno i wanna make this a command
- `make` pre-append for notes
  - `oopsie`:`problem` - problem
  - `uhoh`:`big-prob` - big problem
  - `limbo` - limbo issue (one that needs to be fixed but might never get fixed)
- `statdo` - status, do this.
  - `wefucked` - panic project status: "paused work, In repair"
  - `wesoback` - were so back: "in progress, {alt}"
    - `burnrep` - Burnout repair: {alt:Post-burnout repair}
    - `prod`:`production` - Production stage: {alt:production}
    - `proto`:`prototype` - Prototyping stage: {alt:prototyping}
    - or none.
  - `Limbify`:`limbo` - Put the project into limbo and advise no work be done: "Limbo: do not work."
  - 