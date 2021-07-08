'use strict';

function insert_notification_on_top(parent, text) {
  if (!text) {
    text = 'Unknown error';
  }

  const elem = document.getElementById('notification_on_top');

  if (elem) {
    elem.innerText = text;
    return;
  }

  const div = document.createElement('DIV');
  const button = document.createElement('BUTTON');

  div.className = 'notification is-danger';
  button.className = 'delete';

  div.appendChild(button);
  div.innerText = text;
  div.id = 'notification_on_top';

  parent.insertBefore(div, parent.firstChild);
}

function send_json(json, url, success_fn, error_fn) {
  console.table(json, url);
  var xhr = new XMLHttpRequest();
  xhr.open('POST', url, true);
  xhr.setRequestHeader('Content-Type', 'application/json');
  xhr.onreadystatechange = () => {
    if (xhr.readyState === 4) {
      console.log(`Response: ${xhr.responseText}`);
      if (xhr.status === 200) {
        success_fn(xhr.responseText);
      } else {
        error_fn(xhr.responseText);
      }
    }
  };
  xhr.send(json);
}

function get_jwt_header() {
  const jwt = localStorage.getItem('triox-jwt');

  if (jwt) {
    return {
      Authorization: 'Bearer ' + jwt,
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
    body: JSON.stringify(data),
  })
    .then(response => response.json())
    .then(data => console.log(data));
}
