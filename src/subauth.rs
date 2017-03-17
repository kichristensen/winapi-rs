// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Types and macros for Subauthentication Packages.
STRUCT!{struct UNICODE_STRING {
    Length: ::USHORT,
    MaximumLength: ::USHORT,
    Buffer: ::PWSTR,
}}
pub type PUNICODE_STRING = *mut UNICODE_STRING;
STRUCT!{struct STRING {
    Length: ::USHORT,
    MaximumLength: ::USHORT,
    Buffer: ::PCHAR,
}}
pub type PSTRING = *mut STRING;
STRUCT!{struct OLD_LARGE_INTEGER {
    LowPart: ::ULONG,
    HighPart: ::LONG,
}}
pub type POLD_LARGE_INTEGER = *mut OLD_LARGE_INTEGER;
pub type SAM_HANDLE = ::PVOID;
pub type PSAM_HANDLE = *mut ::PVOID;
pub const USER_ACCOUNT_DISABLED: ::ULONG = 0x00000001;
pub const USER_HOME_DIRECTORY_REQUIRED: ::ULONG = 0x00000002;
pub const USER_PASSWORD_NOT_REQUIRED: ::ULONG = 0x00000004;
pub const USER_TEMP_DUPLICATE_ACCOUNT: ::ULONG = 0x00000008;
pub const USER_NORMAL_ACCOUNT: ::ULONG = 0x00000010;
pub const USER_MNS_LOGON_ACCOUNT: ::ULONG = 0x00000020;
pub const USER_INTERDOMAIN_TRUST_ACCOUNT: ::ULONG = 0x00000040;
pub const USER_WORKSTATION_TRUST_ACCOUNT: ::ULONG = 0x00000080;
pub const USER_SERVER_TRUST_ACCOUNT: ::ULONG = 0x00000100;
pub const USER_DONT_EXPIRE_PASSWORD: ::ULONG = 0x00000200;
pub const USER_ACCOUNT_AUTO_LOCKED: ::ULONG = 0x00000400;
pub const USER_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ::ULONG = 0x00000800;
pub const USER_SMARTCARD_REQUIRED: ::ULONG = 0x00001000;
pub const USER_TRUSTED_FOR_DELEGATION: ::ULONG = 0x00002000;
pub const USER_NOT_DELEGATED: ::ULONG = 0x00004000;
pub const USER_USE_DES_KEY_ONLY: ::ULONG = 0x00008000;
pub const USER_DONT_REQUIRE_PREAUTH: ::ULONG = 0x00010000;
pub const USER_PASSWORD_EXPIRED: ::ULONG = 0x00020000;
pub const USER_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ::ULONG = 0x00040000;
pub const USER_NO_AUTH_DATA_REQUIRED: ::ULONG = 0x00080000;
pub const USER_PARTIAL_SECRETS_ACCOUNT: ::ULONG = 0x00100000;
pub const USER_USE_AES_KEYS: ::ULONG = 0x00200000;
pub const NEXT_FREE_ACCOUNT_CONTROL_BIT: ::ULONG = USER_USE_AES_KEYS << 1;
pub const USER_MACHINE_ACCOUNT_MASK: ::ULONG = USER_INTERDOMAIN_TRUST_ACCOUNT
    | USER_WORKSTATION_TRUST_ACCOUNT | USER_SERVER_TRUST_ACCOUNT;
pub const USER_ACCOUNT_TYPE_MASK: ::ULONG = USER_TEMP_DUPLICATE_ACCOUNT | USER_NORMAL_ACCOUNT
    | USER_MACHINE_ACCOUNT_MASK;
