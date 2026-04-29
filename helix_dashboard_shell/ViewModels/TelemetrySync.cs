using System;
using System.Threading;
using System.Threading.Tasks;
using System.ComponentModel;

namespace HelixShell.ViewModels
{
    public class TelemetrySync : INotifyPropertyChanged
    {
        public event PropertyChangedEventHandler PropertyChanged;

        private double _apuTemperature;
        public double ApuTemperature
        {
            get => _apuTemperature;
            set { _apuTemperature = value; OnPropertyChanged(nameof(ApuTemperature)); }
        }

        private string _memoryPressureState;
        public string MemoryPressureState
        {
            get => _memoryPressureState;
            set { _memoryPressureState = value; OnPropertyChanged(nameof(MemoryPressureState)); }
        }

        private CancellationTokenSource _cts;

        public TelemetrySync()
        {
            _memoryPressureState = "NORMAL";
            _apuTemperature = 45.0;
        }

        public void StartMonitoring()
        {
            _cts = new CancellationTokenSource();
            Task.Run(() => MonitorKernelTelemetry(_cts.Token));
        }

        public void StopMonitoring()
        {
            _cts?.Cancel();
        }

        private async Task MonitorKernelTelemetry(CancellationToken token)
        {
            // In a real implementation, this reads from a Named Pipe or Shared Memory 
            // segment exposed by the Rust Kernel. We simulate the data stream here.
            
            Random rnd = new Random();

            while (!token.IsCancellationRequested)
            {
                // Simulate polling APU thermals
                ApuTemperature = 60.0 + (rnd.NextDouble() * 15.0);

                // Simulate checking the ResilientAllocator for "Memory Crisis" states
                int randomPressure = rnd.Next(0, 100);
                if (randomPressure > 95)
                {
                    MemoryPressureState = "CRISIS - FSR RESERVE PROTECTED";
                }
                else if (randomPressure > 80)
                {
                    MemoryPressureState = "ELEVATED";
                }
                else
                {
                    MemoryPressureState = "NORMAL";
                }

                // Poll every 500ms
                await Task.Delay(500, token);
            }
        }

        protected void OnPropertyChanged(string propertyName)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
    }
}
