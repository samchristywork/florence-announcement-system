let announcements = document.querySelector("#announcements");

document.querySelector("#checkbox")
  .addEventListener("change", function(e) {
    console.log(this.checked)
  });

fetch("./all/")
  .then(response => response.json())
  .then(data => {
    announcements.innerHTML=JSON.stringify(data);
  });
