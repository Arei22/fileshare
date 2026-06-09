const ACCEPTED_MIME_TYPES = [
    "application/zip",
    "application/x-tar",
    "application/gzip",
    "application/x-7z-compressed",
];

const dropZone = document.getElementById("drop-zone") as HTMLLabelElement;
const preview = document.getElementById("preview") as HTMLLabelElement;

dropZone.addEventListener("drop", dropHandler);

window.addEventListener("drop", (e) => {
    if ([...e.dataTransfer.items].some((item) => item.kind === "file")) {
        e.preventDefault();
    }
});

dropZone.addEventListener("dragover", (e) => {
    const fileItems = [...e.dataTransfer.items].filter(
        (item) => item.kind === "file",
    );
    if (fileItems.length > 0) {
        e.preventDefault();
        if (fileItems.some((item) => ACCEPTED_MIME_TYPES.includes(item.type))) {
            e.dataTransfer.dropEffect = "copy";
        } else {
            e.dataTransfer.dropEffect = "none";
        }
    }
});

window.addEventListener("dragover", (e) => {
    const fileItems = [...e.dataTransfer.items].filter(
        (item) => item.kind === "file",
    );
    if (fileItems.length > 0) {
        e.preventDefault();
        if (!dropZone.contains(e.target)) {
            e.dataTransfer.dropEffect = "none";
        }
    }
});


function displayFile(file: File) {
    if (ACCEPTED_MIME_TYPES.includes(file.type)) {
        preview.textContent = file.name;
    }
}

function dropHandler(ev: DragEvent) {
    ev.preventDefault();

    const file = [...ev.dataTransfer!.items]
        .map((item) => item.getAsFile())
        .find(
            (file): file is File =>
                file !== null && ACCEPTED_MIME_TYPES.includes(file.type),
        );

    if (!file) {
        return;
    }

    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);
    fileInput.files = dataTransfer.files;

    displayFile(file);
}


const fileInput = document.getElementById("file-input") as HTMLInputElement;
fileInput.addEventListener("change", (e) => {
    const file = (e.target as HTMLInputElement).files?.[0];

    if (file && ACCEPTED_MIME_TYPES.includes(file.type)) {
        displayFile(file);
    }
});

const clearBtn = document.getElementById("clear-btn") as HTMLDivElement;
clearBtn.addEventListener("click", () => {
    preview.textContent = "Lachez l'archive ici, ou clickez pour l'upload.";
    fileInput.files = new DataTransfer().files;
});