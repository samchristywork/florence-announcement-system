let announcements = document.querySelector("#announcements");

fetch("./announcements")
  .then(response => response.text())
  .then(data => announcements.innerHTML = data);
