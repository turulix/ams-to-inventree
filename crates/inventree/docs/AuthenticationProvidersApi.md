# \AuthenticationProvidersApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_provider_redirect_post**](AuthenticationProvidersApi.md#allauth_auth_provider_redirect_post) | **POST** /api/auth/v1/auth/provider/redirect | Provider redirect
[**allauth_auth_provider_signup_get**](AuthenticationProvidersApi.md#allauth_auth_provider_signup_get) | **GET** /api/auth/v1/auth/provider/signup | Provider signup information
[**allauth_auth_provider_signup_post**](AuthenticationProvidersApi.md#allauth_auth_provider_signup_post) | **POST** /api/auth/v1/auth/provider/signup | Provider signup
[**allauth_auth_provider_token_post**](AuthenticationProvidersApi.md#allauth_auth_provider_token_post) | **POST** /api/auth/v1/auth/provider/token | Provider token



## allauth_auth_provider_redirect_post

> allauth_auth_provider_redirect_post(provider, callback_url, process)
Provider redirect

Initiates the third-party provider authentication redirect flow. As calling this endpoint results in a user facing redirect (302), this call is only available in a browser, and must be called in a synchronous (non-XHR) manner. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | The provider ID.  | [required] |
**callback_url** | **String** | The URL to return to after the redirect flow is complete.  Note that this is not to be mistaken with the callback URL that you configure over at the OAuth provider during the OAuth app/client setup. The flow is as follows:    1. Your frontend redirects to the headless provider redirect      endpoint in a synchronous (non-XHR) manner, informing allauth      (by means of `callback_url`) where to redirect to after the      provider handshake is completed.    2. Headless will redirect to the (OAuth) identity provider to      initiate the handshake, passing along a different callback URL      to the provider: one that points to an allauth backend URL.      This is the URL that you need to have setup at your OAuth      app/client configuration. Note that this must be a backend URL      as providers can use POST requests to perform their callbacks,      which is something a frontend would not be able to handle.    3. After the authorization at the provider is completed, the      provider redirects to the *backend* allauth callback URL, which      will then redirect back to the *frontend* callback URL.    4. Your frontend is now expected to fetch the current session to      determine what the next course of action is. The user could be      authenticated at this point, or another flow is pending      (e.g. email verification, or, provider signup). In case of      errors a `?error=` is passed to the frontend callback URL.  | [required] |
**process** | [**models::AllauthProcess**](AllauthProcess.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_provider_signup_get

> models::AllauthAuthProviderSignupGet200Response allauth_auth_provider_signup_get()
Provider signup information

If, while signing up using a third-party provider account, there is insufficient information received from the provider to automatically complete the signup process, an additional step is needed to complete the missing data before the user is fully signed up and authenticated. The information available so far, such as the pending provider account, can be retrieved via this endpoint. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthProviderSignupGet200Response**](allauth_auth_provider_signup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_provider_signup_post

> models::AllauthAuthenticatedResponse allauth_auth_provider_signup_post(allauth_provider_signup)
Provider signup

If, while signing up using a third-party provider account, there is insufficient information received from the provider to automatically complete the signup process, an additional step is needed to complete the missing data before the user is fully signed up and authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_provider_signup** | [**AllauthProviderSignup**](AllauthProviderSignup.md) | Provider signup. | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_provider_token_post

> models::AllauthAuthenticatedResponse allauth_auth_provider_token_post(allauth_provider_token)
Provider token

Authenticates with a third-party provider using provider tokens received by other means. For example, in case of a mobile app, the authentication flow runs completely on the device itself, without any interaction with the API. Then, when the (device) authentication completes and the mobile app receives an access and/or ID token, it can hand over these tokens via this endpoint to authenticate on the server. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_provider_token** | [**AllauthProviderToken**](AllauthProviderToken.md) |  | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

