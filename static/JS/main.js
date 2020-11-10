'use strict';

function send_json(json, url, success_fn) {
    console.table(json, url);
    var xhr = new XMLHttpRequest();
    xhr.open('POST', url, true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.onreadystatechange = () => {
        if (xhr.readyState === 4) {
            console.log(`Response: ${xhr.responseText}`);
            if (xhr.status === 200) {
                success_fn(xhr.responseText);
            }
        }
    }
    xhr.send(json);
}

function get_jwt() {
    fetch('/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            redirect: 'follow',
            referrerPolicy: 'no-referrer',
            body: JSON.stringify(data)
        })
        .then(response => response.json())
        .then(data => console.log(data));
}

function upload_files(path = "", form, success_fn) {
    const jwt = localStorage.getItem('triox-jwt');
    const formData = new FormData(form);

    fetch(`/app/files/up/${path}`, {
        method: "POST",
        headers: {
            'x-triox-jwt': jwt,
        },
        body: formData
    }).then(() => {
        if (success_fn) {
            success_fn();
        }
    });
}

async function list_files(path = "") {
    const jwt = localStorage.getItem('triox-jwt');

    const response = await fetch(`/app/files/list/${path}`, {
        method: 'GET',
        headers: {
            'x-triox-jwt': jwt
        },
    });

    const result = response.json();

    return result;
}

function download_file(path) {
    const jwt = localStorage.getItem('triox-jwt');

    fetch(`/app/files/get/${path}`, {
            method: "GET",
            headers: {
                'x-triox-jwt': jwt,
            },
        }).then(response => response.blob())
        .then(blob => {
            var url = window.URL.createObjectURL(blob);
            var a = document.createElement('a');
            a.href = url;
            a.download = path;
            document.body.appendChild(a); // we need to append the element to the dom -> otherwise it will not work in firefox
            a.click();
            a.remove(); //afterwards we remove the element again
        });;
}
