let announcements = document.querySelector("#announcements");

document.querySelector("#checkbox")
  .addEventListener("change", function(e) {
    console.log(this.checked)
  });

fetch("./all/")
  .then(response => response.json())
  .then(data => {

    let html="";

    for (let x of data) {

      if (x.scheduled == "--") {
        x.scheduled = x.created;
      }
      html+=`<div class='announcement-public'>
        <div class='date'>
          <div>Date: ${x.scheduled}</div>
          <div>Expires: ${x.expires}</div>
        </div>
        <div style='display: grid; grid-template-columns: 1fr 1fr'>
          <div class='title'>${x.title}</div>
        </div>
        <div class='body'>${x.body}</div>
      </div>`
    }

    html+="<hr/>";

    announcements.innerHTML=html;
  });
