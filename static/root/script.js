let unpublished = document.querySelector("#announcements-unpublished");
let published = document.querySelector("#announcements-published");

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

function add_announcement() {
  let title = window.prompt("Please enter a title for the announcement:", "Some Title");
  let body = window.prompt("Please enter an annoucement body:", "Some Body");

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
