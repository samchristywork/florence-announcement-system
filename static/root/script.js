let unpublished = document.querySelector("#announcements-unpublished");
let published = document.querySelector("#announcements-published");
let recurring = document.querySelector("#recurring");

fetch("./announcements/list/unpublished")
  .then(response => response.text())
  .then(data => {
    unpublished.innerHTML = data
  });

fetch("./announcements/list/published")
  .then(response => response.text())
  .then(data => {
    published.innerHTML = data
  });

fetch("./recurring/list")
  .then(response => response.text())
  .then(data => {
    recurring.innerHTML = data
  });

function add_recurring() {
  let title = window.prompt("Please enter a title for the recurring task:", "Some Title");
  let body = window.prompt("Please enter a recurring task body:", "Some Body");

  var d = new Date();
  var d_plus_five = new Date(d);
  d_plus_five.setDate(d_plus_five.getDate() + 5);

  const options = {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(
        {
          "id": Math.random().toString(16).substr(2, 8),
          "title": title,
          "body": body,
          "created": d.toLocaleString('en-US', { timeZone: 'America/Chicago' }) + " CT",
          "mode": "daily",
          "time_frame": "2:00",
        }
      )
  };

  fetch('./recurring/add', options)
      .then(response => response.text())
      .then(response => {
        location.reload();
      });
}

function add_announcement() {
  let title = window.prompt("Please enter a title for the announcement:", "Some Title");
  let body = window.prompt("Please enter an announcement body:", "Some Body");

  var d = new Date();
  var d_plus_five = new Date(d);
  d_plus_five.setDate(d_plus_five.getDate() + 5);

  const options = {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(
        {
          "title": title,
          "body": body,
          "created": d.toLocaleString('en-US', { timeZone: 'America/Chicago' }) + " CT",
          "scheduled": "--",
          "id": Math.random().toString(16).substr(2, 8),
          "status": "neutral",
          "expires": d_plus_five.toLocaleString('en-US', { timeZone: 'America/Chicago' }) + " CT",
        }
      )
  };

  fetch('./announcements/add', options)
      .then(response => response.text())
      .then(response => {
        location.reload();
      });
}

function recur_daily(id) {
  let answer = window.prompt("Enter time in 24-hour format (e.g. 14:30 for 2:30 pm)");
  answer = answer.split(":");

  if (answer.length != 2) {
    window.alert("Invalid input");
    return;
  }

  let d = new Date();
  d.setHours(answer[0]);
  d.setMinutes(answer[1]);

  if (d=="Invalid date") {
    window.alert("Invalid input");
    return;
  }

  window.alert(d);
}

function update_expiration(id) {
  var d = new Date();
  d = d.toLocaleString('en-US', { timeZone: 'America/Chicago' });
  var d_plus_five = new Date(d);
  d_plus_five.setDate(d_plus_five.getDate() + 5);

  let content = window.prompt("Please enter a new date Ex: (2023/10/12):", d_plus_five);
  content = new Date(content);
  content = content.toLocaleString('en-US', { timeZone: 'America/Chicago' }) + " CT";

  const options = {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(
        {
          "field": "expires",
          "content": content,
        }
      )
  };

  fetch('./announcements/update/'+id, options)
      .then(response => response.text())
      .then(response => {
        location.reload();
      });
}

function update_schedule(id) {
  var d = new Date();
  d = d.toLocaleString('en-US', { timeZone: 'America/Chicago' });

  let content = window.prompt("Please enter a new date Ex: (2023/10/12):", d);
  content = new Date(content);
  content = content.toLocaleString('en-US', { timeZone: 'America/Chicago' }) + " CT";

  const options = {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(
        {
          "field": "scheduled",
          "content": content,
        }
      )
  };

  fetch('./announcements/update/'+id, options)
      .then(response => response.text())
      .then(response => {
        location.reload();
      });
}

function update_announcement(id, field, current) {
  let content = window.prompt("Please enter a new " + field + " for the announcement:", current);
  const options = {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(
        {
          "field": field,
          "content": content,
        }
      )
  };

  fetch('./announcements/update/'+id, options)
      .then(response => response.text())
      .then(response => {
        location.reload();
      });
}

function set_state(id, state) {
  const options = {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(
        {
          "field": "status",
          "content": state,
        }
      )
  };

  fetch('./announcements/update/'+id, options)
      .then(response => response.text())
      .then(response => {
        location.reload();
      });
}

function delete_announcement(id) {
  const options = {
      method: 'POST',
  };

  if (window.confirm("Are you sure you want to delete this announcement?")) {
    fetch('./announcements/delete/'+id, options)
        .then(response => response.text())
        .then(response => {
          location.reload();
        });
  }
}
