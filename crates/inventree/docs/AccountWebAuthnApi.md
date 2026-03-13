# \AccountWebAuthnApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_account_authenticators_webauthn_delete**](AccountWebAuthnApi.md#allauth_account_authenticators_webauthn_delete) | **DELETE** /api/auth/v1/account/authenticators/webauthn | Delete a WebAuthn credential 
[**allauth_account_authenticators_webauthn_get**](AccountWebAuthnApi.md#allauth_account_authenticators_webauthn_get) | **GET** /api/auth/v1/account/authenticators/webauthn | Get WebAuthn credential creation options 
[**allauth_account_authenticators_webauthn_post**](AccountWebAuthnApi.md#allauth_account_authenticators_webauthn_post) | **POST** /api/auth/v1/account/authenticators/webauthn | Add a WebAuthn credential 
[**allauth_account_authenticators_webauthn_put**](AccountWebAuthnApi.md#allauth_account_authenticators_webauthn_put) | **PUT** /api/auth/v1/account/authenticators/webauthn | Rename a WebAuthn credential 



## allauth_account_authenticators_webauthn_delete

> models::AllauthAuthEmailVerifyResendPost200Response allauth_account_authenticators_webauthn_delete(allauth_account_authenticators_webauthn_delete_request)
Delete a WebAuthn credential 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_authenticators_webauthn_delete_request** | Option<[**AllauthAccountAuthenticatorsWebauthnDeleteRequest**](AllauthAccountAuthenticatorsWebauthnDeleteRequest.md)> |  |  |

### Return type

[**models::AllauthAuthEmailVerifyResendPost200Response**](allauth_auth_email_verify_resend_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_webauthn_get

> models::AllauthAccountAuthenticatorsWebauthnGet200Response allauth_account_authenticators_webauthn_get(passwordless)
Get WebAuthn credential creation options 

Returns the WebAuthn credential creation options, that can be processed using `parseCreationOptionsFromJSON()` on the frontend. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**passwordless** | Option<**bool**> | When present (regardless of its value), enables passwordless sign-in via a WebAuthn credential (Passkey), but may enforce additional multi-factor authentication (MFA) requirements. Omit the parameter to disable.  |  |

### Return type

[**models::AllauthAccountAuthenticatorsWebauthnGet200Response**](allauth_account_authenticators_webauthn_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_webauthn_post

> models::AllauthAccountAuthenticatorsWebauthnPost200Response allauth_account_authenticators_webauthn_post(allauth_auth_webauthn_signup_put_request)
Add a WebAuthn credential 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_auth_webauthn_signup_put_request** | Option<[**AllauthAuthWebauthnSignupPutRequest**](AllauthAuthWebauthnSignupPutRequest.md)> |  |  |

### Return type

[**models::AllauthAccountAuthenticatorsWebauthnPost200Response**](allauth_account_authenticators_webauthn_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_authenticators_webauthn_put

> models::AllauthAccountAuthenticatorsWebauthnPut200Response allauth_account_authenticators_webauthn_put(allauth_account_authenticators_webauthn_put_request)
Rename a WebAuthn credential 

You can alter the name of a WebAuthn credential by PUT'ting the ID and name of the authenticator representing that credential. You can obtain the credentials via the \"List authenticators\" endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_authenticators_webauthn_put_request** | Option<[**AllauthAccountAuthenticatorsWebauthnPutRequest**](AllauthAccountAuthenticatorsWebauthnPutRequest.md)> |  |  |

### Return type

[**models::AllauthAccountAuthenticatorsWebauthnPut200Response**](allauth_account_authenticators_webauthn_put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

