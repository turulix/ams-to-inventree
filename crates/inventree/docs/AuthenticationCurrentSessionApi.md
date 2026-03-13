# \AuthenticationCurrentSessionApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_session_delete**](AuthenticationCurrentSessionApi.md#allauth_auth_session_delete) | **DELETE** /api/auth/v1/auth/session | Logout
[**allauth_auth_session_get**](AuthenticationCurrentSessionApi.md#allauth_auth_session_get) | **GET** /api/auth/v1/auth/session | Get authentication status 



## allauth_auth_session_delete

> allauth_auth_session_delete()
Logout

Logs out the user from the current session. 

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


## allauth_auth_session_get

> models::AllauthAuthenticatedResponse allauth_auth_session_get()
Get authentication status 

Retrieve information about the authentication status for the current session. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

