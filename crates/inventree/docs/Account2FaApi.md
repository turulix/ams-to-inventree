# \Account2FaApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_account_authenticators_get**](Account2FaApi.md#allauth_account_authenticators_get) | **GET** /api/auth/v1/account/authenticators | List authenticators
[**allauth_account_authenticators_recovery_codes_get**](Account2FaApi.md#allauth_account_authenticators_recovery_codes_get) | **GET** /api/auth/v1/account/authenticators/recovery-codes | List recovery codes
[**allauth_account_authenticators_recovery_codes_post**](Account2FaApi.md#allauth_account_authenticators_recovery_codes_post) | **POST** /api/auth/v1/account/authenticators/recovery-codes | Regenerate recovery codes
[**allauth_account_authenticators_totp_delete**](Account2FaApi.md#allauth_account_authenticators_totp_delete) | **DELETE** /api/auth/v1/account/authenticators/totp | Deactivate TOTP
[**allauth_account_authenticators_totp_get**](Account2FaApi.md#allauth_account_authenticators_totp_get) | **GET** /api/auth/v1/account/authenticators/totp | TOTP authenticator status
[**allauth_account_authenticators_totp_post**](Account2FaApi.md#allauth_account_authenticators_totp_post) | **POST** /api/auth/v1/account/authenticators/totp | Activate TOTP



## allauth_account_authenticators_get

> models::AllauthAccountAuthenticatorsGet200Response allauth_account_authenticators_get()
List authenticators

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAccountAuthenticatorsGet200Response**](allauth_account_authenticators_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_recovery_codes_get

> models::AllauthAccountAuthenticatorsRecoveryCodesGet200Response allauth_account_authenticators_recovery_codes_get()
List recovery codes

List recovery codes. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAccountAuthenticatorsRecoveryCodesGet200Response**](allauth_account_authenticators_recovery_codes_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_recovery_codes_post

> allauth_account_authenticators_recovery_codes_post()
Regenerate recovery codes

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_totp_delete

> models::AllauthAuthEmailVerifyResendPost200Response allauth_account_authenticators_totp_delete()
Deactivate TOTP

Deactivates TOTP authentication. If the user authentication is not sufficiently recent, a reauthentication flow (`401`) will is presented. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthEmailVerifyResendPost200Response**](allauth_auth_email_verify_resend_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_totp_get

> models::AllauthAccountAuthenticatorsTotpGet200Response allauth_account_authenticators_totp_get()
TOTP authenticator status

Retrieve the information about the current TOTP authenticator, if any. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAccountAuthenticatorsTotpGet200Response**](allauth_account_authenticators_totp_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_totp_post

> models::AllauthAccountAuthenticatorsTotpGet200Response allauth_account_authenticators_totp_post(allauth_account_authenticators_totp_post_request)
Activate TOTP

The code should be provided from the consuming TOTP authenticator application which was generated using the TOTP authenticator secret retrieved from the TOTP authenticator status endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_authenticators_totp_post_request** | Option<[**AllauthAccountAuthenticatorsTotpPostRequest**](AllauthAccountAuthenticatorsTotpPostRequest.md)> |  |  |

### Return type

[**models::AllauthAccountAuthenticatorsTotpGet200Response**](allauth_account_authenticators_totp_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