pub const USER_COMPUTED_ACCOUNT_CONTROL_BITS: ::ULONG = USER_ACCOUNT_AUTO_LOCKED
| USER_PASSWORD_EXPIRED;
pub const SAM_DAYS_PER_WEEK: ::USHORT = 7;
pub const SAM_HOURS_PER_WEEK: ::USHORT = 24 * SAM_DAYS_PER_WEEK;
pub const SAM_MINUTES_PER_WEEK: ::USHORT = 60 * SAM_HOURS_PER_WEEK;
STRUCT!{struct LOGON_HOURS {
    UnitsPerWeek: ::USHORT,
    LogonHours: ::PUCHAR,
}}
pub type PLOGON_HOURS = *mut LOGON_HOURS;
STRUCT!{struct SR_SECURITY_DESCRIPTOR {
    Length: ::ULONG,
    SecurityDescriptor: ::PUCHAR,
}}
pub type PSR_SECURITY_DESCRIPTOR = *mut SR_SECURITY_DESCRIPTOR;
STRUCT!{struct USER_ALL_INFORMATION {
    LastLogon: ::LARGE_INTEGER,
    LastLogoff: ::LARGE_INTEGER,
    PasswordLastSet: ::LARGE_INTEGER,
    AccountExpires: ::LARGE_INTEGER,
    PasswordCanChange: ::LARGE_INTEGER,
    PasswordMustChange: ::LARGE_INTEGER,
    UserName: UNICODE_STRING,
    FullName: UNICODE_STRING,
    HomeDirectory: UNICODE_STRING,
    HomeDirectoryDrive: UNICODE_STRING,
    ScriptPath: UNICODE_STRING,
    ProfilePath: UNICODE_STRING,
    AdminComment: UNICODE_STRING,
    WorkStations: UNICODE_STRING,
    UserComment: UNICODE_STRING,
    Parameters: UNICODE_STRING,
    LmPassword: UNICODE_STRING,
    NtPassword: UNICODE_STRING,
    PrivateData: UNICODE_STRING,
    SecurityDescriptor: SR_SECURITY_DESCRIPTOR,
    UserId: ::ULONG,
    PrimaryGroupId: ::ULONG,
    UserAccountControl: ::ULONG,
    WhichFields: ::ULONG,
    LogonHours: LOGON_HOURS,
    BadPasswordCount: ::USHORT,
    LogonCount: ::USHORT,
    CountryCode: ::USHORT,
    CodePage: ::USHORT,
    LmPasswordPresent: ::BOOLEAN,
    NtPasswordPresent: ::BOOLEAN,
    PasswordExpired: ::BOOLEAN,
    PrivateDataSensitive: ::BOOLEAN,
}}
pub type PUSER_ALL_INFORMATION = *mut USER_ALL_INFORMATION;
pub const USER_ALL_PARAMETERS: ::ULONG = 0x00200000;
pub const CLEAR_BLOCK_LENGTH: usize = 8;
STRUCT!{struct CLEAR_BLOCK {
    data: [::CHAR; CLEAR_BLOCK_LENGTH],
}}
pub type PCLEAR_BLOCK = *mut CLEAR_BLOCK;
pub const CYPHER_BLOCK_LENGTH: usize = 8;
STRUCT!{struct CYPHER_BLOCK {
    data: [::CHAR; CYPHER_BLOCK_LENGTH],
}}
pub type PCYPHER_BLOCK = *mut CYPHER_BLOCK;
STRUCT!{struct LM_OWF_PASSWORD {
    data: [CYPHER_BLOCK; 2],
}}
pub type PLM_OWF_PASSWORD = *mut LM_OWF_PASSWORD;
pub type LM_CHALLENGE = CLEAR_BLOCK;
pub type PLM_CHALLENGE = *mut LM_CHALLENGE;
pub type NT_OWF_PASSWORD = LM_OWF_PASSWORD;
pub type PNT_OWF_PASSWORD = *mut NT_OWF_PASSWORD;
pub type NT_CHALLENGE = LM_CHALLENGE;
pub type PNT_CHALLENGE = *mut NT_CHALLENGE;
pub const USER_SESSION_KEY_LENGTH: usize = CYPHER_BLOCK_LENGTH * 2;
STRUCT!{struct USER_SESSION_KEY {
    data: [CYPHER_BLOCK; 2],
}}
pub type PUSER_SESSION_KEY = *mut USER_SESSION_KEY;
ENUM!{enum NETLOGON_LOGON_INFO_CLASS {
    NetlogonInteractiveInformation = 1,
    NetlogonNetworkInformation,
    NetlogonServiceInformation,
    NetlogonGenericInformation,
    NetlogonInteractiveTransitiveInformation,
    NetlogonNetworkTransitiveInformation,
    NetlogonServiceTransitiveInformation,
}}
STRUCT!{struct NETLOGON_LOGON_IDENTITY_INFO {
    LogonDomainName: UNICODE_STRING,
    ParameterControl: ::ULONG,
    LogonId: OLD_LARGE_INTEGER,
    UserName: UNICODE_STRING,
    Workstation: UNICODE_STRING,
}}
pub type PNETLOGON_LOGON_IDENTITY_INFO = *mut NETLOGON_LOGON_IDENTITY_INFO;
STRUCT!{struct NETLOGON_INTERACTIVE_INFO {
    Identity: NETLOGON_LOGON_IDENTITY_INFO,
    LmOwfPassword: LM_OWF_PASSWORD,
    NtOwfPassword: NT_OWF_PASSWORD,
}}
pub type PNETLOGON_INTERACTIVE_INFO = *mut NETLOGON_INTERACTIVE_INFO;
STRUCT!{struct NETLOGON_SERVICE_INFO {
    Identity: NETLOGON_LOGON_IDENTITY_INFO,
    LmOwfPassword: LM_OWF_PASSWORD,
    NtOwfPassword: NT_OWF_PASSWORD,
}}
pub type PNETLOGON_SERVICE_INFO = *mut NETLOGON_SERVICE_INFO;
STRUCT!{struct NETLOGON_NETWORK_INFO {
    Identity: NETLOGON_LOGON_IDENTITY_INFO,
    LmChallenge: LM_CHALLENGE,
    NtChallengeResponse: STRING,
    LmChallengeResponse: STRING,
}}
pub type PNETLOGON_NETWORK_INFO = *mut NETLOGON_NETWORK_INFO;
STRUCT!{struct NETLOGON_GENERIC_INFO {
    Identity: NETLOGON_LOGON_IDENTITY_INFO,
    PackageName: UNICODE_STRING,
    DataLength: ::ULONG,
    LogonData: ::PUCHAR,
}}
pub type PNETLOGON_GENERIC_INFO = *mut NETLOGON_GENERIC_INFO;
pub const MSV1_0_PASSTHRU: ::ULONG = 0x01;
pub const MSV1_0_GUEST_LOGON: ::ULONG = 0x02;
STRUCT!{struct MSV1_0_VALIDATION_INFO {
    LogoffTime: ::LARGE_INTEGER,
    KickoffTime: ::LARGE_INTEGER,
    LogonServer: UNICODE_STRING,
    LogonDomainName: UNICODE_STRING,
    SessionKey: USER_SESSION_KEY,
    Authoritative: ::BOOLEAN,
    UserFlags: ::ULONG,
    WhichFields: ::ULONG,
    UserId: ::ULONG,
}}
pub type PMSV1_0_VALIDATION_INFO = *mut MSV1_0_VALIDATION_INFO;
pub const MSV1_0_VALIDATION_LOGOFF_TIME: ::ULONG = 0x00000001;
pub const MSV1_0_VALIDATION_KICKOFF_TIME: ::ULONG = 0x00000002;
pub const MSV1_0_VALIDATION_LOGON_SERVER: ::ULONG = 0x00000004;
pub const MSV1_0_VALIDATION_LOGON_DOMAIN: ::ULONG = 0x00000008;
pub const MSV1_0_VALIDATION_SESSION_KEY: ::ULONG = 0x00000010;
pub const MSV1_0_VALIDATION_USER_FLAGS: ::ULONG = 0x00000020;
pub const MSV1_0_VALIDATION_USER_ID: ::ULONG = 0x00000040;
pub const MSV1_0_SUBAUTH_ACCOUNT_DISABLED: ::ULONG = 0x00000001;
pub const MSV1_0_SUBAUTH_PASSWORD: ::ULONG = 0x00000002;
pub const MSV1_0_SUBAUTH_WORKSTATIONS: ::ULONG = 0x00000004;
pub const MSV1_0_SUBAUTH_LOGON_HOURS: ::ULONG = 0x00000008;
pub const MSV1_0_SUBAUTH_ACCOUNT_EXPIRY: ::ULONG = 0x00000010;
pub const MSV1_0_SUBAUTH_PASSWORD_EXPIRY: ::ULONG = 0x00000020;
pub const MSV1_0_SUBAUTH_ACCOUNT_TYPE: ::ULONG = 0x00000040;
pub const MSV1_0_SUBAUTH_LOCKOUT: ::ULONG = 0x00000080;
