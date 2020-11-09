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

function list_files(path = "") {
    const jwt = localStorage.getItem('triox-jwt');

    fetch(`/app/files/list/${path}`, {
            method: 'GET',
            headers: {
                'x-triox-jwt': jwt
            },
        })
        .then(response => response.json())
        .then(data => console.log(data));
}

function upload_files_old(path = "", form) {
    const jwt = localStorage.getItem('triox-jwt');
    const formData = new FormData(form);

    fetch(`/app/files/up/${path}`, {
        method: "POST",
        headers: {
            'x-triox-jwt': jwt,
        },
        body: formData
    });
}

// Because we want to access DOM nodes,
// we initialize our script at page load.
/*window.addEventListener( 'load', function () {
    const file = {
        dom: document.getElementById( "theFile" ),
        binary: null
    };

    // Use the FileReader API to access file content
    const reader = new FileReader();

    // Because FileReader is asynchronous, store its
    // result when it finishes to read the file
    reader.addEventListener("load", function() {
        file.binary = reader.result;
    });

    // At page load, if a file is already selected, read it.
    if (file.dom.files[0]) {
        reader.readAsBinaryString(file.dom.files[0]);
    }

    // If not, read the file once the user selects it.
    file.dom.addEventListener("change", function() {
        if (reader.readyState === FileReader.LOADING) {
            reader.abort();
        }

        reader.readAsBinaryString(file.dom.files[0]);
    });

    // sendData is our main function
    function sendData() {
        // If there is a selected file, wait it is read
        // If there is not, delay the execution of the function
        if (!file.binary && file.dom.files.length > 0) {
            setTimeout(sendData, 10);
            return;
        }

        // To construct our multipart form data request,
        // We need an XMLHttpRequest instance
        const XHR = new XMLHttpRequest();

        // We need a separator to define each part of the request
        const boundary = "blob";

        // Store our body request in a string.
        let data = "";

        // So, if the user has selected a file
        if (file.dom.files[0]) {
            // Start a new part in our body's request
            data += "--" + boundary + "\r\n";

            // Describe it as form data
            data += 'content-disposition: form-data; '
                // Define the name of the form data
                +
                'name="' + file.dom.name + '"; '
                // Provide the real name of the file
                +
                'filename="' + file.dom.files[0].name + '"\r\n';
            // And the MIME type of the file
            data += 'Content-Type: ' + file.dom.files[0].type + '\r\n';

            // There's a blank line between the metadata and the data
            data += '\r\n';

            // Append the binary data to our body's request
            data += file.binary + '\r\n';

            // Once we are done, "close" the body's request
            data += "--" + boundary + "--";
        }

        // Define what happens on successful data submission
        XHR.addEventListener('load', function(event) {
            alert('Yeah! Data sent and response loaded.');
        });

        // Define what happens in case of error
        XHR.addEventListener('error', function(event) {
            alert('Oops! Something went wrong.');
        });

        // Set up our request
        XHR.open('POST', '/app/files/up/');
        XHR.setRequestHeader("x-triox-jwt", localStorage.getItem('triox-jwt'))

        // Add the required HTTP header to handle a multipart form data POST request
        XHR.setRequestHeader('Content-Type', 'multipart/form-data; boundary=' + boundary);

        // And finally, send our data.
        XHR.send(data);
    }
    // Access our form...
    const form = document.getElementById("theForm");

    // ...to take over the submit event
    form.addEventListener('submit', function(event) {
        event.preventDefault();
        sendData();
    });
});*/
