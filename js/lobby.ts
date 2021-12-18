import * as ui from "./components";
import * as feather from "feather-icons";

export function lobby() {
    let root = ui.centre(
        ui.vboxFill([
            ui.hboxFill([
                ui.hook(ui.p("#59055"), (p) => {
                    p.style.fontSize = "24pt";
                }),
                ui.iconButton("copy"),
                ui.filler(),
                ui.hook(ui.p("LOBBY"), (p) => {
                    p.classList.add("ui-align-right");
                    p.style.fontWeight = "900";
                    p.style.fontSize = "24pt";
                }),
            ]),
            ui.panel(
                ui.hboxFill([
                    lobbyControls(true),
                    ui.vsep(),
                    playerList(
                        ["funny man", "funny man rises", "funny woman"],
                        true
                    ),
                    ui.vsep(),
                    ui.margin(
                        ui.vboxFill([
                            ui.hook(
                                ui.button(),
                                (btn) => (btn.innerHTML = "Ready")
                            ),
                            ui.hook(
                                ui.button(),
                                (btn) => (btn.innerHTML = "Exit")
                            ),
                        ]),
                        [-5, 10, 0, 0]
                    ),
                ])
            ),
        ])
    );

    document.body.appendChild(root);
    feather.replace({
        color: "white",
    });
}

function lobbyControls(isHost: boolean) {
    const numberInput = (name: string) => {
        let decrBtn = numberInputControl("minus");
        let incrBtn = numberInputControl("plus");
        let el = ui.hboxFill([
            ui.hook(ui.p(name), (input) => (input.style.fontSize = "20pt")),
            ui.filler(),
            decrBtn,
            ui.hook(ui.input(), (input) => {
                input.type = "number";
                input.value = "0";
                input.style.width = "100px";

                decrBtn.addEventListener("mousedown", (ev: MouseEvent) => {
                    input.value = String(parseInt(input.value) - 1);
                });

                incrBtn.addEventListener("mousedown", (ev: MouseEvent) => {
                    input.value = String(parseInt(input.value) + 1);
                });
            }),
            incrBtn,
        ]);

        return el;
    };

    return ui.vboxFill([
        numberInput("Width"),
        numberInput("Height"),
        numberInput("Timer"),
    ]);
}

function playerList(players: string[], isHost: boolean) {
    return ui.vboxFill(
        players.map((player) => {
            let elements: HTMLElement[] = [ui.p(player)];
            if (isHost) {
                elements.push(ui.filler());
                elements.push(ui.iconButton("x"));
            }
            return ui.softPanel(ui.hboxFill(elements));
        })
    );
}

function numberInputControl(icon: string) {
    let btn = ui.iconButton(icon);
    btn.classList.add("ui-number-input-control");
    return btn;
}
