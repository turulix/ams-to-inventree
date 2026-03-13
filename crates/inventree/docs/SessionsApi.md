# \SessionsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_sessions_delete**](SessionsApi.md#allauth_auth_sessions_delete) | **DELETE** /api/auth/v1/auth/sessions | End one or more sessions
[**allauth_auth_sessions_get**](SessionsApi.md#allauth_auth_sessions_get) | **GET** /api/auth/v1/auth/sessions | List sessions



## allauth_auth_sessions_delete

> models::AllauthAuthSessionsGet200Response allauth_auth_sessions_delete(allauth_end_sessions)
End one or more sessions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_end_sessions** | [**AllauthEndSessions**](AllauthEndSessions.md) |  | [required] |

### Return type

[**models::AllauthAuthSessionsGet200Response**](allauth_auth_sessions_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_sessions_get

> models::AllauthAuthSessionsGet200Response allauth_auth_sessions_get()
List sessions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthSessionsGet200Response**](allauth_auth_sessions_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

