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

      html+=`<div class='announcement-public'>
        <div class='date'>
          <div>Created: ${x.created}</div>
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
