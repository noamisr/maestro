import logging
import tempfile
from pathlib import Path
from typing import Dict, Any

import librosa
import numpy as np
import soundfile as sf

from ..config import settings

logger = logging.getLogger("maestro-sidecar.audio")

SUPPORTED_EXTENSIONS = {".wav", ".mp3", ".aiff", ".aif", ".flac", ".ogg"}


def is_supported_audio(path: str) -> bool:
    """Check if a file path has a supported audio extension."""
    return Path(path).suffix.lower() in SUPPORTED_EXTENSIONS


def extract_metadata(file_path: str) -> Dict[str, Any]:
    """Extract audio metadata for ChromaDB storage."""
    path = Path(file_path)

    try:
        info = sf.info(file_path)
        duration = float(info.duration)
        sample_rate = info.samplerate
        channels = info.channels
    except Exception:
        # Fallback for formats soundfile can't read natively
        y, sr = librosa.load(file_path, sr=None, duration=5.0)
        duration = float(len(y) / sr)
        sample_rate = sr
        channels = 1

    # Quick RMS energy analysis (first second only)
    try:
        y_short, _ = librosa.load(file_path, sr=None, duration=1.0, mono=True)
        rms = float(np.sqrt(np.mean(y_short**2)))
    except Exception:
        rms = 0.0

    return {
        "file_path": str(path.absolute()),
        "file_name": path.stem,
        "extension": path.suffix.lower(),
        "directory": str(path.parent),
        "duration_seconds": round(duration, 3),
        "sample_rate": sample_rate,
        "channels": channels,
        "rms_energy": round(rms, 4),
        "file_size_bytes": path.stat().st_size,
    }


def prepare_for_embedding(
    file_path: str,
    target_sr: int = settings.sample_rate,
    max_duration: int = settings.max_audio_duration_seconds,
) -> str:
    """Resample audio to target sample rate if needed.

    CLAP expects 48kHz audio. Returns path to a temp file if resampling
    was needed, or the original path if already compatible.
    """
    try:
        info = sf.info(file_path)
        if info.samplerate == target_sr and info.duration <= max_duration:
            return file_path
    except Exception:
        pass

    # Need to resample or truncate
    y, sr = librosa.load(file_path, sr=target_sr, duration=max_duration, mono=True)
    temp_dir = Path(tempfile.gettempdir()) / "maestro_audio"
    temp_dir.mkdir(exist_ok=True)
    temp_path = temp_dir / f"{Path(file_path).stem}_resampled.wav"
    sf.write(str(temp_path), y, target_sr)
    return str(temp_path)
