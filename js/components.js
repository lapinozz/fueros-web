export function hook(x, f) {
    f(x);
    return x;
}

export function button() {
    let btn = document.createElement("button");
    btn.classList.add("ui-button");
    return btn;
}

export function input() {
    let input = document.createElement("input");
    input.classList.add("ui-input");
    return input;
}

export function h1(text) {
    let h1 = document.createElement("h1");
    h1.classList.add("ui-h1");
    h1.innerHTML = text;
    return h1;
}

export function vboxFill(children) {
    let div = document.createElement("div");
    div.classList.add("ui-vbox-fill");

    for (const child of children) {
        div.appendChild(child);
    }

    return div;
}

export function hboxFill(children) {
    let div = document.createElement("div");
    div.classList.add("ui-hbox-fill");

    for (const child of children) {
        div.appendChild(child);
    }

    return div;
}

export function panel(child) {
    let div = document.createElement("div");
    div.classList.add("ui-panel");
    div.appendChild(child);
    return div;
}

export function centre(child) {
    let div = document.createElement("div");
    div.classList.add("ui-centre");
    div.appendChild(child);
    return div;
}

export function margin(child) {
    let div = document.createElement("div");
    div.classList.add("ui-margin");
    div.appendChild(child);
    return div;
}

export function vsep() {
    let div = document.createElement("div");
    div.classList.add("ui-vsep");
    return div;
}

export function hsep() {
    let div = document.createElement("div");
    div.classList.add("ui-hsep");
    return div;
}
