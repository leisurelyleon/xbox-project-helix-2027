using System;
using System.ComponentModel;
using System.Runtime.InteropServices;
using System.Windows.Input;

namespace HelixShell.ViewModels
{
    public class ShellStateManager : INotifyPropertyChanged
    {
        public event PropertyChangedEventHandler PropertyChanged;

        // P/Invoke hook into our bare-metal C++ bridge to trigger the hardware partition swap
        [DllImport("helix_shim.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern bool InitiateHardwareModeSwap(int targetMode);

        private bool _isNativePcModeActive;
        public bool IsNativePcModeActive
        {
            get => _isNativePcModeActive;
            set
            {
                if (_isNativePcModeActive != value)
                {
                    _isNativePcModeActive = value;
                    OnPropertyChanged(nameof(IsNativePcModeActive));
                    OnPropertyChanged(nameof(CurrentModeText));
                }
            }
        }

        public string CurrentModeText => _isNativePcModeActive ? "WINDOWS 12 DESKTOP" : "XBOX HELIX SANDBOX";

        public ICommand ToggleEnvironmentCommand { get; }

        public ShellStateManager()
        {
            IsNativePcModeActive = false; // Boot into Console mode by default
            ToggleEnvironmentCommand = new RelayCommand(ExecuteToggleEnvironment);
        }

        private void ExecuteToggleEnvironment()
        {
            Console.WriteLine($"[SHELL] Requesting bare-metal transition. Target PC Mode: {!IsNativePcModeActive}");
            
            // 0 = Xbox Mode, 1 = PC Mode
            bool success = InitiateHardwareModeSwap(!IsNativePcModeActive ? 1 : 0);
            
            if (success)
            {
                IsNativePcModeActive = !IsNativePcModeActive;
            }
            else
            {
                Console.WriteLine("[SHELL] ERROR: Hypervisor denied partition swap. Game currently active?");
            }
        }

        protected void OnPropertyChanged(string propertyName)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
    }
}
