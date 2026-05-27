const submitButton = document.getElementById("submit");

if (!(submitButton instanceof HTMLButtonElement)) {
    throw new Error("Submit button not found");
}

submitButton.addEventListener("click", () => {
    void (async () => {
        const fileElem = document.getElementById("fileElem");

        if (!(fileElem instanceof HTMLInputElement)) {
            console.error("File input not found");
            return;
        }

        const file = fileElem.files?.[0];

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

            document.body.textContent = "Done";
        } catch (err: unknown) {
            console.error(
                `Error: ${err instanceof Error ? err.message : String(err)}`
            );
        }
    })();
});
