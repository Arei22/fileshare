const submitButton = document.getElementById("submit");

if (!(submitButton instanceof HTMLDivElement)) {
    throw new Error("Submit button not found");
}

submitButton.addEventListener("click", () => {
    void (async () => {
        const fileElem = document.getElementById("file-input");

        if (!(fileElem instanceof HTMLInputElement)) {
            console.error("File input not found");
            return;
        }

        const file = fileElem.files?.[0];
        console.log(fileElem.files);

        if (!file) {
            console.error("No file selected");
            return;
        }

        const params = new URLSearchParams(window.location.search);
        const uuid = params.get("uuid");

        if (!uuid) {
            console.error("UUID missing");
            return;
        }

        const formData = new FormData();
        formData.append("file", file);
        formData.append("uuid", uuid);

        try {
            await fetch("/get_upload", {
                method: "POST",
                body: formData,
            });

            document.body.textContent = "Le fichier à bien été uploadé.";
        } catch (err: unknown) {
            console.error(
                `Error: ${err instanceof Error ? err.message : String(err)}`
            );
        }
    })();
});
