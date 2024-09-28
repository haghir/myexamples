window.onload = function() {
    const hello = document.getElementById('hello');
    const button = document.getElementById('button');

    button.onclick = function() {
        fetch('/hello').then((response) => {
            if (!response.ok)
                throw new Error(`Something went wrong (${response.status})`);

            return response.text();
        }).then((response) => {
            hello.innerHTML = response;
        });
    };
};
