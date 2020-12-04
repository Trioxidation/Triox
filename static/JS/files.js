'use strict';

function upload_files(path = "", form, success_fn) {

    const formData = new FormData(form);

    fetch(`/app/files/upload?path=${path}`, {
        method: "POST",
        headers: get_jwt_header(),
        body: formData
    }).then(() => {
        if (success_fn) {
            success_fn();
        }
    });
}

function move_file(old_path, new_path) {

    fetch(`/app/files/move`, {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                from: old_path,
                to: new_path
            }),
        }).then(response => response.body)
        .then(response => load_files());
}

function copy_file(old_path, new_path) {

    fetch(`/app/files/copy`, {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                from: old_path,
                to: new_path
            }),
        }).then(response => response.body)
        .then(response => load_files());
}

function delete_file(path) {

    fetch(`/app/files/remove?path=${encodeURIComponent(path)}`, {
            method: "GET",
        }).then(response => response.body)
        .then(response => load_files());
}

function create_dir(path) {

    fetch(`/app/files/create_dir?path=${encodeURIComponent(path)}`, {
            method: "GET",
        }).then(response => response.body)
        .then(response => load_files());
}

async function list_files(path = "/") {

    const response = await fetch(`/app/files/list?path=${encodeURIComponent(path)}`, {
        method: 'GET',
        headers: get_jwt_header(),
    });

    const result = response.json();

    return result;
}

function download_file(path) {

    fetch(`/app/files/get/${path}`, {
            method: "GET",
            headers: get_jwt_header(),
        }).then(response => response.blob())
        .then(blob => {
            var url = window.URL.createObjectURL(blob);
            var a = document.createElement('a');
            a.href = url;
            a.download = path;
            document.body.appendChild(a); // we need to append the element to the dom -> otherwise it will not work in firefox
            a.click();
            a.remove(); //afterwards we remove the element again
        });
}
