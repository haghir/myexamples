function querySelector(selector: string): HTMLElement {
    const elem: HTMLElement | null = document.querySelector(selector);
    if (elem == null)
        throw new Error(`The element corresponding to ${selector} was not found.`);

    return elem;
}

window.onload = () => {
    const up: HTMLElement = querySelector("#up");
    const down: HTMLElement = querySelector("#down");
    const left: HTMLElement = querySelector("#left");
    const right: HTMLElement = querySelector("#right");
    const z: HTMLElement = querySelector("#z");
    const x: HTMLElement = querySelector("#x");

    function update(key: string, value: string) {
        switch (key) {
            case "ArrowUp":
                up.innerHTML = value;
                break;
            case "ArrowDown":
                down.innerHTML = value;
                break;
            case "ArrowLeft":
                left.innerHTML = value;
                break;
            case "ArrowRight":
                right.innerHTML = value;
                break;
            case "Z":
            case "z":
                z.innerHTML = value;
                break;
            case "X":
            case "x":
                x.innerHTML = value;
                break;
        }
    }

    window.addEventListener("keydown", (e) => {
        if (!e.repeat)
            update(e.key, "●");
    });

    window.addEventListener("keyup", (e) => {
        update(e.key, "○");
    });
};
