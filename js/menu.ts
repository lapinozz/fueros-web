import * as ui from "./components";
import * as feather from "feather-icons";

export function menu() {
    let root = ui.centre(
        ui.vboxFill([
            ui.h1("Fueros"),
            ui.h3("A DOTS AND BOXES GAME"),
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
    feather.replace();
}
