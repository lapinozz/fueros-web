export function hook<Element>(x: Element, f: (el: Element) => void) {
    f(x);
    return x;
}

export function button() {
    let btn = document.createElement("button");
    btn.classList.add("ui-button");
    return btn;
}

export function iconButton(iconName: string) {
    let btn = document.createElement("button");
    btn.classList.add("ui-icon-button");
    btn.appendChild(icon(iconName));
    return btn;
}

export function icon(iconName: string) {
    let i = document.createElement("i");
    i.setAttribute("data-feather", iconName);
    return i;
}

export function input() {
    let input = document.createElement("input");
    input.classList.add("ui-input");
    return input;
}

export function h1(text: string) {
    let h1 = document.createElement("h1");
    h1.classList.add("ui-h1");
    h1.innerHTML = text;
    return h1;
}

export function h3(text: string) {
    let h3 = document.createElement("h3");
    h3.classList.add("ui-h3");
    h3.innerHTML = text;
    return h3;
}

export function p(text: string) {
    let p = document.createElement("p");
    p.classList.add("ui-p");
    p.innerHTML = text;
    return p;
}

export function vboxFill(children: HTMLElement[]) {
    let div = document.createElement("div");
    div.classList.add("ui-vbox-fill");

    for (const child of children) {
        div.appendChild(child);
    }

    return div;
}

export function hboxFill(children: HTMLElement[]) {
    let div = document.createElement("div");
    div.classList.add("ui-hbox-fill");

    for (const child of children) {
        div.appendChild(child);
    }

    return div;
}

export function filler() {
    let div = document.createElement("div");
    div.classList.add("ui-filler");
    return div;
}

export function fill(child: HTMLElement) {
    let div = document.createElement("div");
    div.classList.add("ui-fill");
    div.appendChild(child);
    return div;
}

export function panel(child: HTMLElement) {
    let div = document.createElement("div");
    div.classList.add("ui-panel");
    div.appendChild(child);
    return div;
}

export function softPanel(child: HTMLElement) {
    let div = document.createElement("div");
    div.classList.add("ui-soft-panel");
    div.appendChild(child);
    return div;
}

export function centre(child: HTMLElement) {
    let div = document.createElement("div");
    div.classList.add("ui-centre");
    div.appendChild(child);
    return div;
}

/// [top, right, bottom, left]
export function margin(child: HTMLElement, margins: number[]) {
    let div = document.createElement("div");
    div.classList.add("ui-margin");
    div.style.marginTop = String(margins[0]) + "px";
    div.style.marginRight = String(margins[1]) + "px";
    div.style.marginBottom = String(margins[2]) + "px";
    div.style.marginLeft = String(margins[3]) + "px";
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
