'use strict';

let current_path = [];

function get_dir_string(path = [], secondary_path = []) {
     if (path.length === 0 && secondary_path.length === 0) {
        return "";
     }

     let dir_string = "";

     for (const entry of current_path) {
         dir_string += entry + "/";
     }

     for (const entry of secondary_path) {
         dir_string += entry + "/";
     }

     return dir_string.slice(0, -1);
}

function update_dir() {
     const path_list = document.getElementById("path-list");

     path_list.innerHTML = "";

     const li = document.createElement("LI");
     const a = document.createElement("A");

     a.innerText = "Home";
     li.appendChild(a);
     path_list.appendChild(li);

     for (const entry of current_path) {
         const li = document.createElement("LI");
         const a = document.createElement("A");

         a.innerText = entry;
         li.appendChild(a);

         path_list.appendChild(li);
     }

     const children = path_list.children;
     let z = children.length;

     for (const child of children) {
         z--;
         if (child != path_list.lastChild) {
             let i = z;
             child.addEventListener("click", function () {
                 current_path = current_path.slice(0, -i);
                 update_dir();
             });
         }
     }

     path_list.lastChild.className = "is-active";

     load_files();
}

function load_dir(dir) {
     current_path = dir;
     update_dir();
}

function open_dir(dir) {
     current_path.push(dir);
     update_dir();
}

function leave_dir() {
     current_path.pop();
     update_dir();
}

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
<a href="/app/files/get?path=${get_dir_string(current_path, [name])}" download>
  ${name}
</a>`
    } else {
        name_td.innerHTML = `
<a onclick="open_dir('${name}')">
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
    <span class="icon has-background-link-light">
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

function rename_dialoque(name) {
    const new_name = prompt(`Rename file ${name}`, name);

    if (new_name) {
        move_file(get_dir_string(current_path, [name]), get_dir_string(current_path, [new_name]));
    }
}

function copy_dialoque(name) {
    const new_name = prompt(`Name of copied file`, name);

    if (new_name) {
        copy_file(get_dir_string(current_path, [name]), get_dir_string(current_path, [new_name]));
    }
}

function delete_dialoque(name) {
    if (confirm(`Delete ${name}?`)) {
        delete_file(get_dir_string(current_path, [name]));
    }
}

function create_folder() {
    const path = prompt(`Name of new folder`);

    if (path) {
        create_dir(get_dir_string(current_path, [path]));
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
    upload_files(get_dir_string(current_path), files, () => load_files());
}

function load_files() {
    const list = document.getElementById("file-list");

    list_files(get_dir_string(current_path)).then((result) => {
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

window.addEventListener('load', load_files);
