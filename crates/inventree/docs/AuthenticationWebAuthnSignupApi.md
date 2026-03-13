# \AuthenticationWebAuthnSignupApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_webauthn_signup_get**](AuthenticationWebAuthnSignupApi.md#allauth_auth_webauthn_signup_get) | **GET** /api/auth/v1/auth/webauthn/signup | Get passkey credential request options
[**allauth_auth_webauthn_signup_post**](AuthenticationWebAuthnSignupApi.md#allauth_auth_webauthn_signup_post) | **POST** /api/auth/v1/auth/webauthn/signup | Initiate the passkey signup flow
[**allauth_auth_webauthn_signup_put**](AuthenticationWebAuthnSignupApi.md#allauth_auth_webauthn_signup_put) | **PUT** /api/auth/v1/auth/webauthn/signup | Complete the passkey signup flow



## allauth_auth_webauthn_signup_get

> models::AllauthAuthWebauthnAuthenticateGet200Response allauth_auth_webauthn_signup_get()
Get passkey credential request options

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


## allauth_auth_webauthn_signup_post

> allauth_auth_webauthn_signup_post(allauth_passkey_signup)
Initiate the passkey signup flow

You initiate the passkey signup flow by inputting (`POST`) the required properties (e.g. email) similar to the regular account signup, except that the `password` is to be left out. The user will then be required to verify the email address, after which WebAuthn credential creation options can be retrieved (`GET`) and used to actually complete (`PUT`) the flow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_passkey_signup** | [**AllauthPasskeySignup**](AllauthPasskeySignup.md) | Signup using a passkey | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_webauthn_signup_put

> models::AllauthAuthenticatedResponse allauth_auth_webauthn_signup_put(allauth_auth_webauthn_signup_put_request)
Complete the passkey signup flow

Complete the passkey signup flow by handing over the WebAuthn credential. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_auth_webauthn_signup_put_request** | Option<[**AllauthAuthWebauthnSignupPutRequest**](AllauthAuthWebauthnSignupPutRequest.md)> |  |  |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

