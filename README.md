# Android MDM

`cargo run` gives the following response (a list of devices):

```json
Ok(
(
Response {
  status: 200,
  version: HTTP/1.1,
  headers: {
    "content-type": "application/json; charset=UTF-8",
    "vary": "X-Origin",
    "vary": "Referer",
    "vary": "Origin,Accept-Encoding",
    "date": "Tue, 26 Sep 2023 11:02:16 GMT",
    "server": "ESF",
    "cache-control": "private",
    "x-xss-protection": "0",
    "x-frame-options": "SAMEORIGIN",
    "x-content-type-options": "nosniff",
    "alt-svc": "h3=\":443\"; ma=2592000,h3-29=\":443\"; ma=2592000",
    "accept-ranges": "none",
    "transfer-encoding": "chunked"
  },
  body: Body(
  Streaming,
  )
},
ListDevicesResponse {
devices: Some(
[
Device {
api_level: Some(
30,
),
application_reports: None,
applied_password_policies: None,
applied_policy_name: Some(
"enterprises/LC03ulzkxb/policies/policy1",
),
applied_policy_version: Some(
16,
),
applied_state: Some(
"ACTIVE",
),
common_criteria_mode_info: None,
device_settings: None,
disabled_reason: None,
displays: None,
enrollment_time: Some(
2023-09-05T06: 21: 34.137Z,
),
enrollment_token_data: None,
enrollment_token_name: Some(
"enterprises/LC03ulzkxb/enrollmentTokens/yPNkIkFET2J3Hznb_emgu8dX7clhJdfQELBCnan5pbo",
),
hardware_info: Some(
HardwareInfo {
battery_shutdown_temperatures: None,
battery_throttling_temperatures: None,
brand: Some(
"realme",
),
cpu_shutdown_temperatures: None,
cpu_throttling_temperatures: None,
device_baseband_version: Some(
"M_V3_P10,M_V3_P10",
),
enterprise_specific_id: None,
gpu_shutdown_temperatures: None,
gpu_throttling_temperatures: None,
hardware: Some(
"mt6768",
),
manufacturer: Some(
"realme",
),
model: Some(
"RMX2193",
),
serial_number: Some(
"SC4TW8HYJB99V4EQ",
),
skin_shutdown_temperatures: None,
skin_throttling_temperatures: None,
},
),
hardware_status_samples: None,
last_policy_compliance_report_time: None,
last_policy_sync_time: Some(
2023-09-26T07: 02: 48.905Z,
),
last_status_report_time: Some(
2023-09-26T07: 02: 34.773Z,
),
management_mode: Some(
"DEVICE_OWNER",
),
memory_events: None,
memory_info: Some(
MemoryInfo {
total_internal_storage: Some(
2293370880,
),
total_ram: Some(
3959894016,
),
},
),
name: Some(
"enterprises/LC03ulzkxb/devices/3035feba43ae1b42",
),
network_info: None,
non_compliance_details: Some(
[
NonComplianceDetail {
current_value: None,
field_path: None,
installation_failure_reason: Some(
"NOT_FOUND",
),
non_compliance_reason: Some(
"APP_NOT_INSTALLED",
),
package_name: Some(
"com.google.android.gms.maps",
),
setting_name: Some(
"applications",
),
specific_non_compliance_context: None,
specific_non_compliance_reason: None,
},
],
),
ownership: Some(
"COMPANY_OWNED",
),
policy_compliant: Some(
true,
),
policy_name: Some(
"enterprises/LC03ulzkxb/policies/policy1",
),
power_management_events: None,
previous_device_names: Some(
[
"enterprises/LC03ulzkxb/devices/324c42498aebd26c",
],
),
security_posture: Some(
SecurityPosture {
device_posture: Some(
"SECURE",
),
posture_details: None,
},
),
software_info: None,
state: Some(
"ACTIVE",
),
system_properties: None,
user: None,
user_name: Some(
"enterprises/LC03ulzkxb/users/101289971983021954566",
),
},
],
),
next_page_token: None,
},
),
)

```