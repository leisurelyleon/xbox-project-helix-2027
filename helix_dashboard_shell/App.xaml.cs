using Microsoft.UI.Xaml;
using HelixShell.Views;
using HelixShell.ViewModels;
using System.Threading.Tasks;
using System;

namespace HelixShell
{
    /// <summary>
    /// Provides application-specific behavior to supplement the default Application class.
    /// Acts as the master orchestrator for the Project Helix UX.
    /// </summary>
    public partial class App : Application
    {
        private Window m_window;
        private TelemetrySync m_telemetrySync;

        public App()
        {
            this.InitializeComponent();
            
            // Initialize the background microvisor telemetry listener immediately
            m_telemetrySync = new TelemetrySync();
        }

        protected override void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args)
        {
            Console.WriteLine("[HELIX-SHELL] Booting Windows 12 / Xbox Unified Environment...");
            
            // 1. Start polling the Rust Kernel for APU thermals and Memory Crises
            m_telemetrySync.StartMonitoring();

            // 2. Launch the immersive boot sequence first (The "Sunset" UI)
            m_window = new BootSequence();
            m_window.Activate();

            // 3. Simulate the hardware initialization delay before loading the Unified Storefront
            _ = TransitionToMainShellAsync();
        }

        private async Task TransitionToMainShellAsync()
        {
            // Wait for the BootSequence animation Storyboard to finish (approx 3.5 seconds)
            await Task.Delay(3500);

            Console.WriteLine("[HELIX-SHELL] Hardware sync complete. Injecting main dashboard UI.");

            // In a production implementation, we would swap the Window.Content here 
            // or launch a new Window containing the UnifiedStorefront, injecting 
            // the ShellStateManager context so the user can hit the "PC Mode" toggle.
            
            /*
            var shellManager = new ShellStateManager();
            var mainDashboard = new UnifiedStorefront { DataContext = shellManager };
            m_window.Content = mainDashboard;
            */
        }
    }
}
