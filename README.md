# Focustime

Focustime is a utility for time tracking based on your actively focused desktop window. It is currently in a very early stage and only implements the bare minimum: persisting focus change events for i3wm.

## Data model

#### Currently implemented:  
Every focus change stores a new Event, which is a timestamp with a reference to an Activity. The activity is the class of the window (eg `firefox`) along with its title.  

#### Todo:   
Categories are the things you actually want to track (ie your timesheet entries). Rules provide a mapping from Activity to Category based on some specified filter.  
Events (referencing Activities) are aggregated into Spans (referencing Categories). Spans are turned into the output: Reports, timesheets, etc. 

## Todo

* Ability to create Rules and Categories
* Implement applying rules, spans, and reports
* CLI/TUI frontend

## Nice to have

* Web frontend
* Support for other DE/WMs
* Support for other storage backends (influxdb?)
