# \AuthenticationWebAuthnLoginApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_webauthn_authenticate_get**](AuthenticationWebAuthnLoginApi.md#allauth_auth_webauthn_authenticate_get) | **GET** /api/auth/v1/auth/webauthn/authenticate | Get WebAuthn credential request options for 2FA
[**allauth_auth_webauthn_authenticate_post**](AuthenticationWebAuthnLoginApi.md#allauth_auth_webauthn_authenticate_post) | **POST** /api/auth/v1/auth/webauthn/authenticate | Perform 2FA using WebAuthn
[**allauth_auth_webauthn_login_get**](AuthenticationWebAuthnLoginApi.md#allauth_auth_webauthn_login_get) | **GET** /api/auth/v1/auth/webauthn/login | Get WebAuthn credential request options for login
[**allauth_auth_webauthn_login_post**](AuthenticationWebAuthnLoginApi.md#allauth_auth_webauthn_login_post) | **POST** /api/auth/v1/auth/webauthn/login | Login using WebAuthn
[**allauth_auth_webauthn_reauthenticate_get**](AuthenticationWebAuthnLoginApi.md#allauth_auth_webauthn_reauthenticate_get) | **GET** /api/auth/v1/auth/webauthn/reauthenticate | Get WebAuthn credential request options for reauthentication
[**allauth_auth_webauthn_reauthenticate_post**](AuthenticationWebAuthnLoginApi.md#allauth_auth_webauthn_reauthenticate_post) | **POST** /api/auth/v1/auth/webauthn/reauthenticate | Reauthenticate using WebAuthn



## allauth_auth_webauthn_authenticate_get

> models::AllauthAuthWebauthnAuthenticateGet200Response allauth_auth_webauthn_authenticate_get()
Get WebAuthn credential request options for 2FA

Returns the WebAuthn credential request options, that can be processed using `parseRequestOptionsFromJSON()` on the frontend. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthWebauthnAuthenticateGet200Response**](allauth_auth_webauthn_authenticate_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_webauthn_authenticate_post

> models::AllauthAuthenticatedResponse allauth_auth_webauthn_authenticate_post(allauth_auth_webauthn_authenticate_post_request)
Perform 2FA using WebAuthn

Perform Two-Factor Authentication using a WebAuthn credential. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_auth_webauthn_authenticate_post_request** | [**AllauthAuthWebauthnAuthenticatePostRequest**](AllauthAuthWebauthnAuthenticatePostRequest.md) | Authenticate using WebAuthn. | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_webauthn_login_get

> models::AllauthAuthWebauthnAuthenticateGet200Response allauth_auth_webauthn_login_get()
Get WebAuthn credential request options for login

Returns the WebAuthn credential request options, that can be processed using `parseRequestOptionsFromJSON()` on the frontend. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthWebauthnAuthenticateGet200Response**](allauth_auth_webauthn_authenticate_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_webauthn_login_post

> models::AllauthAuthenticatedResponse allauth_auth_webauthn_login_post(allauth_auth_webauthn_authenticate_post_request)
Login using WebAuthn

Login using a WebAuthn credential (Passkey). Both 200 and 401 can be expected after a successful request.  The 401 can, for example, occur when the credential passed was valid, but the email attached to the account still requires verification. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_auth_webauthn_authenticate_post_request** | [**AllauthAuthWebauthnAuthenticatePostRequest**](AllauthAuthWebauthnAuthenticatePostRequest.md) | Login using WebAuthn. | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_webauthn_reauthenticate_get

> models::AllauthAuthWebauthnAuthenticateGet200Response allauth_auth_webauthn_reauthenticate_get()
Get WebAuthn credential request options for reauthentication

Returns the WebAuthn credential request options, that can be processed using `parseRequestOptionsFromJSON()` on the frontend. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthWebauthnAuthenticateGet200Response**](allauth_auth_webauthn_authenticate_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_webauthn_reauthenticate_post

> models::AllauthAuthenticatedResponse allauth_auth_webauthn_reauthenticate_post(allauth_auth_webauthn_authenticate_post_request)
Reauthenticate using WebAuthn

Reauthenticate the user using a WebAuthn credential. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_auth_webauthn_authenticate_post_request** | [**AllauthAuthWebauthnAuthenticatePostRequest**](AllauthAuthWebauthnAuthenticatePostRequest.md) | Reauthenticate using WebAuthn. | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

