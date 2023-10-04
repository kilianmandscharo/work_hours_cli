# Work Hours CLI application

This is the basis of a CLI application that allows communication with [this server](https://github.com/kilianmandscharo/work_hours) to record and query one's work hours.

These are the available commands:

`block start` Start a new block of work

`block end` End the current block of work

`block delete {id}` Delete a block by ID

`block all` Show all blocks (will have a date range option in the future)

`block current` Show the current block

`block update {id} {start/end/homeoffice}` update the start or end date of a block as well as whether the work was remote or in person by ID, start and end have to be valid RFC3339 dates

`pause start` Start a pause in the current block of work

`pause end` End the current pause

`pause delete {id}` Delete a pause by ID

`pause update {id} {start/end}` Update the start or end date of a pause by ID, start and end have to be valid RFC3339 dates

`exit` Exit the application
