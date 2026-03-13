# AllauthSensitiveRecoveryCodesAuthenticator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_used_at** | **f64** | An epoch based timestamp (trivial to parse using: `new Date(value)*1000`)  | 
**created_at** | **f64** | An epoch based timestamp (trivial to parse using: `new Date(value)*1000`)  | 
**r#type** | [**models::AllauthRecoveryCodesAuthenticatorTypeEnum**](AllauthRecoveryCodesAuthenticatorTypeEnum.md) | The authenticator type.  | 
**total_code_count** | **i32** | The total number of recovery codes that initially were available.  | 
**unused_code_count** | **i32** | The number of recovery codes that are unused.  | 
**unused_codes** | **Vec<String>** | The list of unused codes.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


