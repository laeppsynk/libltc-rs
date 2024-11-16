import wave
import numpy as np

# File paths
raw_file_path = "timecode.raw"  # Path to the raw audio file
output_wav_path = "timecode.wav"  # Path to save the WAV file

# Parameters for the raw file format (assuming 16-bit mono PCM, 44.1kHz sample rate)
sample_rate = 48000  # Samples per second (44.1 kHz)
num_channels = 1     # Mono audio
sample_width = 1     # 16-bit samples (2 bytes per sample)

# Open the raw audio file and read the data
with open(raw_file_path, "rb") as raw_file:
    raw_samples = np.frombuffer(raw_file.read(), dtype=np.int16)

# Create and write the WAV file
with wave.open(output_wav_path, "wb") as wav_file:
    wav_file.setnchannels(num_channels)
    wav_file.setsampwidth(sample_width)
    wav_file.setframerate(sample_rate)
    wav_file.writeframes(raw_samples.tobytes())

print(f"WAV file saved as {output_wav_path}")

