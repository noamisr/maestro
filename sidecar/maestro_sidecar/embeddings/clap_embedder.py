import logging
import os
from typing import List

import numpy as np

from ..config import settings

logger = logging.getLogger("maestro-sidecar.embeddings")


class CLAPEmbedder:
    """Wrapper around the LAION CLAP model for generating audio/text embeddings."""

    def __init__(self):
        import laion_clap

        # Set model cache directory
        os.makedirs(settings.model_cache_dir, exist_ok=True)
        os.environ["TORCH_HOME"] = settings.model_cache_dir

        logger.info("Loading CLAP model: %s", settings.clap_model)
        self.model = laion_clap.CLAP_Module(
            enable_fusion=settings.clap_enable_fusion,
        )
        self.model.load_ckpt()
        logger.info("CLAP model loaded successfully")

    def embed_audio_files(self, file_paths: List[str]) -> np.ndarray:
        """Generate embeddings for a list of audio files.

        Returns:
            numpy array of shape (N, 512) where N = len(file_paths).
        """
        embeddings = self.model.get_audio_embedding_from_filelist(
            x=file_paths,
            use_tensor=False,
        )
        return embeddings

    def embed_text(self, texts: List[str]) -> np.ndarray:
        """Generate embeddings for text descriptions.

        Returns:
            numpy array of shape (N, 512) where N = len(texts).
        """
        embeddings = self.model.get_text_embedding(texts)
        return embeddings

    @property
    def dimension(self) -> int:
        return 512
