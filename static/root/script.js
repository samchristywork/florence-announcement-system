let unpublished = document.querySelector("#announcements-unpublished");
let published = document.querySelector("#announcements-published");

fetch("./announcements/list")
  .then(response => response.text())
  .then(data => {
    unpublished.innerHTML = data
    published.innerHTML = data
  });

