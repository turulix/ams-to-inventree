# \Authentication2FaApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth2fa_authenticate_post**](Authentication2FaApi.md#allauth_auth2fa_authenticate_post) | **POST** /api/auth/v1/auth/2fa/authenticate | Two-factor authentication
[**allauth_auth2fa_reauthenticate_post**](Authentication2FaApi.md#allauth_auth2fa_reauthenticate_post) | **POST** /api/auth/v1/auth/2fa/reauthenticate | Reauthenticate using 2FA
[**allauth_auth2fa_trust_post**](Authentication2FaApi.md#allauth_auth2fa_trust_post) | **POST** /api/auth/v1/auth/2fa/trust | Trust this browser



## allauth_auth2fa_authenticate_post

> models::AllauthAuthenticatedResponse allauth_auth2fa_authenticate_post(allauth_mfa_authenticate)
Two-factor authentication

If, during authentication,  a response with status 401 is encountered where one of the pending flows has ID `mfa_authenticate`, that indicates that the Two-Factor Authentication stage needs to be completed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_mfa_authenticate** | [**AllauthMfaAuthenticate**](AllauthMfaAuthenticate.md) |  | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth2fa_reauthenticate_post

> models::AllauthAuthenticatedResponse allauth_auth2fa_reauthenticate_post(allauth_mfa_authenticate)
Reauthenticate using 2FA

In order to safeguard the account, some actions require the user to be recently authenticated.  If you try to perform such an action without having been recently authenticated, a `401` status is returned, listing flows that can be performed to reauthenticate. One such flow is the flow with ID `mfa_reauthenticate`, which allows for the user to input an authenticator code (e.g. TOTP or recovery code). This is the endpoint related towards that flow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_mfa_authenticate** | [**AllauthMfaAuthenticate**](AllauthMfaAuthenticate.md) |  | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth2fa_trust_post

> models::AllauthAuthenticatedResponse allauth_auth2fa_trust_post(allauth_mfa_trust)
Trust this browser

If \"Trust this browser?\" is enabled (`MFA_TRUST_ENABLED`), the `mfa_trust` flow activates after the user completes the MFA authentication flow, offering to skip MFA for this particular browser. This endpoint is used to complete the `mfa_trust` flow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_mfa_trust** | [**AllauthMfaTrust**](AllauthMfaTrust.md) |  | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

