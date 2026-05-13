# static-track

# Overview
static-track is a CLI-based project-by-project, non-network management tool in the idea stage for my future game studio.\
The goal for this tool is to allow users who are in a free-floating management structure to have easy access to "signed up" work they may have forgotten about,\
to view work's status, and automatically generate detailed project information without requiring meetings or complex communications in a structureless team. (flat management structure)

Do you want to use this tool? Go for it! Its open source for a reason. If you have ideas, please dont fear to make a pull request!
## (planned) Features
- Fully off-network, relies on local file histories.
- Active persons on the project
  - Automatic booting for inactivity. aka, auto-leave. This will be an automatic action.
- project status change limits
- auto-checking project system file checking for fake edits. (like changing the proj director, there will be a hash to make sure its correct.)
- limbofy: if a project has had no interactions for up to 42 days, a warning will be sent to the user that it has been limbofied. (this will be an automatic action the next time the application is run.)
### Auto-leave
If a project member has been added, and the history of the project shows they have not made a change in 30 days, the program will automatically remove them from the project.

# Planned commands
- `proj` - shows project details.
  - `all` - shows all the project details.
- `init` - if no project is found, init one. If it is found, skip. Brings user through the setup and appends them as director.
- `lead` - shows project lead(s)
- `make` "Make" exportable files inside a project.
  - `md` - Make a Markdown file.
  - `pdf` - Make a PDF.
- `statdo` - status, do this.
  - `prod`
  - `proto`
  - `preprod`
  - `testing`
  - `assembly`
- `verify` - Verifies project and command files for unauthorized changes.