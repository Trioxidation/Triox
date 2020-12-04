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
            child.addEventListener("click", function() {
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

function new_list_entry(file, type) {
    const row = document.createElement("tr");

    const checkbox_td = document.createElement("td");

    checkbox_td.innerHTML = `
<label>
  <input type="checkbox">
</label>`;

    row.appendChild(checkbox_td);

    const name_td = document.createElement("td");

    if (type === "File") {
        name_td.innerHTML = `
<a href="/app/files/get?path=${encodeURIComponent(get_dir_string(current_path, [file.name]))}" download>
  ${file.name}
</a>`
    } else {
        name_td.innerHTML = `
<a onclick="open_dir('${file.name}')">
  ${file.name}
</a>`
    }
    row.appendChild(name_td);


    const size_td = document.createElement("td");

    if (type === "File") {
        let size = file.size;
        let tousands = 0;

        while (size > 1000) {
            size /= 1000;
            tousands++;
        }

        size = size.toFixed(1);

        let unit;
        switch (tousands) {
            case 0:
                unit = "B";
                break;
            case 1:
                unit = "KB";
                break;
            case 2:
                unit = "MB";
                break;
            case 3:
                unit = "GB";
                break;
            case 4:
                unit = "TB";
                break;
        }
        size_td.innerText = `${size} ${unit}`;
    } else {

    }

    row.appendChild(size_td);

    const last_modified = new Date(file.last_modified * 1000);

    const millis_since_modified = Date.now() - last_modified;

    let time_string;

    const years = Math.floor(millis_since_modified / 31536000000);
    const weeks = Math.floor(millis_since_modified / 604800000);
    const days = Math.floor(millis_since_modified / 86400000);
    const hours = Math.floor(millis_since_modified / 3600000);
    const minutes = Math.floor(millis_since_modified / 60000);
    const seconds = Math.floor(millis_since_modified / 1000);

    if (last_modified == 0) {
        time_string = "never";
    } else if (years == 1) {
        time_string = `one year ago`;
    } else if (years != 0) {
        time_string = `${years} years ago`;
    } else if (weeks == 1) {
        time_string = `one week ago`;
    } else if (weeks != 0) {
        time_string = `${weeks} weeks ago`;
    } else if (days == 1) {
        time_string = `one day ago`;
    } else if (days != 0) {
        time_string = `${days} days ago`;
    } else if (hours == 1) {
        time_string = `one hour ago`;
    } else if (hours != 0) {
        time_string = `${hours} hours ago`;
    } else if (minutes == 1) {
        time_string = `one minute ago`;
    } else if (minutes != 0) {
        time_string = `${minutes} minutes ago`;
    } else if (seconds == 1) {
        time_string = `one second ago`;
    } else if (seconds != 0) {
        time_string = `${seconds} seconds ago`;
    } else {
        time_string = "just now";
    }

    const last_modified_td = document.createElement("td");
    last_modified_td.innerText = time_string;
    row.appendChild(last_modified_td);


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

function rename_dialogue(name) {
    const new_name = prompt(`Rename file ${name}`, name);

    if (new_name) {
        move_file(get_dir_string(current_path, [name]), get_dir_string(current_path, [new_name]));
    }
}

function move_dialogue(name) {
    const new_name = prompt(`Move file ${name} to`, name);

    if (new_name) {
        move_file(get_dir_string(current_path, [name]), get_dir_string(current_path, [new_name]));
    }
}

function copy_dialogue(name) {
    const new_name = prompt(`Name of copied file`, name);

    if (new_name) {
        copy_file(get_dir_string(current_path, [name]), get_dir_string(current_path, [new_name]));
    }
}

function delete_dialogue(name) {
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

    // extract name
    const file_name = ev.currentTarget.parentElement.parentElement.children[1].innerText;

    switch (ev.target.innerText.trim()) {
        case "Rename":
            rename_dialogue(file_name);
            break;
        case "Copy":
            copy_dialogue(file_name);
            break;
        case "Move":
            move_dialogue(file_name);
            break;
        case "Delete":
            delete_dialogue(file_name);
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

        list.innerHTML = '';

        for (let file of result.files) {
            const new_entry = new_list_entry(file, "File");
            //new_entry.addEventListener("click", download_file_ev);
            list.appendChild(new_entry);
        }

        for (let dir of result.directories) {
            const new_entry = new_list_entry(dir, "Folder");
            list.appendChild(new_entry);
        }
    });
}

function download_file_ev(ev) {
    download_file(ev.target.innerText);
}

window.addEventListener('load', load_files);
