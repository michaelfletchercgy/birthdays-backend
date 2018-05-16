# Changelog

## Version 1

* You can add/update/remove birthdays.
* Notifications on upcoming birthdays.
* Birthdays display in upcoming order or alphabetical order..
* Is production and uat versions deployed over HTTPS.

## (MVP)


* Seems to require two builds to update the client (think it is the serviceworker?)
* Error tracking / monitoring.
* You can login, logout.
* You can register.
* You can change your password.
* Probably a good code cleanup.  Review "expects", "panics" and all uses of promises in js.
* https://github.com/brayniac/tic metrics
* https://prometheus.io/ to track stats and usage
* https://grafana.com/ to view
* Notices / Alerts with problems.
* Rebranch and get rid of any 'initial-version' branches.  Run off a released branch.

## Possible Future Changes / Roadmap

* Clean up the appearance.  I think a bunch of pieces of paper with menu would look nicer.  Age and room for notifiactions.
* Better error handling.
* Time until birthday
* Metrics / Measurements
* Partial loading of content (ie a large sub thing that consumes a bunch of something.)
* Age / turning.
* Highlight the milestone birthdays.
* Some ideas around partial saving / redo/undo would be cool.
* Link to "day in"
* Reminders (related fields)
* Share birthday (a complicated related value, with children, with searching, one to many with values on the relationship).
* (multi-level hierarchal values)