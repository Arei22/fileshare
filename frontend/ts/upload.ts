const submit_buton: HTMLButtonElement = document.getElementById('submit');

submit_buton.addEventListener("click", async () => {
    const fileElem = document.getElementById("fileElem") as HTMLInputElement;
    const file = fileElem.files?.[0];
    var url = window.location;
    let params = new URLSearchParams(url.search);
    let uuid = params.get("uuid");

    const formData = new FormData();
    formData.append('file', file);
    formData.append('uuid', uuid);



    try {
        await fetch('/get_upload', {
            method: 'post',
            body: formData
        });
        document.querySelector("body").innerHTML = "";
        document.querySelector("body").textContent = "Done";
    } catch (err) {
        console.error(`Error: ${err}`);
    }
});