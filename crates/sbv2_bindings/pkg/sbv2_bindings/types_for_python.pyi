from typing import Optional, Union
from pathlib import Path

class TTSModel:
    """TTSModel class

    音声合成するために使うクラス

    Parameters
    ----------
    bert_model_bytes : bytes
        BERTモデルのバイナリデータ
    tokenizer_bytes : bytes
        トークナイザーのバイナリデータ
    max_loaded_models : Optional[int]
        同時にVRAMに存在するモデルの数
    """
    def __init__(
        self,
        bert_model_bytes: bytes,
        tokenizer_bytes: bytes,
        max_loaded_models: Optional[int] = None
    ) -> None: ...

    @staticmethod
    def from_path(
        bert_model_path: Union[str, Path],
        tokenizer_path: Union[str, Path],
        max_loaded_models: Optional[int] = None
    ) -> "TTSModel":
        """パスからTTSModelインスタンスを生成する

        Parameters
        ----------
        bert_model_path : str
            BERTモデルのパス
        tokenizer_path : str
            トークナイザーのパス
        max_loaded_models: Optional[int]
            同時にVRAMに存在するモデルの数

        Returns
        -------
        TTSModel
            生成されたTTSModelインスタンス
        """
        ...

    def load_sbv2file(self, ident: str, sbv2file_bytes: bytes) -> None:
        """SBV2ファイルを読み込む

        Parameters
        ----------
        ident : str
            識別子
        sbv2file_bytes : bytes
            SBV2ファイルのバイナリデータ
        """
        ...

    def load_aivmx(self, ident: str, aivmx_bytes: bytes) -> None:
        """AIVMXファイルを読み込む

        Parameters
        ----------
        ident : str
            識別子
        aivmx_bytes : bytes
            AIVMXファイルのバイナリデータ
        """
        ...

    def load_sbv2file_from_path(self, ident: str, sbv2file_path: Union[str, Path]) -> None:
        """パスからSBV2ファイルを読み込む

        Parameters
        ----------
        ident : str
            識別子
        sbv2file_path : str
            SBV2ファイルのパス
        """
        ...

    def load_aivmx_from_path(self, ident: str, aivmx_path: Union[str, Path]) -> None:
        """パスからAIVMXファイルを読み込む

        Parameters
        ----------
        ident : str
            識別子
        aivmx_path : str
            AIVMXファイルのパス
        """
        ...

    def synthesize(
        self,
        text: str,
        ident: str,
        style_id: int,
        speaker_id: int,
        sdp_ratio: float,
        length_scale: float
    ) -> bytes:
        """テキストから音声を合成する

        Parameters
        ----------
        text : str
            テキスト
        ident : str
            識別子
        style_id : int
            スタイルID
        speaker_id : int
            話者ID
        sdp_ratio : float
            SDP比率
        length_scale : float
            音声の長さのスケール

        Returns
        -------
        bytes
            音声データ
        """
        ...

    def unload(self, ident: str) -> bool:
        """モデルをアンロードする

        Parameters
        ----------
        ident : str
            識別子

        Returns
        -------
        bool
            アンロードが成功したかどうか
        """
        ...