# \TokensApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_tokens_refresh_get**](TokensApi.md#allauth_tokens_refresh_get) | **GET** /api/auth/v1/tokens/refresh | Refresh the access token 



## allauth_tokens_refresh_get

> models::AllauthTokensRefreshGet200Response allauth_tokens_refresh_get(allauth_tokens_refresh_get_request)
Refresh the access token 

Used to retrieve a new access token. Depending on `settings.HEADLESS_JWT_ROTATE_REFRESH_TOKEN`, a new refresh token is returned as well. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_tokens_refresh_get_request** | Option<[**AllauthTokensRefreshGetRequest**](AllauthTokensRefreshGetRequest.md)> |  |  |

### Return type

[**models::AllauthTokensRefreshGet200Response**](allauth_tokens_refresh_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

