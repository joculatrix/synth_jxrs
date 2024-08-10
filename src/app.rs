
use tokio::sync::broadcast::{Sender, Receiver};
use crate::{main, message::{Channel, Message}, osc, synth};

pub fn run(tx: Sender<Message>) {
    let main_window = MainWindow::new().unwrap();

    let tx_clone = tx.clone();
    main_window.on_prop_changed(move |index, prop, value| {
        match prop {
            OscProps::Freq => {
                tx_clone.send(Message::Freq(index as usize, value as f64));
            }
            OscProps::Waveform => {
                let waveform = match value {
                    0 => osc::wave::Waveform::Noise,
                    1 => osc::wave::Waveform::Saw,
                    3 => osc::wave::Waveform::Square,
                    4 => osc::wave::Waveform::Triangle,
                    _ => osc::wave::Waveform::Sine, // just set to Sine if something goes wrong?
                };
                tx_clone.send(Message::Waveform(index as usize, waveform));
            }
        }
    });

    main_window.run().unwrap();
}

slint::slint! {
    import { ComboBox, HorizontalBox, LineEdit } from "std-widgets.slint";

    export enum OscProps { freq, waveform }

    component KnobBackground inherits Path {
        in property <length> thickness;
        in property <float> size;

        stroke: black;
        stroke-width: thickness;

        viewbox-width: size;
        viewbox-height: size;

        // path just constructs a circle outline:

        MoveTo {
            x: size / 2;
            y: size;
        }
        ArcTo {
            sweep: true;
            radius-x: size / 2;
            radius-y: size / 2;
            x: size / 2;
            y: 0;
        }
        ArcTo {
            sweep: true;
            radius-x: size / 2;
            radius-y: size / 2;
            x: size / 2;
            y: size;
        }
    }

    component KnobEmpty inherits Path {
        in property <length> thickness;
        in property <float> size;

        stroke: @linear-gradient(0deg, #232629 20%, #393c40 90%);
        stroke-width: thickness;

        viewbox-width: size;
        viewbox-height: size;

        MoveTo {
            x: size / 2;
            y: size;
        }
        ArcTo {
            large-arc: true;
            sweep: true;
            radius-x: size / 2;
            radius-y: size / 2;
            x: size;
            y: size / 2;
        }
    }

    component KnobFill inherits Path {
        in property <length> thickness;
        in property <float> size;
        in property <float> progress;
        in property <color> color;

        stroke: color;
        stroke-width: thickness;

        viewbox-width: size;
        viewbox-height: size;
        MoveTo {
            x: size / 2;
            y: size;
        }
        // draw a section of a (near) circle from the bottom of the knob to the appropriate value:
        ArcTo {
            // this large-arc assignment helps keep the curve mostly aligned to the circle
            large-arc: progress >= 0.67 ? true : false;
            sweep: true;
            radius-x: size / 2;
            radius-y: size / 2;
            x: (size / 2) - (size / 2) * sin(progress * 270deg);
            y: (size / 2) + (size / 2) * cos(progress * 270deg);
        }
    }

    component Knob inherits Rectangle {
        in property <length> thickness: 10px;
        in property <length> size: 250px;
        in-out property <float> progress;
        in-out property <int> value;
        in property <color> accent-color: blue;
        in property <string> text;

        callback changed;
        callback double-clicked;
        callback text_input_accepted(string);

        width: size / 2 + 10px;
        height: size / 2 + 40px;

        VerticalLayout {
            spacing: 5px;
            padding: 5px; 
            alignment: center;

            Text {
                horizontal-alignment: center;
                text: root.text;
            }
            Rectangle {
                width: root.size / 2;
                height: root.size / 2;
                border-radius: size/2;
                background: @linear-gradient(0deg, #4f5d6e 20%, #939aa3 90%);

                KnobBackground {
                    thickness: root.thickness;
                    size: root.size / 1px;
                }
                KnobEmpty {
                    thickness: root.thickness;
                    size: root.size / 1px;
                }
                KnobFill {
                    thickness: root.thickness;
                    size: root.size / 1px;
                    progress <=> root.progress;
                    color: root.accent-color;
                }
                Rectangle {
                    // TODO: this math technically works ok, but isn't perfect. workshop better solutions
                    background: root.accent-color;
                    width: root.size / 35;
                    height: root.size / 35;
                    border-radius: self.width / 2;

                    x: ((root.size / 4) - sin(root.progress * 270deg) * (root.size / 6)) - (self.width / 2);
                    y: ((root.size / 4) + cos(root.progress * 270deg) * (root.size / 6)) - (self.height / 2);
                }
                area := TouchArea {
                    height: root.size / 2 - 40px;

                    moved => {
                        root.progress = self.mouse-y > self.pressed-y ?
                            max(0.0, (root.progress) - ((self.mouse-y) - self.pressed-y) / 5000px)
                            : min(1.0, root.progress + ((self.pressed-y) - self.mouse-y) / 5000px);
                        root.value = floor(root.progress / 0.0005 + 10.0);
                        root.changed();
                    }
                    double-clicked => {
                        root.double-clicked();
                    }
                }
            }
            input_field := LineEdit {
                width: root.size / 3;
                placeholder-text: root.value;

                accepted(s) => {
                    root.text_input_accepted(s);
                    self.text = ""; // clears the editing text, allowing the placeholder text to update again
                    fs.focus(); // takes focus from the text field
                    root.changed();
                }
            }
        }

        fs := FocusScope { } // hacky solution to remove focus from other elements
    }

    component Oscillator inherits Rectangle {
        in-out property <int> frequency: 440;
        in property <color> accent-color: blue;

        callback changed(OscProps, int);

        border-radius: 10px;
        background: @linear-gradient(0deg, #4f5d6e 20%, #939aa3 90%);

        GridLayout {
            padding: 10px;
            spacing: 10px;

            Row {
                ComboBox {
                    model: ["Noise", "Saw", "Sine", "Square", "Triangle"];
                    current-value: "Sine";

                    selected(s) => {
                        root.changed(OscProps.waveform, self.current-index);
                    }
                }
            }
            Row {
                freq_knob := Knob {
                    text: "FREQUENCY";
                    value <=> root.frequency;
                    progress: ((self.value) - 10.0) * 0.0005;
                    size: 200px;
                    accent-color: root.accent-color;

                    changed => {
                        root.changed(OscProps.freq, freq_knob.value);
                    }
                    double-clicked => {
                        freq_knob.value = 440;
                        freq_knob.progress = ((freq_knob.value) - 10.0) * 0.0005;
                    }
                    text_input_accepted(s) => {
                        freq_knob.value = max(10, (min(2010, s.to-float())));
                        freq_knob.progress = ((freq_knob.value) - 10.0) * 0.0005;
                    }
                }
            }
        }
    }

    export component MainWindow inherits Window {
        callback prop_changed(int, OscProps, int);

        HorizontalBox {
            osc1 := Oscillator {
                accent-color: @linear-gradient(0deg, #0768b8 20%, #16b4e4 90%);

                changed(prop, val) => {
                    root.prop_changed(0, prop, val);
                }
            }
            osc2 := Oscillator {
                accent-color: @linear-gradient(0deg, #aa0a6d 20%, #db2cc4 90%);

                changed(prop, val) => {
                    root.prop_changed(1, prop, val);
                }
            }
            osc3 := Oscillator {
                accent-color: @linear-gradient(0deg, #da460c 20%, #e76a17 90%);

                changed(prop, val) => {
                    root.prop_changed(2, prop, val);
                }
            }
        }
    }
}