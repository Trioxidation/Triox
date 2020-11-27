'use strict';

function new_list_entry(name, type, date = "never") {
    const row = document.createElement("tr");

    const checkbox_td = document.createElement("td");

    checkbox_td.innerHTML = `
<label>
  <input type="checkbox">
</label>`;

    row.appendChild(checkbox_td);


    const name_td = document.createElement("td");

    if (type == "File") {
        name_td.innerHTML = `
<a href="/app/files/get/${name}" download>
  ${name}
</a>`
    } else {
        name_td.innerHTML = `
<a href="/app/files/get/${name}" download>
  ${name}
</a>`
    }
    row.appendChild(name_td);


    const type_td = document.createElement("td");
    type_td.innerText = type;
    row.appendChild(type_td);


    const date_td = document.createElement("td");
    date_td.innerText = date;
    row.appendChild(date_td);


    const dropdown_td = document.createElement("td");

    const drop_down_html = `
<div class="dropdown" onclick="toggle_dropdown(event)">
  <div class="dropdown-trigger">
    <span class="icon">
      <img src="/static/icons/keyboard_arrow_down.svg"></img>
    </span>
  </div>
  <div class="dropdown-menu" id="dropdown-menu" role="menu">
    <div class="dropdown-content">
      <a class="dropdown-item">
        Rename
      </a>
      <a class="dropdown-item">
        Copy
      </a>
      <a class="dropdown-item">
        Move
      </a>
      <a class="dropdown-item">
        Delete
      </a>
    </div>
  </div>
</div>`;

    dropdown_td.innerHTML = drop_down_html;

    row.appendChild(dropdown_td);

    return row;
}


window.addEventListener('load', load_files);

function rename_dialoque(name) {
    const new_name = prompt(`Rename file ${name}`, name);

    if (new_name) {
        move_file(name, new_name);
    }
}

function copy_dialoque(name) {
    const new_name = prompt(`Name of copied file`, name);

    if (new_name) {
        copy_file(name, new_name);
    }
}

function delete_dialoque(name) {
    if (confirm(`Delete ${name}?`)) {
        delete_file(name);
    }
}

function create_folder() {
    const path = prompt(`Name of new folder`);

    if (path) {
        create_dir(path);
    }
}

function toggle_dropdown(ev) {
    ev.currentTarget.classList.toggle("is-active");

    switch (ev.target.innerText.trim()) {
        case "Rename":
            // extract name
            rename_dialoque(ev.currentTarget.parentElement.parentElement.children[1].innerText);
            break;
        case "Copy":
            // extract name
            copy_dialoque(ev.currentTarget.parentElement.parentElement.children[1].innerText);
            break;
        case "Delete":
            delete_dialoque(ev.currentTarget.parentElement.parentElement.children[1].innerText);
            break;
    }
}

function upload_files_ev() {
    const files = document.getElementById("theForm");
    upload_files("", files, () => load_files());
}

function load_files() {
    const list = document.getElementById("file-list");

    list_files().then((result) => {
        console.table(result);

        list.innerHTML = '';

        for (let file of result.files) {
            const new_entry = new_list_entry(file, "File");
            //new_entry.addEventListener("click", download_file_ev);
            list.appendChild(new_entry);
        }

        for (let dir of result.dirs) {
            const new_entry = new_list_entry(dir, "Folder");
            list.appendChild(new_entry);
        }
    });
}

function download_file_ev(ev) {
    download_file(ev.target.innerText);
}
