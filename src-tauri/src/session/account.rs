use serde::Serialize;
use std::path::PathBuf;

/// The globally logged-in Claude account, read from `~/.claude.json`.
///
/// Login in Claude Code is machine-global (`/login` switches the whole
/// install), so this is not a per-session attribute — it is surfaced once for
/// the whole app so you can tell at a glance which account your sessions are
/// billing against.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub email: String,
    pub organization: Option<String>,
    /// e.g. "claude_team", "claude_max" — lets the UI distinguish a shared
    /// team account from a personal one.
    pub account_type: Option<String>,
}

/// Read the logged-in account from `~/.claude.json`. Returns `None` when the
/// file is missing/unreadable or carries no `oauthAccount` (e.g. logged out).
pub fn read_account() -> Option<Account> {
    let path = claude_json_path()?;
    let raw = std::fs::read_to_string(path).ok()?;
    parse_account(&raw)
}

/// Split out from `read_account` so it can be unit-tested without a real
/// `~/.claude.json` on disk.
fn parse_account(raw: &str) -> Option<Account> {
    let value: serde_json::Value = serde_json::from_str(raw).ok()?;
    let acct = value.get("oauthAccount")?;
    let email = acct.get("emailAddress")?.as_str()?.to_string();
    Some(Account {
        email,
        organization: non_empty_str(acct.get("organizationName")),
        account_type: non_empty_str(acct.get("organizationType")),
    })
}

fn non_empty_str(v: Option<&serde_json::Value>) -> Option<String> {
    v.and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

fn claude_json_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".claude.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_account() {
        let json = r#"{"oauthAccount":{"emailAddress":"jeff@getrecast.com","organizationName":"Recast","organizationType":"claude_team"}}"#;
        let a = parse_account(json).unwrap();
        assert_eq!(a.email, "jeff@getrecast.com");
        assert_eq!(a.organization.as_deref(), Some("Recast"));
        assert_eq!(a.account_type.as_deref(), Some("claude_team"));
    }

    #[test]
    fn empty_org_fields_become_none() {
        let json = r#"{"oauthAccount":{"emailAddress":"x@y.com","organizationName":"","organizationType":""}}"#;
        let a = parse_account(json).unwrap();
        assert_eq!(a.email, "x@y.com");
        assert!(a.organization.is_none());
        assert!(a.account_type.is_none());
    }

    #[test]
    fn missing_email_yields_none() {
        let json = r#"{"oauthAccount":{"organizationName":"Recast"}}"#;
        assert!(parse_account(json).is_none());
    }

    #[test]
    fn no_oauth_account_yields_none() {
        assert!(parse_account(r#"{"other":true}"#).is_none());
    }

    #[test]
    fn malformed_json_yields_none() {
        assert!(parse_account("not json").is_none());
    }
}
