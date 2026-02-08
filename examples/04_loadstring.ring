load "slint.ring"

cSlintSource = '
import { Button, LineEdit, VerticalBox, HorizontalBox, GroupBox } from "std-widgets.slint";

export component DynamicApp inherits Window {
    title: "Ring Slint";
    
    callback greet(string);
    callback update-message(string);
    callback clear-form();
    
    in-out property <string> greeting: "Enter your name and click Greet!";
    
    VerticalBox {
        padding: 16px;
        spacing: 12px;
        
        HorizontalLayout {
            alignment: center;
            spacing: 8px;
            
            Path {
                width: 24px;
                height: 24px;
                viewbox-width: 24;
                viewbox-height: 24;
                fill: #f59e0b;
                commands: "M13 2L3 14h9l-1 8 10-12h-9l1-8z";
            }
            
            Text {
                text: "Ring Slint";
                font-size: 24px;
                font-weight: 700;
                vertical-alignment: center;
            }
        }
        
        Rectangle {
            vertical-stretch: 1;
            
            Text {
                text: greeting;
                font-size: 18px;
                horizontal-alignment: center;
                vertical-alignment: center;
                wrap: word-wrap;
            }
        }
        
        VerticalBox {
            spacing: 8px;
            
            name-input := LineEdit {
                placeholder-text: "Enter your name...";
                font-size: 16px;
            }
            
            Button {
                text: "Greet";
                primary: true;
                clicked => { greet(name-input.text); }
            }
            
            HorizontalBox {
                spacing: 8px;
                
                Button {
                    text: "Hello";
                    clicked => { update-message("Hello!"); }
                }
                
                Button {
                    text: "Goodbye";
                    clicked => { update-message("Goodbye!"); }
                }
                
                Button {
                    text: "Clear";
                    clicked => { clear-form(); }
                }
            }
        }
    }
}
'

oApp = new SlintApp {
    loadUIString(cSlintSource, "dynamic://app.slint")
    setCallback("greet", :onGreet)
    setCallback("update-message", :onUpdateMessage)
    setCallback("clear-form", :onClearForm)
    show()
    run()
}

func onGreet
    cName = oApp.callbackArg(1)
    if len(cName) > 0
        oApp.set("greeting", "Hello, " + cName + "!")
    else
        oApp.set("greeting", "Please enter your name first!")
    ok
    ? "Greeted: " + cName

func onUpdateMessage
    cMessage = oApp.callbackArg(1)
    oApp.set("greeting", cMessage)

func onClearForm
    oApp.set("greeting", "Enter your name and click Greet!")
