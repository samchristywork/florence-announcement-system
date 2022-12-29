let unpublished = document.querySelector("#announcements-unpublished");
let published = document.querySelector("#announcements-published");

fetch("./announcements/list")
  .then(response => response.text())
  .then(data => {
    unpublished.innerHTML = data
    published.innerHTML = data
  });

function add_announcement() {
  let title = window.prompt("Please enter a title for the announcement:", "Some Title");
  let body = window.prompt("Please enter an annoucement body:", "Some Body");

  var d = new Date();
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
        }
      )
  };

  fetch('./announcements/add', options)
      .then(response => response.text())
      .then(response => {
        location.reload();
      });
}

