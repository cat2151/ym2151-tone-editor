//! 自動アップデート機能。
//! 起動時はバックグラウンドで更新有無を確認し、
//! 明示的な `check` / `update` サブコマンドや終了後更新には
//! cat-self-update-lib を利用する。

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::Result;
use cat_self_update_lib::{check_remote_commit, self_update, CheckResult};

const REPO_OWNER: &str = "cat2151";
const REPO_NAME: &str = "ym2151-tone-editor";
const MAIN_BRANCH: &str = "main";

/// ビルド時に埋め込まれたgit commit hash
const LOCAL_HASH: &str = env!("GIT_COMMIT_HASH");

/// ローカルhashが有効なSHA-1の40文字16進数文字列かを確認する
fn is_valid_sha1(s: &str) -> bool {
    s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn validate_local_hash(hash: &str) -> Result<&str> {
    let trimmed = hash.trim();
    if !is_valid_sha1(trimmed) {
        anyhow::bail!("埋め込みコミットハッシュが無効です: {trimmed}");
    }
    Ok(trimmed)
}

fn should_skip_background_check(local_hash: &str, is_debug_build: bool) -> bool {
    is_debug_build || !is_valid_sha1(local_hash.trim())
}

/// バックグラウンドでアップデートチェックを実行する。
/// 更新が必要な場合は `update_available` を true にセットする。
pub fn spawn_update_check(update_available: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        if should_skip_background_check(LOCAL_HASH, cfg!(debug_assertions)) {
            return;
        }

        if let Ok(result) = check_for_update() {
            if !result.is_up_to_date() {
                update_available.store(true, Ordering::Relaxed);
            }
        }
    });
}

/// 現在のビルド埋め込みコミットとリモート main を比較する。
pub fn check_for_update() -> Result<CheckResult> {
    let local_hash = validate_local_hash(LOCAL_HASH)?;
    check_remote_commit(REPO_OWNER, REPO_NAME, MAIN_BRANCH, local_hash)
        .map_err(|e| anyhow::anyhow!("アップデートチェックに失敗しました: {}", e))
}

/// フォアグラウンドでアップデートを実行する。
pub fn run_foreground_update() -> Result<()> {
    println!("アップデートを開始します...");
    self_update(REPO_OWNER, REPO_NAME, &[])
        .map_err(|e| anyhow::anyhow!("アップデートに失敗しました: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sha1_with_valid_hash() {
        assert!(is_valid_sha1("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3"));
    }

    #[test]
    fn test_is_valid_sha1_with_all_zeros() {
        assert!(is_valid_sha1("0000000000000000000000000000000000000000"));
    }

    #[test]
    fn test_is_valid_sha1_too_short() {
        assert!(!is_valid_sha1("abc123"));
    }

    #[test]
    fn test_is_valid_sha1_too_long() {
        assert!(!is_valid_sha1("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3ff"));
    }

    #[test]
    fn test_is_valid_sha1_non_hex_char() {
        // 'z' はhex文字でない
        assert!(!is_valid_sha1("z94a8fe5ccb19ba61c4c0873d391e987982fbbd3"));
    }

    #[test]
    fn test_is_valid_sha1_with_unknown_string() {
        assert!(!is_valid_sha1("unknown"));
    }

    #[test]
    fn test_is_valid_sha1_empty() {
        assert!(!is_valid_sha1(""));
    }

    /// デバッグビルド（テスト実行時）では check_for_update は早期リターンし、
    /// update_available フラグは変化しないことを確認する。
    #[test]
    fn test_validate_local_hash_accepts_trimmed_valid_hash() {
        assert_eq!(
            validate_local_hash("  a94a8fe5ccb19ba61c4c0873d391e987982fbbd3  ").unwrap(),
            "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3"
        );
    }

    #[test]
    fn test_validate_local_hash_rejects_unknown_string() {
        assert!(validate_local_hash("unknown").is_err());
    }

    #[test]
    fn test_should_skip_background_check_in_debug_build() {
        assert!(should_skip_background_check(
            "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3",
            true
        ));
    }

    #[test]
    fn test_should_skip_background_check_with_invalid_hash() {
        assert!(should_skip_background_check("unknown", false));
    }

    #[test]
    fn test_should_not_skip_background_check_with_valid_release_hash() {
        assert!(!should_skip_background_check(
            "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3",
            false
        ));
    }
}
