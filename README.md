![Banner](https://s-christy.com/status-banner-service/florence-announcement-system/banner-slim.svg)

## Overview

This is a simple announcement and news manager to help local government
officials distribute news to residents.

The end-goal is for this to be an API server for mobile apps, which is why the
main public endpoint is JSON. Security will be important, but as of writing has
not been built into the system.

## Screenshots

## Features

- Stores data in a database for persistence and reliable data recall
- Announcements automatically expire over time
- UI to easily transfer announcements between statuses
- Tags to sort, filter, and search for announcements of a specific type
- Recurring announcements that automatically trigger
- Schedule announcements to be sent at a future date
- UI for easy creation, modification, and deletion of data
- Serialization and deserialization of data to/from JSON for ease of transfer
- Server-side rendering of HTML
- SQL connection management for connecting to multiple databases
- Intelligent time period specifiers for recurring announcements
- Based on the high-performance Actix web server
- REST API for creating, updating, and deleting data
- Cookies for easy session management
- Hide/unhide announcement in the admin UI
- RSS feed generation
- Logging

## Usage

Start the server with `cargo run` and navigate to `localhost:8080` or
`localhost:8080/admin` in a web browser.

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)
