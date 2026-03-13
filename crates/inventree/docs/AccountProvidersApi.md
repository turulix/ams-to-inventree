# \AccountProvidersApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_account_providers_delete**](AccountProvidersApi.md#allauth_account_providers_delete) | **DELETE** /api/auth/v1/account/providers | Disconnect a third-party provider account 
[**allauth_account_providers_get**](AccountProvidersApi.md#allauth_account_providers_get) | **GET** /api/auth/v1/account/providers | List the connected third-party provider accounts



## allauth_account_providers_delete

> models::AllauthAccountProvidersGet200Response allauth_account_providers_delete(allauth_account_providers_delete_request)
Disconnect a third-party provider account 

Disconnect a third-party provider account, returning the remaining accounts that are still connected. The disconnect is not allowed if it would leave the account unusable. For example, if no password was set up yet. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_providers_delete_request** | Option<[**AllauthAccountProvidersDeleteRequest**](AllauthAccountProvidersDeleteRequest.md)> |  |  |

### Return type

[**models::AllauthAccountProvidersGet200Response**](allauth_account_providers_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_providers_get

> models::AllauthAccountProvidersGet200Response allauth_account_providers_get()
List the connected third-party provider accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAccountProvidersGet200Response**](allauth_account_providers_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

