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

function get_jwt_header() {
    const jwt = localStorage.getItem('triox-jwt');

    if (jwt) {
        return {
            'Authorization': "Bearer " + jwt,
        };
    } else {
        return {};
    }
}

function get_jwt() {
    fetch('/login', {
            method: 'POST',
            headers: get_jwt_header(),
            redirect: 'follow',
            referrerPolicy: 'no-referrer',
            body: JSON.stringify(data)
        })
        .then(response => response.json())
        .then(data => console.log(data));
}
