# AllauthWebAuthnAuthenticator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_used_at** | **f64** | An epoch based timestamp (trivial to parse using: `new Date(value)*1000`)  | 
**created_at** | **f64** | An epoch based timestamp (trivial to parse using: `new Date(value)*1000`)  | 
**r#type** | [**models::AllauthWebAuthnAuthenticatorTypeEnum**](AllauthWebAuthnAuthenticatorTypeEnum.md) |  | 
**id** | **i32** | Authenticator ID.  | 
**name** | **String** |  | 
**is_passwordless** | Option<**bool**> | Whether or not this authenticator represents a passkey. Absent if it is not specified.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


