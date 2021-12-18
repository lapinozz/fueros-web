import * as ui from "./components";

export function menu() {
    let root = ui.centre(
        ui.vboxFill([
            ui.h1("Fueros"),
            ui.h3("A dots and boxes game"),
            ui.panel(
                ui.hboxFill([
                    ui.vboxFill([
                        ui.hook(
                            ui.input(),
                            (input) => (input.placeholder = "Username")
                        ),
                    ]),
                    ui.vsep(),
                    ui.vboxFill([
                        ui.hook(
                            ui.button(),
                            (btn) => (btn.innerHTML = "New Game")
                        ),
                        ui.hsep(),
                        ui.hook(
                            ui.input(),
                            (input) => (input.placeholder = "Lobby code")
                        ),
                        ui.hook(
                            ui.button(),
                            (btn) => (btn.innerHTML = "Join Game")
                        ),
                    ]),
                ])
            ),
        ])
    );

    document.body.appendChild(root);
}
