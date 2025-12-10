($ => {
    const NS = "http://www.w3.org/2000/svg";

    window.addEventListener("load", () => {
        const div = $.querySelector("#timer");

        const svg = $.createElementNS(NS, "svg");
        svg.setAttribute("width", "500");
        svg.setAttribute("height", "500");
        svg.setAttribute("viewBox", "-500 -500 1000 1000");
        div.appendChild(svg);

        const timer = $.createElementNS(NS, "path");
        timer.setAttribute("d", "M 0 0 L 0 -400 A 400 400 0 1 0 400 0 Z");
        timer.setAttribute("fill", "black");
        svg.appendChild(timer);

        let step = 0;
        setInterval(() => {
            step = (step + 1) % 100;
            const sweep = 1 - Math.floor(step / 50);
            const angle = Math.PI * (step - 25) / 50;
            const x = 400 * Math.cos(angle);
            const y = 400 * Math.sin(angle);
            timer.setAttribute("d", `M 0 0 L 0 -400 A 400 400 0 ${sweep} 0 ${x} ${y} Z`);
        }, 100);
    });
})(document);