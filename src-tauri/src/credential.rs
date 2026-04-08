use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::Win32::Foundation::ERROR_NOT_FOUND;
use windows::Win32::Security::Credentials::{
    CredDeleteW, CredReadW, CredWriteW, CREDENTIALW, CRED_FLAGS, CRED_PERSIST_LOCAL_MACHINE,
    CRED_TYPE_GENERIC,
};

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

pub fn credential_target(host: &str) -> String {
    format!("git:https://{}", host)
}

pub fn write_credential(host: &str, username: &str, token: &str) -> Result<(), String> {
    let target = credential_target(host);
    let target_wide = to_wide(&target);
    let username_wide = to_wide(username);
    let token_bytes = token.as_bytes();

    let mut cred = CREDENTIALW {
        Flags: CRED_FLAGS(0),
        Type: CRED_TYPE_GENERIC,
        TargetName: windows::core::PWSTR(target_wide.as_ptr() as *mut u16),
        Comment: windows::core::PWSTR::null(),
        LastWritten: Default::default(),
        CredentialBlobSize: token_bytes.len() as u32,
        CredentialBlob: token_bytes.as_ptr() as *mut u8,
        Persist: CRED_PERSIST_LOCAL_MACHINE,
        AttributeCount: 0,
        Attributes: std::ptr::null_mut(),
        TargetAlias: windows::core::PWSTR::null(),
        UserName: windows::core::PWSTR(username_wide.as_ptr() as *mut u16),
    };

    unsafe {
        CredWriteW(&mut cred, 0).map_err(|e| format!("CredWrite failed: {}", e))
    }
}

pub fn read_credential(host: &str) -> Result<Option<(String, String)>, String> {
    let target = credential_target(host);
    let target_wide = to_wide(&target);

    unsafe {
        let mut pcredential: *mut CREDENTIALW = std::ptr::null_mut();
        let result = CredReadW(
            windows::core::PCWSTR(target_wide.as_ptr()),
            CRED_TYPE_GENERIC,
            None,
            &mut pcredential,
        );

        match result {
            Ok(()) => {
                let cred = &*pcredential;
                let username = pwstr_to_string(cred.UserName);
                let token = if cred.CredentialBlobSize > 0 && !cred.CredentialBlob.is_null() {
                    let slice = std::slice::from_raw_parts(
                        cred.CredentialBlob,
                        cred.CredentialBlobSize as usize,
                    );
                    String::from_utf8_lossy(slice).to_string()
                } else {
                    String::new()
                };
                windows::Win32::Security::Credentials::CredFree(pcredential as *mut _);
                Ok(Some((username, token)))
            }
            Err(e) if e.code() == ERROR_NOT_FOUND.to_hresult() => Ok(None),
            Err(e) => Err(format!("CredRead failed: {}", e)),
        }
    }
}

pub fn delete_credential(host: &str) -> Result<bool, String> {
    let target = credential_target(host);
    let target_wide = to_wide(&target);

    unsafe {
        let result = CredDeleteW(
            windows::core::PCWSTR(target_wide.as_ptr()),
            CRED_TYPE_GENERIC,
            None,
        );

        match result {
            Ok(()) => Ok(true),
            Err(e) if e.code() == ERROR_NOT_FOUND.to_hresult() => Ok(false),
            Err(e) => Err(format!("CredDelete failed: {}", e)),
        }
    }
}

unsafe fn pwstr_to_string(pwstr: windows::core::PWSTR) -> String {
    if pwstr.is_null() {
        return String::new();
    }
    let mut len = 0;
    while *pwstr.0.add(len) != 0 {
        len += 1;
    }
    let slice = std::slice::from_raw_parts(pwstr.0, len);
    String::from_utf16_lossy(slice)
}
