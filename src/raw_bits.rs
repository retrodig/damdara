use crate::load::parse_bitstring_to_save_data;
use crate::save::SaveData;
use crate::save::build_password_base;

/// 120bit（15バイト）のビット列を表す中間構造体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawBits120(pub [u8; 15]);

impl RawBits120 {
    /// 先頭8bit（チェックサム）
    pub fn checksum(&self) -> u8 {
        self.0[0]
    }

    /// チェックサムを除いた実データ部分（112bit）
    pub fn payload(&self) -> &[u8] {
        &self.0[1..]
    }

    /// SaveData → RawBits120 に変換（暗号化済みビット列を構築）
    pub fn from_save_data(data: &SaveData) -> Result<Self, String> {
        let bit_array = build_password_base(data)?; // Vec<String> (8bit×15)
        if bit_array.len() != 15 {
            return Err("build_password_base returned unexpected size".to_string());
        }

        let mut bytes = [0u8; 15];
        for (i, bstr) in bit_array.iter().enumerate() {
            bytes[i] =
                u8::from_str_radix(bstr, 2).map_err(|e| format!("bit parse error: {}", e))?;
        }

        Ok(Self(bytes))
    }

    /// RawBits120 → SaveData に復元（load側の処理を統合）
    pub fn to_save_data(&self) -> Result<SaveData, String> {
        parse_bitstring_to_save_data(&self.0)
    }

    /// チェックサム（CRC）を検証するメソッド（必要なら追加）
    pub fn verify_crc(&self) -> bool {
        // TODO: 既存の CRC 計算と比較する処理を入れても良い
        true
    }
}
