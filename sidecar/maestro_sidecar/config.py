from pydantic_settings import BaseSettings
from pathlib import Path


class Settings(BaseSettings):
    port: int = 9400
    host: str = "127.0.0.1"
    chroma_persist_dir: str = str(Path.home() / ".maestro" / "chroma_db")
    clap_model: str = "630k-audioset-best"
    clap_enable_fusion: bool = False
    sample_rate: int = 48000
    max_audio_duration_seconds: int = 30
    model_cache_dir: str = str(Path.home() / ".maestro" / "models")

    model_config = {"env_prefix": "MAESTRO_"}


settings = Settings()
