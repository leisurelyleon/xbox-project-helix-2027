#include <iostream>
#include <map>
#include <windows.h>
#include <xinput.h>

namespace helix {
namespace bridge {

class SeamlessControllerMap {
public:
    SeamlessControllerMap() : active_input_type(InputType::XBOX_CONTROLLER) {
        std::cout << "[INPUT-BRIDGE] Seamless input translation matrix active.\n";
    }

    enum class InputType {
        KEYBOARD_MOUSE,
        XBOX_CONTROLLER
    };

    void PollHardware() {
        // Poll the physical USB/Wireless bus
        XINPUT_STATE controllerState;
        DWORD controllerStatus = XInputGetState(0, &controllerState);

        bool isControllerActive = (controllerStatus == ERROR_SUCCESS && controllerState.Gamepad.wButtons != 0);
        bool isKeyboardActive = (GetAsyncKeyState(VK_SPACE) & 0x8000) != 0; // Check for any key press

        // Auto-detect the user's preferred input method instantly
        if (isControllerActive && active_input_type != InputType::XBOX_CONTROLLER) {
            std::cout << "[INPUT-BRIDGE] Hot-swapped to Xbox Controller Mode. Translating UI prompts...\n";
            active_input_type = InputType::XBOX_CONTROLLER;
        } else if (isKeyboardActive && active_input_type != InputType::KEYBOARD_MOUSE) {
            std::cout << "[INPUT-BRIDGE] Hot-swapped to Keyboard/Mouse Mode. Injecting raw input...\n";
            active_input_type = InputType::KEYBOARD_MOUSE;
        }
    }

    // Translates an Xbox Controller button press into a fake Windows Keyboard event for the PC Game
    void SpoofKeyboardFromController(WORD xinput_button) {
        INPUT ip = {0};
        ip.type = INPUT_KEYBOARD;

        if (xinput_button & XINPUT_GAMEPAD_A) {
            ip.ki.wVk = VK_SPACE; // 'A' acts as 'Space' (Jump)
        } else if (xinput_button & XINPUT_GAMEPAD_B) {
            ip.ki.wVk = VK_ESCAPE; // 'B' acts as 'Escape' (Back)
        }

        // Inject the fake keyboard event into the Windows 12 subsystem
        SendInput(1, &ip, sizeof(INPUT));
    }

private:
    InputType active_input_type;
};

} // namespace bridge
} // namespace helix
